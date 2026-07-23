//! `emit` — the drift engine.
//!
//! Drift detection (a direct edit to emitted
//! output is drift routed to the authored source, never merged back).
//!
//! [`emit_program`] runs the SDK program (`node <workspace>/harness.ts`) and hands its
//! JSON payload to [`emit`], the sole compiler of every projection and the whole lock —
//! no harness re-supply, the payload IS the source. Each projection is re-emitted
//! **whole** and byte-deterministically — verified by a double-emit comparison, so
//! nondeterministic authoring is a loud failure, never a silent churn. A hand-edited
//! projection is overwritten: it is drift routed to the source, surfaced by
//! `config.stale`/the guard, not a merge. [`place`] is the whole-file placement merge
//! for artifacts temper *places* rather than emits; it keeps its own three-state conflict detection until `install` rides
//! emit's projection.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;
use serde_json::Value as JsonValue;
use toml_edit::{
    Array, ArrayOfTables, DocumentMut, InlineTable, Item, Table, TableLike, Value, value,
};

use crate::compose;
use crate::contract;
use crate::extract::host_address;
use crate::graph;
use crate::hash::{canonicalize_eol, sha256_hex};
use crate::kind::{
    CollectionAddress, Commitment, Content, Format, collection_address_from_row,
    commitment_from_row, content_from_row, format_from_row,
};
use crate::layout::{Layout, LayoutRegion};
use std::cell::Cell;

thread_local! {
    /// Per-thread count of lock.toml file reads. Incremented each time the lock file
    /// is actually read from disk, pinning that whole-input work hoists lock parsing
    /// (one per run) rather than repeating it per call site.
    static LOCK_READS: Cell<usize> = const { Cell::new(0) };
    /// Per-thread count of lock.toml parse operations. Incremented each time the lock
    /// text is parsed into a TOML document, pinning that parsing is shared across
    /// emit's multiple phases rather than recomputed per phase.
    static LOCK_PARSES: Cell<usize> = const { Cell::new(0) };
    /// Per-thread count of represented manifest file reads. Incremented each time a
    /// manifest file is read from disk for drift detection, pinning that the read is
    /// done once per manifest per emit() run and shared with the write-decision phase.
    static MANIFEST_READS: Cell<usize> = const { Cell::new(0) };
}

/// This thread's cumulative count of lock.toml file reads. Read before and after an
/// emit run to pin that the lock is read exactly once per run.
#[must_use]
pub fn lock_read_count() -> usize {
    LOCK_READS.with(Cell::get)
}

/// This thread's cumulative count of lock.toml parse operations. Read before and after
/// an emit run to pin that parsing is done exactly once per run.
#[must_use]
pub fn lock_parse_count() -> usize {
    LOCK_PARSES.with(Cell::get)
}

/// This thread's cumulative count of represented manifest file reads. Read before and
/// after an emit run to pin that each manifest is read exactly once per run.
#[must_use]
pub fn manifest_read_count() -> usize {
    MANIFEST_READS.with(Cell::get)
}

fn increment_lock_reads() {
    LOCK_READS.with(|c| c.set(c.get() + 1));
}

fn increment_lock_parses() {
    LOCK_PARSES.with(|c| c.set(c.get() + 1));
}

fn increment_manifest_reads() {
    MANIFEST_READS.with(|c| c.set(c.get() + 1));
}

/// Errors raised by `emit`, `place`, and the lock-reading helpers in this module —
/// a source or lock that fails to read, write, parse, or reproduce deterministically.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum DriftError {
    /// A recorded source path could not be read — and not because it is absent
    /// (a missing source is the `removed` state, not an error).
    #[error("failed to read source {path}")]
    #[diagnostic(code(temper::drift::read))]
    Read {
        /// The source path whose read failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A re-emitted projection could not be written back to the harness during `emit`.
    #[error("failed to write source {path}")]
    #[diagnostic(code(temper::drift::write))]
    Write {
        /// The destination source path that failed to write.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A reaped orphan projection — byte-identical to its lock fingerprint, its
    /// owning member gone — could not be deleted.
    #[error("failed to remove orphaned projection {path}")]
    #[diagnostic(code(temper::drift::remove))]
    Remove {
        /// The orphaned projection path that failed to delete.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The reap wave would delete every live projection the lock owns while
    /// emitting nothing in their place — the `--into` re-root of an adopted
    /// harness turns the whole projection tree ownerless at once. Refused at the
    /// cliff (decision 0024) rather than mass-deleted silently; a genuine full
    /// teardown is the `--teardown` flag the author spells.
    #[error(
        "refusing to reap {count} live projections at once: this would delete every projection the lock owns while emitting nothing in their place (an `--into` re-root strands the whole tree). Pass `--teardown` to tear the harness down on purpose."
    )]
    #[diagnostic(code(temper::drift::total_reap_wave))]
    TotalReapWave {
        /// How many byte-faithful projections the refused wave would have reaped.
        count: usize,
    },

    /// A re-read produced zero members for a whole layer the committed lock still
    /// declares — every embedded member of one host gone at once, the signature of a
    /// pre-0018 harness whose nested members vanish on re-emit. Refused at the cliff
    /// (decision 0024) rather than dropped silently: the loss is loud because the
    /// prior lock still carries the layer, never because a projection's prose was
    /// mined. A genuine layer removal is the `--teardown` flag the author spells.
    #[error(
        "refusing to drop the whole `{host}` embedded-member layer: the committed lock declares {count} nested members under it and this re-emit derives none (a re-read mining no member where the lock still carries a layer). Pass `--teardown` to remove the layer on purpose."
    )]
    #[diagnostic(code(temper::drift::layer_dropped))]
    LayerDropped {
        /// The host member address (`kind:name`) whose embedded-member layer dropped.
        host: String,
        /// How many nested members the committed lock still declares under the host.
        count: usize,
    },

    /// A partial manifest rewrite would drop every discovered member of one collection
    /// while the payload declares none in its place — the whole live `hooks` (or
    /// `enabledPlugins`, `mcpServers`) block a settings.json rewrite would strand. Refused
    /// at the segment cliff, the whole-file [`TotalReapWave`](DriftError::TotalReapWave)
    /// doctrine carried down to a manifest's segments (decision 0024): a genuine segment
    /// teardown is the `--teardown` flag the author spells.
    #[error(
        "refusing to drop the whole `{collection}` collection of `{manifest}`: it carries {count} discovered member(s) on disk and this emit declares none in their place (a partial manifest rewrite would strand them silently). Pass `--teardown` to clear the collection on purpose."
    )]
    #[diagnostic(code(temper::drift::segment_reap_wave))]
    SegmentReapWave {
        /// The manifest whose collection would be emptied, harness-relative.
        manifest: String,
        /// The collection key (`hooks`, `enabledPlugins`, `mcpServers`) being emptied.
        collection: String,
        /// How many discovered members the collection carries on disk.
        count: usize,
    },

    /// A projection did not reproduce byte-for-byte across a double-emit: the
    /// authoring surface is nondeterministic (a timestamp, an unordered map surfacing
    /// into a field). Law 5 makes this a loud failure rather than a silent churn the
    /// next `emit` would rewrite.
    #[error("emit is nondeterministic for {path} (a double-emit produced differing bytes)")]
    #[diagnostic(code(temper::drift::nondeterministic))]
    Nondeterministic {
        /// The projection source path whose re-emit diverged.
        path: PathBuf,
    },

    /// The workspace lock could not be read for its last-applied fingerprints.
    #[error("failed to read lock {path}")]
    #[diagnostic(code(temper::drift::lock_read))]
    LockRead {
        /// The lock path whose read failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The workspace lock is not valid TOML, so its fingerprints cannot be read
    /// or updated.
    #[error("failed to parse lock {path}")]
    #[diagnostic(code(temper::drift::lock_parse))]
    LockParse {
        /// The lock path that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },

    /// No SDK program exists at the harness workspace's entry point — the seam has
    /// nothing to compile from.
    #[error("no SDK program at {path} — the represented path requires one (install scaffolds it)")]
    #[diagnostic(code(temper::drift::no_sdk_program))]
    NoSdkProgram {
        /// The harness entry path that was expected but absent.
        path: PathBuf,
    },

    /// `node` could not be spawned to run the SDK program.
    #[error("failed to run the SDK program {path} (is `node` on PATH?)")]
    #[diagnostic(code(temper::drift::sdk_spawn))]
    SdkProgramSpawn {
        /// The harness entry path the process was invoked with.
        path: PathBuf,
        /// The underlying spawn error.
        #[source]
        source: std::io::Error,
    },

    /// The SDK program exited non-zero — a refusal
    /// or an authoring error; its stderr carries the reason.
    #[error("the SDK program {path} exited with a failure:\n{stderr}")]
    #[diagnostic(code(temper::drift::sdk_program_failed))]
    SdkProgramFailed {
        /// The harness entry path that failed.
        path: PathBuf,
        /// The program's captured stderr.
        stderr: String,
    },

    /// The SDK program's stdout was not valid UTF-8 — the JSON pipe is text.
    #[error("the SDK program {path} printed non-UTF-8 output")]
    #[diagnostic(code(temper::drift::sdk_program_output))]
    SdkProgramOutput {
        /// The harness entry path whose output failed to decode.
        path: PathBuf,
        /// The underlying UTF-8 decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// The SDK program's stdout did not parse as the seam's JSON payload.
    #[error("the SDK program {path} printed a payload that failed to parse")]
    #[diagnostic(code(temper::drift::payload_parse))]
    PayloadParse {
        /// The harness entry path whose payload failed to parse.
        path: PathBuf,
        /// The underlying JSON parse error.
        #[source]
        source: serde_json::Error,
    },

    /// The payload's pinned `version` does not match the engine's — the SDK and the
    /// engine have drifted out of the lockstep the seam requires.
    #[error(
        "the SDK program's payload declares seam version {got}; this engine reads version {SEAM_VERSION}"
    )]
    #[diagnostic(code(temper::drift::seam_version))]
    UnsupportedSeamVersion {
        /// The version the payload declared.
        got: u32,
    },

    /// A projected member's payload names a kind absent from the payload's own
    /// `declarations.kinds` family — the engine is kind-blind and has nowhere to read that kind's locus/format/unit-shape from.
    #[error(
        "member `{member}` names kind `{kind}`, which the payload's declarations carry no kind fact for"
    )]
    #[diagnostic(code(temper::drift::unknown_kind))]
    UnknownKind {
        /// The kind name the member declared.
        kind: String,
        /// The member that named it.
        member: String,
    },

    /// A prose reference — a layout region's `import` or a composed-prose `include` —
    /// names a file that does not exist on disk. Refused before a byte is written: the
    /// author cannot produce output from a source that references content that is not
    /// there.
    #[error(
        "member `{member}` references `{import}`, resolving to `{path}`, which does not exist — a dangling reference"
    )]
    #[diagnostic(code(temper::drift::dangling_import))]
    DanglingImport {
        /// The referencing member's `kind:name` address.
        member: String,
        /// The reference the source declared, verbatim.
        import: String,
        /// The path the reference resolved to.
        path: PathBuf,
    },

    /// A composed-prose include target's bytes are not valid UTF-8 — an include splices
    /// the target's text into the host projection, so a non-text target cannot be pulled
    /// in. Refused before a byte is written.
    #[error(
        "member `{member}` includes `{path}`, whose bytes are not valid UTF-8 — a composed-prose include pulls text, not binary"
    )]
    #[diagnostic(code(temper::drift::include_not_utf8))]
    IncludeNotUtf8 {
        /// The including member's `kind:name` address.
        member: String,
        /// The include target's path.
        path: String,
    },

    /// A composed-prose member's body carries a different number of include slots than
    /// the payload declares includes for it — a malformed seam (the SDK plants one slot
    /// per include). Refused before a byte is written.
    #[error(
        "member `{member}` declares {declared} include(s) but its body carries {slots} include slot(s) — a malformed seam"
    )]
    #[diagnostic(code(temper::drift::include_arity))]
    IncludeArity {
        /// The member's `kind:name` address.
        member: String,
        /// The number of includes the payload declares for the member.
        declared: usize,
        /// The number of include slots the member's body carries.
        slots: usize,
    },

    /// A member carries an authored body at a format that renders its fields alone — a
    /// `json-document` artifact is one JSON object, with no prose slot to render words
    /// into. Refused before a byte is written rather than projected without them: temper
    /// never drops authored words (invariant 3), and never silently degrades a projection
    /// it could refuse (invariant 6). The honest repair is the author's, not the engine's —
    /// there is no home to invent here.
    #[error(
        "member `{member}` carries an authored body, but its `{format}` format renders its fields alone — the words have no home in this format; move them to a body-bearing member or drop them at the source"
    )]
    #[diagnostic(code(temper::drift::body_has_no_home))]
    BodyHasNoHome {
        /// The member's `kind:name` address.
        member: String,
        /// The declared format label the member's kind carries.
        format: String,
    },

    /// A member's kind declares a **read face only** — `toml-document` has no write twin,
    /// and none is coming: emit's codomain is the committed tree, and a format joined for
    /// reading uncommitted documents has no projection to render. Refused loud rather than
    /// rendered through whichever encoder the write dispatch would otherwise fall through
    /// to, which would put a member's fields on disk in a format its author never declared
    /// (invariant 6: no path silently degrades).
    #[error(
        "member `{member}` declares format `{format}`, which is a read face only — it has no write face to project through"
    )]
    #[diagnostic(code(temper::drift::format_has_no_write_face))]
    FormatHasNoWriteFace {
        /// The member's `kind:name` address.
        member: String,
        /// The declared format label the member's kind carries.
        format: String,
    },

    /// A harness-level settings-residue key names a manifest no in-play kind declares — so
    /// there is no manifest to fold it into. Refused loud before a byte is written rather
    /// than shedding the authored key (invariant 6).
    #[error(
        "settings residue key `{key}` names manifest `{manifest}`, which no in-play kind declares — it has nowhere to land"
    )]
    #[diagnostic(code(temper::drift::unplaceable_settings))]
    UnplaceableSettings {
        /// The manifest the residue key named.
        manifest: String,
        /// The residue key that could not be placed.
        key: String,
    },

    /// A harness-level settings-residue key collides with a differing residue value already
    /// present in its manifest (a container member's field, or a duplicate settings row).
    /// Refused loud rather than silently shedding one of the two values (invariant 6).
    #[error(
        "settings residue key `{key}` collides with a differing value already in manifest `{manifest}`"
    )]
    #[diagnostic(code(temper::drift::settings_residue_collision))]
    SettingsResidueCollision {
        /// The manifest carrying the colliding key.
        manifest: String,
        /// The colliding residue key.
        key: String,
    },

    /// A flat `file` kind's `governs_glob` is neither a single-segment single-`*`
    /// pattern nor an any-depth `**` glob, so its members have no one path to project
    /// onto — name-splicing the first `*` would leave a stray literal `*` (a multi-star
    /// glob) or a literal directory segment (a multi-segment glob) in the derived path.
    /// Refused loud before a byte is written (invariant 6) rather than emitting the
    /// nonsense path: depth is a skill (agent-loaded) or a nesting kind (governed
    /// content), never a directory-sliced flat file.
    #[error(
        "kind `{kind}` governs a flat file at glob `{glob}`, which is neither a single-segment `*` pattern nor an any-depth `**` glob — a flat file kind maps its name through exactly one `*`; for depth use a skill (agent-loaded) or a nesting kind (governed content)"
    )]
    #[diagnostic(code(temper::drift::flat_glob_depth))]
    FlatGlobDepth {
        /// The kind whose glob cannot map its members to one projection path.
        kind: String,
        /// The offending glob, verbatim.
        glob: String,
    },

    /// A nested file member's path cannot be composed: its host's unit and the host
    /// template's pattern are the two halves it derives from, and one is missing. The
    /// child kind governs no glob of its own, so there is no second derivation to fall
    /// back on — refused loud before a byte is written (invariant 6) rather than guessing
    /// a locus.
    #[error(
        "nested file member `{member}` of kind `{kind}` has no path to compose: {detail} — a nested file child's path composes from its host's unit and the host template's pattern, and the pattern is the host kind's declared fact"
    )]
    #[diagnostic(code(temper::drift::nested_file_locus))]
    NestedFileLocus {
        /// The nested file kind whose member cannot be placed.
        kind: String,
        /// The member with no composable path.
        member: String,
        /// Which half of the composition is missing.
        detail: String,
    },

    /// A committed lock carries a present-but-malformed declaration row — a required
    /// column absent, a column the wrong type, a malformed nested element, or a label
    /// outside its closed vocabulary. Surfaced at load rather than silently dropped: a
    /// dropped row would narrow the gate's verdict against a corrupt lock.
    #[error(transparent)]
    #[diagnostic(transparent)]
    LockRow(#[from] LockRowError),
}

/// One row of the `lock.toml` roll-up index: an artifact's identity, its source
/// provenance, and its two freshness facts — disk-vs-lock drift's whole comparison.
/// Shared by every kind —
/// a `[[skill]]`, `[[rule]]`, and every custom `[[<kind>]]` row all carry the same
/// four columns.
///
/// `pub(crate)` so `emit` can build the row for a freshly projected member and hand it
/// to [`write_rollup`] rather than re-deriving the fingerprints.
pub(crate) struct RollupEntry {
    /// Artifact name (and its `<kind>/<name>/` surface directory).
    pub(crate) name: String,
    /// Path to the original source file, as given relative to the harness arg.
    pub(crate) source_path: String,
    /// SHA-256 of the authored source bytes — the **source freshness fact**, the
    /// anchor source-drift detection compares against.
    pub(crate) source_hash: String,
    /// SHA-256 of the last emitted projection — the **emit freshness fact**, the
    /// baseline `config.stale` and projection freshness compare a committed output
    /// against. At import it provisionally equals `source_hash`: no `emit` has run
    /// yet, so the last thing projected onto the source is the source as imported
    /// (`emit` advances it once it lands).
    pub(crate) emit_hash: String,
}

/// Write the `<into>/lock.toml` roll-up: one `[[<kind>]]` table per emitted member,
/// key-sorted, each with `name`, `source_path`, `source_hash`, and the `emit_hash`
/// fingerprint. `emit` is the sole caller: a kind with no emitted member simply has
/// no entry, matching the toml round-trip reality — an empty `ArrayOfTables` emits
/// nothing, so a written-then-vanished section would break idempotence against a
/// re-parse that never sees it.
///
/// After the per-member sections come the program's **declaration rows** — kind facts,
/// clauses, requirements, assembly facts under an implicit `[declaration]` table;
/// the gate side reads them through [`read_declarations`]. The `nested_member` family
/// carries the program's own embedded-member facts *and* the rows emit derives from
/// layout sources in the same pass (`emit` merges them before this write), so a layout
/// document's members reach the lock as declaration rows without a projection of their
/// own. `layout_imports` and `includes` are the layout sources' and composed prose's
/// fingerprinted content dependencies, written into the same `[declaration]` table
/// under their own families.
pub(crate) fn write_rollup(
    into: &Path,
    rollups: &BTreeMap<String, Vec<RollupEntry>>,
    declarations: &Declarations,
    layout_imports: &[LayoutImportRow],
    includes: &[LayoutImportRow],
) -> Result<(), DriftError> {
    let mut doc = DocumentMut::new();
    for (kind, rows) in rollups {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(rows));
    }
    declarations.write_into(&mut doc);
    write_source_deps(&mut doc, "layout_import", layout_imports);
    write_source_deps(&mut doc, "include", includes);

    let path = into.join(crate::LOCK_FILENAME);
    crate::fs_util::write_creating_parents(&path, doc.to_string().as_bytes())
        .map_err(|source| DriftError::Write { path, source })
}

/// Build the `ArrayOfTables` for one kind's roll-up rows — the four shared columns
/// (`name`, `source_path`, `source_hash`, `emit_hash`) in a fixed order, one
/// table per entry.
fn rollup_tables(rollup: &[RollupEntry]) -> ArrayOfTables {
    let mut tables = ArrayOfTables::new();
    for entry in rollup {
        let mut table = Table::new();
        table["name"] = value(entry.name.clone());
        table["source_path"] = value(entry.source_path.clone());
        table["source_hash"] = value(entry.source_hash.clone());
        table["emit_hash"] = value(entry.emit_hash.clone());
        tables.push(table);
    }
    tables
}

/// A present declaration row the lock reader could not lift. The lock is tool-written
/// and never hand-patched, so a row the SDK could not have emitted is a corrupt lock
/// rejected loud at load, never a silently dropped row narrowing the gate's verdict. A
/// missing lock, an absent family, and an absent optional column stay legitimate
/// absence — only a *present* row that fails its lift is an error.
#[derive(Debug, Clone, thiserror::Error, miette::Diagnostic)]
#[diagnostic(code(temper::drift::lock_row))]
pub enum LockRowError {
    /// A present row omits a column its family requires.
    #[error("lock `{family}` declaration row is missing the required `{column}` column")]
    MissingColumn {
        /// The declaration family the row belongs to.
        family: String,
        /// The absent required column.
        column: String,
    },
    /// A present row's column is not the TOML type its family declares.
    #[error("lock `{family}` declaration row column `{column}` is not the expected {want}")]
    WrongType {
        /// The declaration family the row belongs to.
        family: String,
        /// The mis-typed column.
        column: String,
        /// The type the column was expected to hold.
        want: String,
    },
    /// A present family is not the array-of-tables shape the reader expects.
    #[error("lock `{family}` declaration family is not an array of tables")]
    FamilyShape {
        /// The mis-shaped declaration family.
        family: String,
    },
    /// A present row carries a label outside its column's closed vocabulary.
    #[error(
        "lock `{family}` declaration row column `{column}` value `{value}` is outside the closed vocabulary"
    )]
    Vocabulary {
        /// The declaration family the row belongs to.
        family: String,
        /// The column carrying the out-of-vocabulary label.
        column: String,
        /// The unrecognized label.
        value: String,
    },
}

/// A declaration-row column problem before the family that scopes it is known — the
/// per-column lifts raise this, and [`family`] attaches the family name to make a
/// [`LockRowError`].
#[derive(Debug)]
enum RowError {
    Missing { column: String },
    WrongType { column: String, want: &'static str },
}

impl RowError {
    fn missing(column: &str) -> Self {
        Self::Missing {
            column: column.to_string(),
        }
    }

    fn wrong(column: &str, want: &'static str) -> Self {
        Self::WrongType {
            column: column.to_string(),
            want,
        }
    }

    /// Scope this column problem to `family`, producing the surfaced [`LockRowError`].
    fn at(self, family: &str) -> LockRowError {
        match self {
            RowError::Missing { column } => LockRowError::MissingColumn {
                family: family.to_string(),
                column,
            },
            RowError::WrongType { column, want } => LockRowError::WrongType {
                family: family.to_string(),
                column,
                want: want.to_string(),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// emit — the write direction
// ---------------------------------------------------------------------------

/// The marker the SDK plants in a composed-prose body per include, in authored order —
/// `U+0001`, the include counterpart to the SDK's `U+0000` mention marker (`prose.ts`).
/// A rendered body carries no mention markers (those resolve to display text SDK-side),
/// so splitting a body on this byte recovers the literal chunks the include contents
/// interleave between.
const INCLUDE_SLOT: char = '\u{1}';

/// Options controlling an [`emit`] run.
#[derive(Debug, Clone, Copy, Default)]
pub struct EmitOptions {
    /// When set, compute every projection and report it but write nothing — neither
    /// the re-emitted harness sources nor the updated lock fingerprints.
    pub dry_run: bool,
    /// Refuse network access — the CI posture.
    /// `emit` performs no network I/O today (it compiles a materialized
    /// surface), so this changes nothing yet; accepted for CLI-surface / CI parity.
    pub frozen: bool,
    /// Spell a full teardown: let a reap wave that would delete every live
    /// projection through instead of refusing it. Off by default, so the cliff
    /// refusal guards a re-rooted `--into` (or any wholesale ownerless sweep)
    /// unless the author names the teardown on purpose.
    pub teardown: bool,
}

/// One artifact's outcome from an [`emit`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitOutcome {
    /// The projection was re-emitted whole to match the surface (or, under
    /// `--dry-run`, would have been): its bytes differed from disk, or the source
    /// was absent. Emit regenerates from the authored source, so a hand-edited
    /// projection is overwritten — that edit is drift routed to the source, never a
    /// merge.
    Emitted,
    /// The re-emitted projection already sat on disk byte-for-byte; nothing to
    /// write. The idempotent no-op — a re-run of a clean emit lands here for every
    /// artifact.
    Unchanged,
    /// The prior lock named this projection but no current member owns it (its
    /// member was dropped from the program), and the on-disk bytes still hashed to
    /// the lock's recorded `emit_hash` — temper wrote every one of those bytes, so
    /// deleting it (or, under `--dry-run`, reporting that it would be deleted)
    /// loses nothing authored.
    Reaped,
    /// The prior lock named this projection but no current member owns it, and the
    /// on-disk bytes no longer hash to the lock's recorded `emit_hash` — a hand
    /// edit, or some other out-of-band change. Left on disk and only reported:
    /// deleting hand-authored bytes is never the safe default.
    OrphanDrift,
    /// A discovered manifest member — a hook, an installed plugin, an MCP server, or an
    /// opaque residue key — the on-disk manifest carried and the payload no longer
    /// declares. A represented manifest is regenerated whole, so an undeclared member would
    /// vanish with no finding; this names it in the reap ledger instead. The segment-level
    /// peer of [`Reaped`](EmitOutcome::Reaped): temper wrote the manifest, so dropping the
    /// member loses nothing authored, but the drop is surfaced, never silent.
    MemberReaped,
}

impl EmitOutcome {
    /// The lower-case label used in the rendered report and stable for tests.
    #[must_use]
    fn label(self) -> &'static str {
        match self {
            EmitOutcome::Emitted => "emitted",
            EmitOutcome::Unchanged => "unchanged",
            EmitOutcome::Reaped => "reaped",
            EmitOutcome::OrphanDrift => "orphan-drift",
            EmitOutcome::MemberReaped => "member-reaped",
        }
    }
}

/// One row of an [`EmitReport`]: which artifact, of which kind, located where, and
/// the outcome emit produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitEntry {
    /// The artifact kind — the payload member's bare kind name (`"skill"`, `"rule"`, …).
    pub kind: String,
    /// The artifact name (its surface name).
    pub name: String,
    /// The on-disk source path the projection targeted.
    pub source_path: PathBuf,
    /// What `emit` did (or would do, under `--dry-run`) for this artifact.
    pub outcome: EmitOutcome,
}

/// The typed result of an [`emit`]: every current artifact's outcome, in the
/// payload's stable load order (kind-then-name), followed by an entry for every
/// lock-known projection the payload no longer owns (reaped or drifted-orphan).
/// Renders nothing itself — [`render_emit`] turns it into text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitReport {
    /// Every projected artifact, across every kind the payload names, plus any
    /// ownerless projection the prior lock still named.
    pub entries: Vec<EmitEntry>,
}

/// The engine's pinned seam version — the JSON pipe rides it in lockstep with the
/// SDK's own `SEAM_VERSION`.
pub const SEAM_VERSION: u32 = 2;

/// One projected member's erased payload — the SDK's whole output surface for a
/// member that lives at a path locus (`sdk/src/generated/PayloadMember`). An
/// embedded member never appears here (it carries no standalone projection); its
/// facts ride the [`NestedMemberRow`] family instead.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct PayloadMember {
    /// The kind's bare name — joins this payload's own `declarations.kinds` family.
    pub kind: String,
    /// Identity within the kind.
    pub name: String,
    /// The `kind:name` address of the host member this member's unit composes under —
    /// carried by a **nested file** child, whose path is its host's unit joined with the
    /// host template's pattern; absent at every other locus.
    #[serde(default)]
    pub host: Option<String>,
    /// The kind's typed fields, flat and ordered — the projected frontmatter. The
    /// value is arbitrary JSON, so the seam type is `unknown`, never a serde_json
    /// binding — the SDK reads the field values, never the engine over this pipe.
    #[ts(type = "Array<[string, unknown]>")]
    pub fields: Vec<(String, JsonValue)>,
    /// The resolved prose body, byte-faithful.
    pub body: String,
    /// The resolved `file()` asset's absolute path, when the member's prose is
    /// `file()` — absent for `text`/`blocks` prose or no prose.
    #[serde(default)]
    pub source_path: Option<String>,
}

/// The whole seam payload the SDK program prints to stdout:
/// the
/// declaration rows (the lock's seven families) and every projected member's erased
/// payload. The engine is the sole compiler of every projection and the whole lock
/// from this one value — no harness re-supply, the payload IS the source.
///
/// Not `Eq`: its declarations may carry `f64` `range` bounds ([`RangeBoundRow`]).
#[derive(Debug, Clone, Deserialize, PartialEq, ts_rs::TS)]
pub struct Payload {
    /// The pinned seam version this payload was compiled against.
    pub version: u32,
    /// The seven declaration families.
    pub declarations: Declarations,
    /// Every projected member.
    pub members: Vec<PayloadMember>,
}

/// The desired projection of one member: its identity, the harness-relative path it
/// projects to, and its fields/body.
struct Projection {
    kind: String,
    name: String,
    /// The path the artifact projects to, relative to the harness root — the lock's own
    /// vocabulary. [`emit_one`] joins it onto the harness root to reach disk.
    source_path: PathBuf,
    /// The kind's declared projection format, selecting the canonical write face the
    /// member's bytes are rendered through. `None` for a kind declaring none.
    format: Option<Format>,
    /// The desired header fields in canonical order (known fields first, then the
    /// preserved unknown keys). The whole set is re-emitted into a fresh
    /// frontmatter block — the projection is regenerated, never patched.
    fields: Vec<(String, JsonValue)>,
    /// The desired body — the surface body, projected byte-faithfully.
    body: String,
}

/// Render a path for a lock row's `source_path`: always `/`-separated and with no
/// redundant `./` prefix, regardless of host. `lock.toml` is committed, and
/// `Path::join` inserts the host separator at each join boundary (backslash on
/// Windows) — left alone, that forks the byte-committed lock by host.
pub(crate) fn to_lock_path(path: &Path) -> String {
    normalize_lock_path(&path.to_string_lossy())
}

/// Canonicalize a lock `source_path` spelling for a byte-stable, host- and
/// engine-independent join: `/`-separated, no leading `./`. An engine before the
/// workspace path was lexically normalized spelled a root-`.` member `./CLAUDE.md`
/// where the current pass keys the bare `CLAUDE.md`; the reap sweep must normalize
/// both sides at read time (decision 0024) or the older lock's every live
/// projection reads ownerless and is mass-reaped.
fn normalize_lock_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}

/// The harness root a `.temper` workspace sits inside (`<harness>/.temper` → `<harness>`):
/// the base every lock row is spelled relative to, and the base a reader joins a row back
/// onto to reach disk.
///
/// `.` when the workspace names no parent segment. `./.temper` and `.temper` name one
/// surface, but their raw `parent()`s differ (`.` vs the empty path) — left alone that
/// forks a row's spelling between two emits of the same workspace, and an empty root
/// cannot be made absolute, which would silently spell a relativized target absolute
/// instead ([`harness_relative`]). Lexical normalization plus the `.` floor collapses both
/// to one root.
pub fn harness_root_of(workspace_dir: &Path) -> PathBuf {
    let normalized = crate::path::normalize_path(workspace_dir);
    match normalized.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}

/// `relative` joined under a locus `root`, dropping the `.` a root-locus kind carries.
pub fn join_locus(root: &str, relative: &str) -> PathBuf {
    if root == "." {
        PathBuf::from(relative)
    } else {
        Path::new(root).join(relative)
    }
}

/// `name` spliced through `pattern`'s single `*` — the one name-through-a-glob map, shared
/// by a flat `governs` glob and a host template's path pattern. A `*`-free pattern is a
/// fixed path (a manifest container's `settings.json`), spliced nowhere and left verbatim.
///
/// `starred_segment` admits a `*/<file>` glob whose single `*` stars a whole leading
/// directory segment (a starred-segment kind's locus), landing `<name>/<file>`; every other
/// caller passes `false`, where a `/` beside the `*` is a stray directory the splice cannot
/// place.
///
/// # Errors
/// Returns [`DriftError::FlatGlobDepth`] when `pattern` carries a `*` but is neither
/// single-star nor single-segment (and not the admitted leading-segment case): the splice
/// would leave a stray literal `*` (multi-star) or a literal directory segment
/// (multi-segment) in the path.
fn splice_name(
    kind: &str,
    pattern: &str,
    name: &str,
    starred_segment: bool,
) -> Result<String, DriftError> {
    let stars = pattern.matches('*').count();
    let leading_segment = starred_segment && stars == 1 && pattern.starts_with("*/");
    if stars > 0 && !leading_segment && (stars > 1 || pattern.contains('/')) {
        return Err(DriftError::FlatGlobDepth {
            kind: kind.to_string(),
            glob: pattern.to_string(),
        });
    }
    Ok(pattern.replacen('*', name, 1))
}

/// A nested file child's harness-relative locus: its host member's unit joined with the
/// host kind's template pattern for this child kind, the child's name spliced through the
/// pattern. The pattern is the host's declared fact — one home — so the child kind governs
/// no glob and can never contend with its host's own.
///
/// # Errors
/// Returns [`DriftError::NestedFileLocus`] when the host cannot supply both halves of the
/// composition: no host address, an address naming no declared kind, a host kind
/// templating no file layer for this child, or a host owning no directory unit (a
/// template's pattern is relative to its unit, and a lone file has no interior).
/// Propagates [`DriftError::FlatGlobDepth`] from the pattern's own name splice.
fn nested_file_path(
    facts: &KindFactRow,
    name: &str,
    host: Option<&str>,
    kind_facts: &BTreeMap<&str, &KindFactRow>,
) -> Result<PathBuf, DriftError> {
    let refuse = |detail: String| DriftError::NestedFileLocus {
        kind: facts.name.clone(),
        member: name.to_string(),
        detail,
    };
    let address = host.ok_or_else(|| refuse("it names no host member".to_string()))?;
    let (host_kind, host_name) = address
        .split_once(':')
        .ok_or_else(|| refuse(format!("its host `{address}` is no `kind:name` address")))?;
    let host_facts = kind_facts
        .get(host_kind)
        .ok_or_else(|| refuse(format!("its host `{address}` names no declared kind")))?;
    let pattern = host_facts
        .templates
        .iter()
        .find(|template| template.kind == facts.name && template.path.is_some())
        .and_then(|template| template.path.as_deref())
        .ok_or_else(|| {
            refuse(format!(
                "its host kind `{host_kind}` templates no file layer for it"
            ))
        })?;
    let (Some(host_root), Some("directory")) = (
        host_facts.governs_root.as_deref(),
        host_facts.unit_shape.as_deref(),
    ) else {
        return Err(refuse(format!(
            "its host `{address}` owns no directory unit to compose under"
        )));
    };
    let leaf = splice_name(&facts.name, pattern, name, false)?;
    Ok(join_locus(host_root, &format!("{host_name}/{leaf}")))
}

/// The harness-relative locus a member of `facts` named `name` projects onto: a directory
/// unit lands its entry file under `<root>/<name>/`; a lone file replaces the glob's `*`
/// with the name (an any-depth glob, a memory kind's `**/CLAUDE.md`, lands the root
/// `<name>.md`; a starred-segment kind's `*/<file>` lands `<root>/<name>/<file>`); a nested
/// file child — one governing no glob — composes its path under
/// `host`'s own unit ([`nested_file_path`]). The SDK's `projectionPath`
/// (`sdk/src/emit.ts`) derives the same locus from the same facts, and
/// `tests/projection_path_seam.rs` gates the two into agreement.
///
/// # Errors
/// Returns [`DriftError::FlatGlobDepth`] when a glob maps its member name to no one path
/// ([`splice_name`]), or [`DriftError::NestedFileLocus`] when a nested file child's host
/// supplies no unit and pattern to compose against.
fn member_projection_path(
    facts: &KindFactRow,
    name: &str,
    host: Option<&str>,
    kind_facts: &BTreeMap<&str, &KindFactRow>,
) -> Result<PathBuf, DriftError> {
    // The two governs columns are one spelling: present together at an `at` locus, absent
    // together for a nested file kind, whose path composes from its host instead.
    let (Some(root), Some(glob)) = (facts.governs_root.as_deref(), facts.governs_glob.as_deref())
    else {
        return nested_file_path(facts, name, host, kind_facts);
    };
    let relative = if facts.unit_shape.as_deref() == Some("directory") {
        let entry = glob.split_once('/').map_or(glob, |(_, rest)| rest);
        format!("{name}/{entry}")
    } else if glob.contains("**") {
        format!("{name}.md")
    } else {
        let starred_segment = facts.unit_shape.as_deref() == Some("starred-segment");
        splice_name(&facts.name, glob, name, starred_segment)?
    };
    Ok(join_locus(root, &relative))
}

/// Run the SDK program at `<workspace_dir>/harness.ts` and compile its payload in one
/// call — the whole seam: `node` executes the authored program, the engine reads the JSON
/// pipe it prints on stdout and becomes the sole compiler of every projection and the
/// whole lock. No harness root is re-supplied — the payload IS the source.
///
/// # Errors
/// Returns a [`DriftError`] if no SDK program exists at the entry point, `node`
/// cannot be spawned, the program exits non-zero, its output fails to parse, or
/// [`emit`] itself fails.
pub fn emit_program(workspace_dir: &Path, options: EmitOptions) -> miette::Result<EmitReport> {
    let harness_entry = workspace_dir.join("harness.ts");
    if !harness_entry.is_file() {
        return Err(DriftError::NoSdkProgram {
            path: harness_entry,
        }
        .into());
    }
    let json = run_sdk_program(&harness_entry)?;
    peek_and_validate_seam_version(&json)?;
    let payload: Payload =
        serde_json::from_str(&json).map_err(|source| DriftError::PayloadParse {
            path: harness_entry.clone(),
            source,
        })?;
    emit(&payload, workspace_dir, options)
}

/// Execute the SDK program at `harness_entry` (`node <path>`) and capture its
/// stdout — the internal versioned JSON pipe. The subprocess's working directory
/// is the program's own directory, so a bare `@dtmd/temper` import resolves
/// through the consuming project's `node_modules`, walking up from there exactly
/// as Node's own resolution would from the program's location. The `node` arg
/// itself is canonicalized first: a relative `harness_entry` (the `./.temper`
/// default) would otherwise be re-resolved by Node against the *new* cwd once
/// `current_dir` moves under it, doubling the path (`./.temper/.temper/harness.ts`,
/// cascade field report 07-06) — an absolute arg is unambiguous regardless of cwd.
fn run_sdk_program(harness_entry: &Path) -> Result<String, DriftError> {
    let cwd = harness_entry.parent().unwrap_or_else(|| Path::new("."));
    let entry_arg =
        fs::canonicalize(harness_entry).map_err(|source| DriftError::SdkProgramSpawn {
            path: harness_entry.to_path_buf(),
            source,
        })?;
    let entry_arg = strip_verbatim_prefix(&entry_arg);
    let output = Command::new("node")
        .arg(&entry_arg)
        .current_dir(cwd)
        .output()
        .map_err(|source| DriftError::SdkProgramSpawn {
            path: harness_entry.to_path_buf(),
            source,
        })?;
    if !output.status.success() {
        return Err(DriftError::SdkProgramFailed {
            path: harness_entry.to_path_buf(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    String::from_utf8(output.stdout).map_err(|source| DriftError::SdkProgramOutput {
        path: harness_entry.to_path_buf(),
        source,
    })
}

/// Strip Windows' `\\?\` verbatim-path prefix from a canonicalized path.
///
/// `fs::canonicalize` on Windows always returns the verbatim form (plain
/// `\\?\C:\...` or UNC `\\?\UNC\server\share\...`), which Node's
/// `resolveMainPath` rejects outright. Elsewhere `canonicalize` never
/// produces this prefix, so this is a no-op.
fn strip_verbatim_prefix(path: &Path) -> PathBuf {
    let Some(raw) = path.to_str() else {
        return path.to_path_buf();
    };
    if let Some(rest) = raw.strip_prefix(r"\\?\UNC\") {
        PathBuf::from(format!(r"\\{rest}"))
    } else if let Some(rest) = raw.strip_prefix(r"\\?\") {
        PathBuf::from(rest)
    } else {
        path.to_path_buf()
    }
}

/// Peek at the `version` field in a raw JSON payload string and validate it matches
/// [`SEAM_VERSION`]. Extracts the version before attempting a full strict deserialize, so a
/// mismatched version surfaces as [`DriftError::UnsupportedSeamVersion`] even when the
/// remaining payload shape is incompatible with [`Payload`].
///
/// # Errors
/// Returns [`DriftError::UnsupportedSeamVersion`] if the version field does not match
/// [`SEAM_VERSION`].
fn peek_and_validate_seam_version(json: &str) -> Result<(), DriftError> {
    #[derive(Deserialize)]
    struct VersionPeek {
        version: u32,
    }

    let peek: VersionPeek =
        serde_json::from_str(json).map_err(|_| DriftError::UnsupportedSeamVersion { got: 0 })?;

    if peek.version != SEAM_VERSION {
        return Err(DriftError::UnsupportedSeamVersion { got: peek.version });
    }

    Ok(())
}

/// Compile a seam `payload` into every projection and the whole lock — the sole
/// compiler. `workspace_dir` is the
/// surface root (`.temper`, carrying `lock.toml`); projections land beside it, at
/// `workspace_dir`'s parent joined with each member's kind-derived locus. Every
/// projection is double-emit verified (`emit_one`); the lock is rewritten whole,
/// never patched — it is tool-written, never composed. Nothing is written under
/// `options.dry_run`.
///
/// # Errors
/// Returns a [`DriftError`] if the payload's seam version is unsupported, a member
/// names an undeclared kind, a projection cannot be read/written, or a projection
/// fails to reproduce byte-for-byte across a double-emit.
pub fn emit(
    payload: &Payload,
    workspace_dir: &Path,
    options: EmitOptions,
) -> miette::Result<EmitReport> {
    if payload.version != SEAM_VERSION {
        return Err(DriftError::UnsupportedSeamVersion {
            got: payload.version,
        }
        .into());
    }

    // Every path this pass books — member index keys, projection paths, manifest keys,
    // owned paths, every lock row — is spelled relative to the harness root, and the root
    // is joined back on only where disk is actually touched. The lock is committed and a
    // verb may be aimed at a harness from any cwd, so a row that baked in the cwd prefix
    // this emit happened to run under would resolve from nowhere else.
    let harness_root = harness_root_of(workspace_dir);
    let kind_facts: BTreeMap<&str, &KindFactRow> = payload
        .declarations
        .kinds
        .iter()
        .map(|row| (row.name.as_str(), row))
        .collect();

    // The projection-path → `kind:name` index a layout import resolves its target
    // against, built over every member before any projection is derived: an import may
    // point at a member that appears later in the list, so the whole map must be on hand
    // first. Keyed by lexically-normalized path so a resolved target joins it cleanly.
    let member_index = member_path_index(&payload.members, &kind_facts)?;

    // The composed-prose includes each host member declares, grouped by host address in
    // authored order — the same order the body's include slots carry, so the k-th slot
    // pulls the k-th include's target.
    let mut includes_by_member: BTreeMap<&str, Vec<&IncludeRow>> = BTreeMap::new();
    for include in &payload.declarations.includes {
        includes_by_member
            .entry(include.member.as_str())
            .or_default()
            .push(include);
    }

    let mut projections = Vec::with_capacity(payload.members.len());
    // The composed-prose includes emit resolved this pass — each fingerprinted as a
    // never-reaped source dependency (its own `include` lock family), refusing loud when
    // the target dangles (below), the same posture a layout import takes.
    let mut include_rows: Vec<LayoutImportRow> = Vec::new();
    // A layout kind's document is a source, not a projection: emit reads it under the
    // declared layout and derives its declaration rows, but writes nothing at its path
    // and never reaps it. Its rows join the lock's `nested_member` family alongside the
    // program's own.
    let mut layout_rows = Vec::new();
    // The layout prose imports emit resolved this pass — each a content dependency the
    // lock fingerprints, refusing loud when the target is dangling (below).
    let mut layout_import_rows: Vec<LayoutImportRow> = Vec::new();
    // The `satisfies` fill edges emit derived from layout edge slots this pass — merged
    // into the program's own `satisfies` family, so a layout host's fills reach the
    // roster/coverage/graph tiers exactly as a file-content member's do.
    let mut layout_satisfies: Vec<SatisfiesRow> = Vec::new();
    let mut layout_paths: BTreeSet<String> = BTreeSet::new();
    // The local-locus members this pass passed over: emit writes nothing at their paths
    // and rows none of them, but the paths are still owned — an author's uncommitted
    // document is never an orphan for the reap to claim (the member loop below).
    let mut local_paths: BTreeSet<String> = BTreeSet::new();

    // The represented manifests this pass writes whole through the canonical write face,
    // keyed by their host manifest's on-disk path (resolved off the registration kind's
    // `governs`): each carries its declared collection segments and, when a container
    // member projects to the same path, that member's opaque residue. Built before the
    // member loop so a container member is recognized by path as the loop reaches it.
    // Keyed harness-relative, the vocabulary a member's own projection path speaks.
    let mut manifests: BTreeMap<PathBuf, ManifestBuild> = BTreeMap::new();
    for registration in &payload.declarations.registrations {
        let facts =
            kind_facts
                .get(registration.kind.as_str())
                .ok_or_else(|| DriftError::UnknownKind {
                    kind: registration.kind.clone(),
                    member: registration.key.clone(),
                })?;
        let path = manifest_target_path(facts).ok_or_else(|| DriftError::NestedFileLocus {
            kind: registration.kind.clone(),
            member: registration.key.clone(),
            detail: "it declares a collection address, and a manifest's host file lives at \
                         the governs path this kind has none of"
                .to_string(),
        })?;
        let collection = collection_key_of(&registration.key_path);
        let segment = manifests
            .entry(path)
            .or_default()
            .segments
            .entry(collection.clone())
            .or_default();

        let entry_shape = collection_address_from_row(facts)?
            .map(|addr| addr.entry_shape)
            .ok_or_else(|| DriftError::UnknownKind {
                kind: registration.kind.clone(),
                member: registration.key.clone(),
            })?;

        match entry_shape {
            crate::kind::EntryShape::GroupArray {
                member_key,
                lifted_fields,
            } => {
                // A group-array entry value is Claude Code's array of matcher groups, not a
                // lone entry object: nest this member's fields into one group and append it
                // under the key, so members sharing a key accumulate into the one array (the
                // flat object a naive insert would write is silently ignored).
                let group = crate::json_manifest::hook_matcher_group(
                    &registration.fields,
                    &member_key,
                    &lifted_fields,
                );
                if let JsonValue::Array(groups) = segment
                    .entry(registration.key.clone())
                    .or_insert_with(|| JsonValue::Array(Vec::new()))
                {
                    groups.push(group);
                }
            }
            crate::kind::EntryShape::Scalar { field } => {
                // A scalar entry's value is the bare value its declared field carries,
                // not an entry object: render it back through the read face's inverse, so the
                // map Claude Code loads round-trips rather than an object it does not document.
                segment.insert(
                    registration.key.clone(),
                    crate::json_manifest::enablement_entry_value(&registration.fields, &field),
                );
            }
            crate::kind::EntryShape::Object => {
                let entry_value: JsonValue = registration
                    .fields
                    .iter()
                    .cloned()
                    .collect::<serde_json::Map<String, JsonValue>>()
                    .into();
                segment.insert(registration.key.clone(), entry_value);
            }
        }
    }

    for member in &payload.members {
        let facts =
            kind_facts
                .get(member.kind.as_str())
                .ok_or_else(|| DriftError::UnknownKind {
                    kind: member.kind.clone(),
                    member: member.name.clone(),
                })?;
        let source_path =
            member_projection_path(facts, &member.name, member.host.as_deref(), &kind_facts)?;
        // A **local**-locus member: the kind is declared and reviewed, its document is
        // not. Emit writes nothing at its path and
        // derives no row for it — no projection, no provenance/emit-hash rollup, no
        // nested-member or satisfies row — so the committed lock captures the committed
        // harness alone and its bytes stay layer-invariant by construction. The
        // declaration rows this pass skips are check's to derive at read time, off the
        // document itself, under the kind this row declares.
        //
        // The path is still booked owned. A local document is an author's own file, so
        // it must never read as an ownerless projection to the reap below — which it
        // otherwise would the first time a kind that used to be committed is declared
        // local, its prior rollup row still on the lock and its live document about to
        // be deleted as the orphan of a member emit no longer projects.
        if commitment_from_row(facts)? == Some(Commitment::Local) {
            local_paths.insert(to_lock_path(&source_path));
            continue;
        }
        // A container member of a represented manifest: its typed fields are the manifest's
        // opaque residue, and the whole file is regenerated by the write face below — never
        // projected as a frontmatter-and-body text artifact.
        if let Some(build) = manifests.get_mut(&source_path) {
            build.residue = member.fields.iter().cloned().collect();
            build.container = Some((member.kind.clone(), member.name.clone()));
            continue;
        }
        if let Content::Layout(layout) = content_from_row(facts)? {
            let edge_fields = layout_edge_fields(&payload.declarations.assembly, &member.kind)?;
            let derivation = derive_layout_rows(
                &layout,
                member,
                &source_path,
                &harness_root,
                &member_index,
                &edge_fields,
            )?;
            layout_rows.extend(derivation.nested);
            layout_import_rows.extend(derivation.imports);
            layout_satisfies.extend(derivation.satisfies);
            layout_paths.insert(to_lock_path(&source_path));
            continue;
        }
        let host = host_address(&member.kind, &member.name);
        let format = format_from_row(facts)?;
        // A read-face-only format has no writer for this member to reach, so it is refused
        // whole — body or not — ahead of the include resolution below, for the reason that
        // resolution's own refusal is sited there: nothing is fingerprinted against a
        // projection that was never going to happen.
        if let Some(Format::TomlDocument) = format {
            return Err(DriftError::FormatHasNoWriteFace {
                member: host,
                format: Format::TomlDocument.label().to_string(),
            }
            .into());
        }
        // A format that renders its fields alone has no slot to put a body in, so an
        // authored body is refused here rather than dropped at the write face. Sited above
        // the include resolution below: the words are already homeless whether or not they
        // splice, and refusing first keeps `emit` from fingerprinting a dependency whose
        // content could reach no artifact.
        if let Some(Format::JsonDocument) = format
            && !member.body.is_empty()
        {
            return Err(DriftError::BodyHasNoHome {
                member: host,
                format: Format::JsonDocument.label().to_string(),
            }
            .into());
        }
        // A composed-prose member whose body declares includes: resolve each against
        // disk (refusing before any byte is written when it dangles), splice its bytes
        // into the body at the matching slot, and fingerprint the dependency.
        let body = match includes_by_member.get(host.as_str()) {
            None => member.body.clone(),
            Some(includes) => {
                let mut contents = Vec::with_capacity(includes.len());
                for include in includes {
                    // The SDK resolves an include's target absolutely; the row is spelled
                    // against the harness root, so it resolves under a harness at any path.
                    let relative = harness_relative(&include.source_path, &harness_root);
                    let (row, bytes) = resolve_source_dependency(
                        &host,
                        &relative,
                        Path::new("."),
                        &harness_root,
                        &member_index,
                    )?;
                    contents.push(String::from_utf8(bytes).map_err(|_| {
                        DriftError::IncludeNotUtf8 {
                            member: host.clone(),
                            path: row.source_path.clone(),
                        }
                    })?);
                    include_rows.push(row);
                }
                splice_includes(&host, &member.body, &contents)?
            }
        };
        projections.push(Projection {
            kind: member.kind.clone(),
            name: member.name.clone(),
            source_path,
            format,
            fields: member.fields.clone(),
            body,
        });
    }

    // The harness-level settings residue this pass folds into its manifest: each key is an
    // opaque top-level entry of the manifest it names (Claude Code's settings.json), the same
    // residue slot a container member's fields fill. A key whose manifest names no in-play
    // kind cannot be placed, and one colliding with a differing value already present would
    // shed a value — each refused loud, never dropped (invariant 6).
    for row in &payload.declarations.settings {
        let path = manifest_path_for(&row.manifest, &kind_facts).ok_or_else(|| {
            DriftError::UnplaceableSettings {
                manifest: row.manifest.clone(),
                key: row.key.clone(),
            }
        })?;
        let residue = &mut manifests.entry(path).or_default().residue;
        match residue.get(&row.key) {
            Some(existing) if existing != &row.value => {
                return Err(DriftError::SettingsResidueCollision {
                    manifest: row.manifest.clone(),
                    key: row.key.clone(),
                }
                .into());
            }
            _ => {
                residue.insert(row.key.clone(), row.value.clone());
            }
        }
    }

    // Total runs in reverse too: a member the prior lock knew and the current
    // payload no longer owns leaves its projection stranded on disk unless emit
    // reaps it. The owned set the prior rows are diffed against is complete now —
    // projections, layout sources, and represented manifests — so the reap wave
    // is decided here, before a byte is written, and the cliff refusal never has
    // to undo a deletion.
    let mut owned_paths: BTreeSet<String> = projections
        .iter()
        .map(|projection| to_lock_path(&projection.source_path))
        .collect();
    // A layout document is a source — never reaped even when no rollup row projects it.
    owned_paths.extend(layout_paths);
    // A local document is the author's own, per-machine and uncommitted — owned by the
    // kind that governs it, never a projection this pass could have written and so never
    // one it may reap.
    owned_paths.extend(local_paths);
    // A represented manifest is emit-owned by its path even when no container member gives
    // it a rollup row, so a re-emit never reaps the file it just wrote.
    owned_paths.extend(manifests.keys().map(|path| to_lock_path(path)));

    // Read and parse the lock document once for reuse across the reap-diff and
    // layer-drop checks below — whole-input work hoists parsing per run, never
    // recomputed per phase (engineering.md, "Cost scale is hoisted").
    let lock_doc = read_lock_document_for_emit(workspace_dir);

    // Classify every prior projection the payload no longer owns without touching
    // disk. Both sides normalized: `owned_paths` came through `to_lock_path`, and
    // an older lock's raw row spelling gets the same canonicalization here, so a
    // `./`-prefixed row still joins its live projection.
    let mut orphans: Vec<(ProvenanceRow, PathBuf, EmitOutcome)> = Vec::new();
    let mut any_survivor = false;
    for row in read_prior_provenance_from_doc(&lock_doc) {
        if owned_paths.contains(&normalize_lock_path(&row.source_path)) {
            any_survivor = true;
            continue;
        }
        // The row is harness-relative, so the file it names is under the root this emit
        // targets — never under whatever cwd the emit happens to run from.
        let disk_path = harness_root.join(&row.source_path);
        if let Some(outcome) = classify_orphan(&disk_path, &row.emit_hash)? {
            orphans.push((row, disk_path, outcome));
        }
    }

    // The cliff (decision 0024): a reap wave that would delete every live
    // projection while emitting nothing in its place refuses, unless the author
    // spells the teardown. A genuine single-orphan reap keeps a survivor, so it
    // never trips this — the guard fires only when the whole prior tree reads
    // ownerless at once, never on a spelling mismatch the normalized join already
    // heals.
    let reaping = orphans
        .iter()
        .filter(|(_, _, outcome)| *outcome == EmitOutcome::Reaped)
        .count();
    if !any_survivor && reaping >= 2 && !options.teardown {
        return Err(DriftError::TotalReapWave { count: reaping }.into());
    }

    // The second cliff (decision 0024): a re-read that mines zero members for a
    // whole layer the committed lock still declares — every embedded member of one
    // host gone at once — refuses, unless the author spells the teardown. The
    // derived side is the payload's own nested-member declarations plus this pass's
    // layout derivations; the prior side is the committed lock's declared
    // collections, grouped by host. A host the lock still carries but this emit
    // derives nothing for is a dropped layer. The disappearance is loud because the
    // lock declares it, never because a projection's prose was scanned (invariant 1),
    // and a partial loss — a host that keeps at least one derived member — never
    // trips it.
    if !options.teardown {
        let mut derived_hosts: BTreeSet<&str> = payload
            .declarations
            .nested_members
            .iter()
            .map(|row| row.host.as_str())
            .collect();
        derived_hosts.extend(layout_rows.iter().map(|row| row.host.as_str()));

        let mut prior_by_host: BTreeMap<String, usize> = BTreeMap::new();
        if let Ok(declarations) = declarations_from_doc(&lock_doc) {
            for row in declarations.nested_members {
                *prior_by_host.entry(row.host).or_default() += 1;
            }
        }
        if let Some((host, count)) = prior_by_host
            .into_iter()
            .find(|(host, _)| !derived_hosts.contains(host.as_str()))
        {
            return Err(DriftError::LayerDropped { host, count }.into());
        }
    }

    // The reap doctrine carried down to a manifest's segments: a represented manifest is
    // rewritten whole, so a member the on-disk file carried and the payload no longer
    // declares would vanish with no finding. The diff runs before any byte is written, so
    // the segment cliff refuses a total collection drop without undoing a write.
    let (segment_reaps, cached_manifest_raws) =
        manifest_segment_reaps(&manifests, &kind_facts, &harness_root, options.teardown)?;

    let mut entries = Vec::with_capacity(projections.len() + orphans.len() + segment_reaps.len());
    let mut rollups: BTreeMap<String, Vec<RollupEntry>> = BTreeMap::new();
    for projection in &projections {
        let (entry, hash) = emit_one(projection, &harness_root, options.dry_run)?;
        rollups
            .entry(projection.kind.clone())
            .or_default()
            .push(RollupEntry {
                name: projection.name.clone(),
                source_path: to_lock_path(&projection.source_path),
                source_hash: hash.clone(),
                emit_hash: hash,
            });
        entries.push(entry);
    }

    // Each represented manifest is regenerated whole through the canonical write face and
    // written like any other projection — its container member's rollup row carries the
    // fingerprint drift compares.
    for (path, build) in &manifests {
        let cached_raw = cached_manifest_raws.get(path).map(|s| s.as_str());
        let (entry, hash) = emit_manifest(path, &harness_root, build, options.dry_run, cached_raw)?;
        if let Some((kind, name)) = &build.container {
            rollups.entry(kind.clone()).or_default().push(RollupEntry {
                name: name.clone(),
                source_path: to_lock_path(path),
                source_hash: hash.clone(),
                emit_hash: hash,
            });
        }
        entries.push(entry);
    }

    // The manifest segment reaps diffed before the writes above — each dropped discovered
    // member named in the ledger beside the whole-file reaps.
    entries.extend(segment_reaps);

    // Reap the byte-faithful orphans classified above — the deletion deferred past
    // the cliff check so a refused wave never removed a file. A drifted orphan is
    // left on disk and only reported: deleting hand-touched bytes is never the safe
    // default. Under `--dry-run` nothing is removed; the outcome stands regardless.
    for (row, disk_path, outcome) in orphans {
        if outcome == EmitOutcome::Reaped && !options.dry_run {
            fs::remove_file(&disk_path).map_err(|source| DriftError::Remove {
                path: disk_path.clone(),
                source,
            })?;
        }
        entries.push(EmitEntry {
            kind: row.kind,
            name: row.name,
            source_path: disk_path,
            outcome,
        });
    }

    if !options.dry_run {
        // The lock carries the program's declaration rows plus the ones emit derived
        // from layout sources this same pass — collection members merged into the
        // `nested_member` family and edge-slot fills into the `satisfies` family, with
        // the layout imports' content-dependency fingerprints alongside.
        let mut declarations = payload.declarations.clone();
        declarations.nested_members.extend(layout_rows);
        declarations.satisfies.extend(layout_satisfies);
        stamp_clause_labels(&mut declarations);
        write_rollup(
            workspace_dir,
            &rollups,
            &declarations,
            &layout_import_rows,
            &include_rows,
        )?;
    }

    Ok(EmitReport { entries })
}

/// Write every clause row's [`label`](ClauseRow::label) — its address — over the whole
/// declaration set, kinds' own clauses and requirements' nested ones alike. The single
/// write: the seam ships no label, so an authored one cannot disagree with the emitted
/// one, and a re-emit of the same program lands the same labels.
///
/// A requirement's nested row names no kind of its own, so its owner comes from the
/// requirement it hangs off — the one place that name is in scope.
fn stamp_clause_labels(declarations: &mut Declarations) {
    for row in &mut declarations.clauses {
        let owner = row.kind.clone();
        stamp_clause_label(row, owner.as_deref());
    }
    for requirement in &mut declarations.requirements {
        let owner = crate::contract::requirement_owner(&requirement.name);
        for row in &mut requirement.clauses {
            stamp_clause_label(row, Some(&owner));
        }
    }
}

/// Write one clause row's address, derived from the row's own identity columns.
fn stamp_clause_label(row: &mut ClauseRow, owner: Option<&str>) {
    row.label = Some(crate::contract::clause_label(
        owner,
        &row.predicate,
        row.field.as_deref(),
    ));
    // Recursively stamp labels on nested body clauses in a `when` clause.
    if let Some(body) = &mut row.body {
        for nested in body {
            stamp_clause_label(nested, None);
        }
    }
}

/// One represented manifest under construction during [`emit`]: its declared collection
/// segments (collection key → entry key → the entry's JSON value) and, when a container
/// member projects to the manifest's path, that member's opaque residue plus its
/// `kind:name` identity for the rollup row. `BTreeMap`s throughout, so a re-emit lands the
/// segments and residue in the same sorted order — the double-emit byte-stability the write
/// face rests on.
#[derive(Default)]
struct ManifestBuild {
    /// Collection key (`hooks`, `mcpServers`) → its entries (entry key → the JSON value).
    segments: BTreeMap<String, BTreeMap<String, JsonValue>>,
    /// The container member's opaque field residue — every top-level key no collection owns.
    residue: BTreeMap<String, JsonValue>,
    /// The container member's `(kind, name)`, when one projects to the manifest's path.
    container: Option<(String, String)>,
}

/// The harness-relative path a manifest kind's host file lives at — its `governs` locus. A
/// manifest kind's glob is a concrete filename (`settings.json`, `.mcp.json`), so this is
/// the file its registration members surface inside and the path the canonical write face
/// regenerates. `None` for a kind governing no locus at all — a nested file kind has no
/// host file to register inside.
fn manifest_target_path(facts: &KindFactRow) -> Option<PathBuf> {
    let root = facts.governs_root.as_deref()?;
    let glob = facts.governs_glob.as_deref()?;
    Some(join_locus(root, glob))
}

/// The harness-relative path the manifest named `manifest` lives at, resolved through any
/// in-play kind that declares it as its collection address's host — the same `governs`
/// locus [`manifest_target_path`] gives a registration kind. `None` when no in-play kind
/// declares the manifest, so its residue has nowhere to land.
fn manifest_path_for(manifest: &str, kind_facts: &BTreeMap<&str, &KindFactRow>) -> Option<PathBuf> {
    kind_facts
        .values()
        .find(|facts| {
            facts
                .collection_address
                .as_ref()
                .is_some_and(|address| address.manifest == manifest)
        })
        .and_then(|facts| manifest_target_path(facts))
}

/// The top-level manifest collection key a registration's key-path label names — the
/// segment its entries land in. `hooks.<Event>` keys under `hooks`, `mcpServers.*` under
/// `mcpServers`: the collection is the label's head, before the first `.`.
fn collection_key_of(key_path: &str) -> String {
    key_path.split('.').next().unwrap_or(key_path).to_string()
}

/// Regenerate one represented manifest whole through the canonical write face and write it
/// like any other projection — its declared collection segments in sorted order, then the
/// container's opaque residue. Returns the [`EmitEntry`] (Emitted vs the idempotent
/// Unchanged) and the SHA-256 of the bytes now on disk. A pure function of the build, so a
/// double-emit reproduces every byte; nothing is written under `dry_run`. An ownerless
/// manifest (no container member) is labelled by its filename under a `manifest` kind.
///
/// When `cached_raw` is `Some`, uses the pre-read file contents instead of reading again,
/// hoisting the read cost outside this function's loop. When `None`, reads the file directly.
fn emit_manifest(
    locus: &Path,
    harness_root: &Path,
    build: &ManifestBuild,
    dry_run: bool,
    cached_raw: Option<&str>,
) -> Result<(EmitEntry, String), DriftError> {
    let path = &harness_root.join(locus);
    let segments: Vec<crate::json_manifest::CollectionSegment> = build
        .segments
        .iter()
        .map(
            |(collection_key, entries)| crate::json_manifest::CollectionSegment {
                collection_key: collection_key.clone(),
                entries: entries.clone(),
            },
        )
        .collect();
    let desired = crate::json_manifest::write_manifest(&segments, &build.residue);

    let (kind, name) = build.container.clone().unwrap_or_else(|| {
        (
            "manifest".to_string(),
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default(),
        )
    });
    let row = |outcome| EmitEntry {
        kind: kind.clone(),
        name: name.clone(),
        source_path: path.to_path_buf(),
        outcome,
    };

    let current = if let Some(raw) = cached_raw {
        Some(raw.as_bytes().to_vec())
    } else {
        match fs::read(path) {
            Ok(bytes) => Some(bytes),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(source) => {
                return Err(DriftError::Read {
                    path: path.to_path_buf(),
                    source,
                });
            }
        }
    };
    let hash = sha256_hex(desired.as_bytes());
    if current.as_deref() == Some(desired.as_bytes()) {
        return Ok((row(EmitOutcome::Unchanged), hash));
    }
    if !dry_run {
        write_placement(path, &desired)?;
    }
    Ok((row(EmitOutcome::Emitted), hash))
}

/// The manifest segment reaps this emit performs: for each represented manifest, every
/// member the on-disk file discovers at a declared collection address — and every opaque
/// residue key it carries — that the payload no longer declares. A represented manifest is
/// regenerated whole through the write face, so an undeclared hook, installed plugin, or
/// residue key would vanish with no finding; this diffs the current file's discovered
/// members and residue keys against the build's `segments` and `residue` and names each
/// dropped one in the reap ledger — the whole-file reap doctrine carried down to a
/// manifest's segments.
///
/// The current file is parsed for **drift detection alone**, never a projection read for
/// meaning: an absent or unreadable file discovers nothing, and the reap is decided here,
/// before a byte is written, so the segment cliff never has to undo a write.
///
/// Returns both the reap entries and a map of manifest paths to their raw file contents
/// (for successful reads), hoisting the read cost outside the emit loop — each manifest
/// file is read exactly once and shared with [`emit_manifest`].
///
/// # Errors
/// Returns [`DriftError::SegmentReapWave`] when the drop is total — every discovered member
/// of one collection gone, the payload declaring none in its place — unless `teardown` is
/// set. Propagates a [`LockRowError`] when a kind fact's collection address carries a
/// key-path label outside the closed vocabulary.
fn manifest_segment_reaps(
    manifests: &BTreeMap<PathBuf, ManifestBuild>,
    kind_facts: &BTreeMap<&str, &KindFactRow>,
    harness_root: &Path,
    teardown: bool,
) -> Result<(Vec<EmitEntry>, BTreeMap<PathBuf, String>), DriftError> {
    let mut reaps = Vec::new();
    let mut cached_raws: BTreeMap<PathBuf, String> = BTreeMap::new();
    for (path, build) in manifests {
        // Every collection address that targets this manifest file — the kinds whose
        // registration members surface inside it (a hook and an installed plugin both key
        // into one settings.json). A collection with no address here reads as one opaque
        // top-level key, so an undeclared collection still surfaces as a residue-key drop,
        // never silently.
        let mut addresses = Vec::new();
        for facts in kind_facts.values() {
            if manifest_target_path(facts).as_deref() == Some(path.as_path())
                && let Some(address) = collection_address_from_row(facts)?
            {
                addresses.push(address);
            }
        }
        let address_refs: Vec<&CollectionAddress> = addresses.iter().collect();

        // Read the current manifest for drift detection only. An absent or unreadable file
        // discovers nothing; a malformed one carries no members this pass can honor — either
        // way the whole-file rewrite below stands, and nothing is falsely reaped.
        let disk_path = harness_root.join(path);
        let Ok(raw) = fs::read_to_string(&disk_path) else {
            continue;
        };
        increment_manifest_reads();
        let Ok(current) = crate::json_manifest::Manifest::parse(&disk_path, &raw, &address_refs)
        else {
            continue;
        };

        // Cache the raw file contents for reuse in emit_manifest, hoisting the read cost
        // outside the emit loop.
        cached_raws.insert(path.clone(), raw);

        let mut discovered: BTreeMap<&str, usize> = BTreeMap::new();
        for member in &current.members {
            *discovered.entry(member.collection.as_str()).or_default() += 1;
            let declared = build
                .segments
                .get(&member.collection)
                .is_some_and(|entries| entries.contains_key(&member.key));
            if !declared {
                reaps.push(EmitEntry {
                    kind: member.collection.clone(),
                    name: member.key.clone(),
                    source_path: disk_path.clone(),
                    outcome: EmitOutcome::MemberReaped,
                });
            }
        }
        // A residue key drop is keyed by the manifest's filename — the same label an
        // ownerless manifest wears in the ledger.
        let manifest_label = disk_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| to_lock_path(path));
        for key in current.opaque_fields.keys() {
            if !build.residue.contains_key(key) {
                reaps.push(EmitEntry {
                    kind: manifest_label.clone(),
                    name: key.clone(),
                    source_path: disk_path.clone(),
                    outcome: EmitOutcome::MemberReaped,
                });
            }
        }

        // The segment cliff: a collection whose every discovered member is dropped, the
        // payload declaring none in its place, is a total teardown of that segment — refused
        // unless the author spells it, the whole-file cliff's segment-level peer. A residue
        // key is no collection, so its drop is a ledger finding but never trips this.
        if !teardown {
            for (collection, count) in &discovered {
                let survives = build
                    .segments
                    .get(*collection)
                    .is_some_and(|entries| !entries.is_empty());
                if !survives {
                    return Err(DriftError::SegmentReapWave {
                        manifest: to_lock_path(path),
                        collection: (*collection).to_string(),
                        count: *count,
                    });
                }
            }
        }
    }
    Ok((reaps, cached_raws))
}

/// What emit derives from one layout source in a single read: its member collections
/// as `nested_member` declaration rows, its prose imports as content-dependency
/// [`LayoutImportRow`]s the lock fingerprints, and its `satisfies` edge slot as
/// [`SatisfiesRow`] fill edges. All fall out of the one document read, so they travel
/// together rather than forcing a second pass over the same source.
struct LayoutDerivation {
    /// The collection members, lowered into `nested_member` rows.
    nested: Vec<NestedMemberRow>,
    /// The prose imports, resolved and fingerprinted.
    imports: Vec<LayoutImportRow>,
    /// The `satisfies` edge slot's entries, lowered into `satisfies` fill-edge rows —
    /// the layout host's own fill claims, keyed by its member name exactly as a
    /// file-content member's SDK-emitted rows are.
    satisfies: Vec<SatisfiesRow>,
}

/// The edge-field slots a layout kind's document reads as **addresses** rather than a
/// verbatim span: the kind's declared relationship fields — its assembly `edge` facts
/// whose `from` is the kind — plus the framework `satisfies` key every kind carries.
///
/// A custom kind's relationships live only in the lock's assembly facts, never on its
/// kind-fact row, so both the emit read (here) and the gate read (`resolve_kind_units`)
/// range over this one set — a relationship section never parses as a verbatim field on
/// one path and as addresses on the other.
///
/// # Errors
///
/// Returns a [`LockRowError`] when a present `edge` fact for this kind omits its required
/// `field` column — a corrupt assembly row surfaced loud, never a silently dropped slot.
pub fn layout_edge_fields(
    assembly: &[AssemblyFactRow],
    kind: &str,
) -> Result<BTreeSet<String>, LockRowError> {
    let mut slots = BTreeSet::new();
    for fact in assembly
        .iter()
        .filter(|fact| fact.fact == "edge" && fact.from.as_deref() == Some(kind))
    {
        let field = fact
            .field
            .clone()
            .ok_or_else(|| LockRowError::MissingColumn {
                family: "assembly".to_string(),
                column: "field".to_string(),
            })?;
        slots.insert(field);
    }
    slots.insert(crate::kind::SATISFIES_EDGE_FIELD.to_string());
    Ok(slots)
}

/// Read one layout member's document off disk and lower it into declaration rows — the
/// rows emit derives from a layout source (`pipeline.md`, "The lock"). The host address
/// is the layout member's own `kind:name`; each collection member becomes one embedded
/// member of its declared child kind, keyed by its slugged-heading (or explicit-key)
/// identity, carrying its own sub-heading spans as leaves. Each prose region declared as
/// an import resolves against raw disk to the file's contents ([`resolve_source_dependency`]),
/// fingerprinted so a moved target is drift; a dangling target refuses loud before a byte
/// is written.
///
/// # Errors
/// Returns a [`DriftError`] if the document cannot be read, a dangling import is found, or
/// a `LayoutError` (as a [`miette::Report`]) when the document does not fit its declared
/// layout.
fn derive_layout_rows(
    layout: &Layout,
    member: &PayloadMember,
    source_path: &Path,
    harness_root: &Path,
    member_index: &BTreeMap<PathBuf, String>,
    edge_fields: &BTreeSet<String>,
) -> miette::Result<LayoutDerivation> {
    let disk_path = harness_root.join(source_path);
    let host = host_address(&member.kind, &member.name);
    let document =
        read_layout_document(layout, &member.kind, &member.name, &disk_path, edge_fields)?;

    let mut imports = Vec::new();
    for region in &layout.regions {
        let LayoutRegion::Prose {
            import: Some(target),
        } = region
        else {
            continue;
        };
        let base_dir = source_path.parent().unwrap_or_else(|| Path::new("."));
        let (row, _bytes) =
            resolve_source_dependency(&host, target, base_dir, harness_root, member_index)?;
        imports.push(row);
    }

    Ok(LayoutDerivation {
        nested: document.nested,
        imports,
        satisfies: document.satisfies,
    })
}

/// The declaration rows one layout **document** yields, read off disk — the part of a
/// layout source's lowering that is the document's alone, with no dependency on how the
/// reader reached it.
///
/// Both faces of a layout document share it. `emit` lowers a committed layout host's
/// source into the rows it writes to the lock ([`derive_layout_rows`], which adds the
/// prose imports it also fingerprints); `check` derives a **local**-locus member's rows
/// here at read time, because a local locus's rows never enter the lock to be read back.
/// One reader means the two faces can never
/// disagree about what a document declares — the same posture `layout_edge_fields`
/// takes for the slots this read is handed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayoutDocumentRows {
    /// The collection members, lowered into `nested_member` rows.
    pub nested: Vec<NestedMemberRow>,
    /// The `satisfies` edge slot's entries, lowered into fill-edge rows.
    pub satisfies: Vec<SatisfiesRow>,
}

/// Read the layout document at `disk_path` and lower it into the rows it declares — the
/// member collections' embedded members and the `satisfies` edge slot's fill claims, each
/// keyed by the host's `kind:name` address.
///
/// # Errors
/// Returns a [`DriftError`] when the document cannot be read, or a `LayoutError` (as a
/// [`miette::Report`]) when it does not fit its declared layout.
pub fn read_layout_document(
    layout: &Layout,
    kind: &str,
    name: &str,
    disk_path: &Path,
    edge_fields: &BTreeSet<String>,
) -> miette::Result<LayoutDocumentRows> {
    let body = fs::read_to_string(disk_path).map_err(|source| DriftError::Read {
        path: disk_path.to_path_buf(),
        source,
    })?;
    let reading = layout.read(&body, disk_path, edge_fields)?;
    let host = host_address(kind, name);

    // A `satisfies` edge slot's entries are the host's own fill claims, keyed by its
    // own `kind:name` address (the label `resolve_kind_units` folds them back onto) — a
    // dangling one is the gate's existing `requirement.dangling` refusal to catch, never
    // a new one.
    let satisfies = reading
        .edges
        .get(crate::kind::SATISFIES_EDGE_FIELD)
        .into_iter()
        .flatten()
        .map(|requirement| SatisfiesRow {
            member: host.clone(),
            requirement: requirement.clone(),
        })
        .collect();

    let nested = reading
        .members
        .into_iter()
        .map(|member| NestedMemberRow {
            host: host.clone(),
            kind: member.member_kind,
            key: member.key,
            leaves: member.leaves,
            collections: Vec::new(),
            // A member embedded in a layout document is read off its host's declared
            // layout — source, never projection — so no format rendered it: none could
            // have omitted an edge, and there is no rendered span to budget.
            placed_edges: None,
            rendered_lines: None,
            rendered_chars: None,
        })
        .collect();
    Ok(LayoutDocumentRows { nested, satisfies })
}

/// Resolve one prose reference — a layout region's `import` or a composed-prose
/// `include` — to its target file's contents, fingerprint it, and return the bytes.
///
/// `target` joins onto `base_dir` — both harness-relative: the referencing document's own
/// directory for a layout import, `.` for an include whose SDK-resolved absolute path was
/// relativized against the root ([`harness_relative`]). That resolved path is what the row
/// records and what the member index is keyed by; `harness_root` joins it back on to reach
/// **raw disk**, never the ignore-filtered discovery view. The two are split because the
/// row is committed and the read is not: a row spelling the emit's cwd resolves under no
/// other one.
///
/// A target absent from disk is a dangling reference, refused loud. When the resolved path
/// is a member's own projection, the edge names that member; a plain repository file
/// carries a content dependency but no member edge (an empty `target`). The returned bytes
/// are the same read the fingerprint hashes, so a splicing caller pulls exactly the bytes
/// it fingerprinted (one read, no time-of-check gap).
///
/// # Errors
/// Returns [`DriftError::DanglingImport`] when the target does not exist, or
/// [`DriftError::Read`] when it exists but cannot be read.
fn resolve_source_dependency(
    host: &str,
    target: &str,
    base_dir: &Path,
    harness_root: &Path,
    member_index: &BTreeMap<PathBuf, String>,
) -> Result<(LayoutImportRow, Vec<u8>), DriftError> {
    let resolved = crate::path::normalize_path(&base_dir.join(target));
    let disk_path = harness_root.join(&resolved);
    let bytes = match fs::read(&disk_path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(DriftError::DanglingImport {
                member: host.to_string(),
                import: target.to_string(),
                path: disk_path,
            });
        }
        Err(source) => {
            return Err(DriftError::Read {
                path: disk_path,
                source,
            });
        }
    };
    let row = LayoutImportRow {
        member: host.to_string(),
        target: member_index.get(&resolved).cloned().unwrap_or_default(),
        source_path: to_lock_path(&resolved),
        import_hash: sha256_hex(&bytes),
    };
    Ok((row, bytes))
}

/// Re-express an include's SDK-resolved absolute `target` as a path relative to
/// `harness_root` — the one home for that transform, and the only place a path enters
/// this pass already absolute. The result matches the member index and rides the lock in
/// the same harness-relative vocabulary a projection path is spelled in, so the committed
/// row resolves under the harness wherever it sits. A target outside the harness tree
/// keeps its absolute form, still readable, just unrooted (joining it back onto any root
/// is a no-op).
///
/// Resolves both paths canonically to handle symlink hopping consistently with how the
/// SDK resolves targets: when a symlink sits between the cwd and the harness, the two
/// sides' lexical absolutization can diverge, failing to strip. Canonicalization unifies
/// them before stripping, so the row is harness-relative regardless of symlinks in the cwd.
fn harness_relative(target: &str, harness_root: &Path) -> String {
    let target_path = PathBuf::from(target);
    // Try canonicalizing both sides to resolve symlinks consistently. The SDK canonicalizes
    // targets via fs::canonicalize (via import.meta.url resolution), so our root must too.
    // Fallback to pure lexical absolutization if either path doesn't exist yet.
    let target_abs = fs::canonicalize(&target_path)
        .or_else(|_| std::path::absolute(&target_path))
        .unwrap_or(target_path);
    let root_abs = fs::canonicalize(harness_root)
        .or_else(|_| std::path::absolute(harness_root))
        .unwrap_or_else(|_| harness_root.to_path_buf());
    match target_abs.strip_prefix(&root_abs) {
        Ok(relative) => to_lock_path(relative),
        Err(_) => to_lock_path(&target_abs),
    }
}

/// Splice each resolved include's `contents` into `body` at its include slot, in order
/// — the body carries one [`INCLUDE_SLOT`] per declared include (SDK-planted, authored
/// order), so the k-th slot becomes the k-th target's text. Splitting on the slot byte
/// yields `contents.len() + 1` chunks when the counts agree.
///
/// # Errors
/// Returns [`DriftError::IncludeArity`] when the body's slot count disagrees with the
/// number of declared includes — a malformed seam.
fn splice_includes(host: &str, body: &str, contents: &[String]) -> Result<String, DriftError> {
    let chunks: Vec<&str> = body.split(INCLUDE_SLOT).collect();
    if chunks.len() != contents.len() + 1 {
        return Err(DriftError::IncludeArity {
            member: host.to_string(),
            declared: contents.len(),
            slots: chunks.len() - 1,
        });
    }
    let mut out = String::from(chunks[0]);
    for (content, chunk) in contents.iter().zip(&chunks[1..]) {
        out.push_str(content);
        out.push_str(chunk);
    }
    Ok(out)
}

/// The projection-path → `kind:name` index every member contributes, keyed by the
/// lexically-normalized harness-relative path so a resolved layout import joins it the way
/// [`resolve_source_dependency`] resolves its target. A member whose kind the payload carries
/// no fact for is skipped — the [`emit`] loop reports that fault where it dispatches.
///
/// # Errors
/// Returns the refusals [`member_projection_path`] raises when a member maps to no one
/// projection path — the same ones [`emit`]'s member loop raises, surfaced here so the
/// index never carries a nonsense path.
fn member_path_index(
    members: &[PayloadMember],
    kind_facts: &BTreeMap<&str, &KindFactRow>,
) -> Result<BTreeMap<PathBuf, String>, DriftError> {
    let mut index = BTreeMap::new();
    for member in members {
        let Some(facts) = kind_facts.get(member.kind.as_str()) else {
            continue;
        };
        let path = crate::path::normalize_path(&member_projection_path(
            facts,
            &member.name,
            member.host.as_deref(),
            kind_facts,
        )?);
        index.insert(path, host_address(&member.kind, &member.name));
    }
    Ok(index)
}

/// One raw row from the lock's declaration table — all fields as Options, since
/// [`read_prior_provenance`], [`config_stale`], and [`emit_owned_targets`] each
/// require different subsets of the columns (name+source_path+emit_hash,
/// name+source_path+emit_hash, and name+source_path respectively). A single
/// `walk_lock_rows` does the file read and lock parse once; each consumer
/// filter_maps over rows to extract its required columns.
struct RawLockRow {
    /// The member's kind (bare name — `"skill"`, `"rule"`, …).
    kind: String,
    /// The member's name, absent if the row's `name` column is missing or malformed.
    name: Option<String>,
    /// The projection's on-disk path as the lock recorded it, absent if missing or malformed.
    source_path: Option<String>,
    /// The projection's last-emitted fingerprint, absent if the row's `emit_hash` column is missing or malformed.
    emit_hash: Option<String>,
}

/// Extract lock rows from an already-parsed lock document, walking every `[[<kind>]]`
/// array-of-tables entry — returns all columns (as Options) for each row.
fn walk_lock_rows_from_doc(doc: &DocumentMut) -> Vec<RawLockRow> {
    let mut rows = Vec::new();
    for (kind, item) in doc.as_table().iter() {
        let Some(table_rows) = item.as_array_of_tables() else {
            continue;
        };
        for row in table_rows.iter() {
            rows.push(RawLockRow {
                kind: kind.to_string(),
                name: row
                    .get("name")
                    .and_then(Item::as_str)
                    .map(|s| s.to_string()),
                source_path: row
                    .get("source_path")
                    .and_then(Item::as_str)
                    .map(|s| s.to_string()),
                emit_hash: row
                    .get("emit_hash")
                    .and_then(Item::as_str)
                    .map(|s| s.to_string()),
            });
        }
    }
    rows
}

/// Walk the committed lock's declaration rows once, reading the lock file and
/// parsing every `[[<kind>]]` array-of-tables entry — returns all columns
/// (as Options) for each row. A missing or malformed lock yields no rows.
fn walk_lock_rows(workspace_dir: &Path) -> Vec<RawLockRow> {
    let doc = read_lock_document_for_emit(workspace_dir);
    walk_lock_rows_from_doc(&doc)
}

/// One provenance row read back off a workspace's prior `lock.toml` — the same
/// `name`/`source_path`/`emit_hash` columns [`config_stale`] and
/// [`emit_owned_targets`] already read, kept here as owned scalars since this
/// reader's rows outlive the parsed document (they cross into the next lock's
/// rewrite).
struct ProvenanceRow {
    /// The member's kind (bare name — `"skill"`, `"rule"`, …).
    kind: String,
    /// The member's name.
    name: String,
    /// The projection's on-disk path, as the lock recorded it.
    source_path: String,
    /// The projection's last-emitted fingerprint.
    emit_hash: String,
}

/// Read and parse the lock document once for emit, returning it for reuse across emit's phases.
/// A missing lock or read/parse failure yields an empty document that will produce
/// empty results when queried, matching the tolerant-read behavior of the helpers
/// that consume lock data.
fn read_lock_document_for_emit(workspace_dir: &Path) -> DocumentMut {
    let path = workspace_dir.join(crate::LOCK_FILENAME);
    match fs::read_to_string(&path) {
        Ok(text) => {
            increment_lock_reads();
            match text.parse::<DocumentMut>() {
                Ok(doc) => {
                    increment_lock_parses();
                    doc
                }
                Err(_) => DocumentMut::new(),
            }
        }
        Err(_) => DocumentMut::new(),
    }
}

/// Every provenance row the lock document carries, across every kind (built-in and
/// custom) — the anchor [`emit`]'s reap step diffs the current payload's owned paths
/// against to find a lock-known projection with no current owner. A row missing a
/// required column yields no rows — the same tolerant-read absence
/// [`config_stale`]/[`emit_owned_targets`] take: nothing to compare against forges
/// no reap, no drift finding.
fn read_prior_provenance_from_doc(doc: &DocumentMut) -> Vec<ProvenanceRow> {
    walk_lock_rows_from_doc(doc)
        .into_iter()
        .filter_map(|raw| {
            let (Some(name), Some(source_path), Some(emit_hash)) =
                (raw.name, raw.source_path, raw.emit_hash)
            else {
                return None;
            };
            Some(ProvenanceRow {
                kind: raw.kind,
                name,
                source_path,
                emit_hash,
            })
        })
        .collect()
}

/// Classify one lock-known projection whose owning member is gone, mutating no
/// disk state: the bytes at `disk_path` — the row's harness-relative path already
/// joined onto the root this emit targets — are hashed against the row's recorded
/// `emit_hash` to tell `Reaped` (byte-faithful — temper wrote every byte, so
/// deleting it loses nothing authored) from `OrphanDrift` (hand-touched — left
/// in place and reported, never silently deleted). A file already absent yields
/// `None`: there is nothing left to act on. The reap wave is decided over this
/// classification before any deletion runs, so the cliff refusal never has to
/// undo one.
fn classify_orphan(disk_path: &Path, emit_hash: &str) -> Result<Option<EmitOutcome>, DriftError> {
    match fs::read(disk_path) {
        Ok(bytes) if sha256_hex(&canonicalize_eol(&bytes)) == emit_hash => {
            Ok(Some(EmitOutcome::Reaped))
        }
        Ok(_) => Ok(Some(EmitOutcome::OrphanDrift)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(DriftError::Read {
            path: disk_path.to_path_buf(),
            source,
        }),
    }
}

/// Normalize line endings to LF: a CRLF pair collapses to one `\n`, and a lone
/// CR (old Mac style) becomes `\n` too — projections are written LF uniformly
/// regardless of the source's own convention.
fn normalize_lf(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            out.push('\n');
        } else {
            out.push(c);
        }
    }
    out
}

/// Re-emit one projection whole, returning its [`EmitEntry`] and the SHA-256 of the
/// bytes now on disk (or that would be, under `--dry-run`) — the fresh rollup row's
/// `source_hash`/`emit_hash`, always equal for a payload-compiled member (there is no
/// separate authored-source file to diverge from; the resolved payload IS the source).
///
/// The projection is regenerated from the payload — never merged against on-disk
/// bytes — so a hand-edited projection is simply overwritten: a direct edit to
/// emitted output is drift routed to the source (`config.stale`/the guard surface
/// it), not a mergeable conflict. The on-disk read decides only `Emitted` vs the
/// idempotent `Unchanged`.
fn emit_one(
    projection: &Projection,
    harness_root: &Path,
    dry_run: bool,
) -> Result<(EmitEntry, String), DriftError> {
    // The projection path is harness-relative (the lock's vocabulary); disk is reached
    // under the root this emit targets, and the report names the file it actually wrote.
    let disk_path = harness_root.join(&projection.source_path);
    let row = |outcome| EmitEntry {
        kind: projection.kind.clone(),
        name: projection.name.clone(),
        source_path: disk_path.clone(),
        outcome,
    };

    // Read the committed projection first — never to merge authored content, but to
    // tell `Emitted` from the idempotent no-op *and* to carry install's frontmatter
    // placements (the schema modeline, the managed-by note) through the whole-file
    // re-emit. Those metadata lines ride `install`, never `emit`, so a re-emit round-trips the ones
    // already on disk instead of clobbering them. An absent source carries no
    // placements and is not a conflict: emit writes it.
    let current = match fs::read(&disk_path) {
        Ok(bytes) => Some(bytes),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(source) => {
            return Err(DriftError::Read {
                path: disk_path.clone(),
                source,
            });
        }
    };
    let placements = current
        .as_deref()
        .map(|bytes| crate::placement::placement_lines(&String::from_utf8_lossy(bytes)))
        .unwrap_or_default();

    let render = || {
        project_bytes(
            projection.format,
            &projection.fields,
            &projection.body,
            &placements,
        )
        .map(|bytes| normalize_lf(&bytes))
        .ok_or_else(|| DriftError::FormatHasNoWriteFace {
            member: host_address(&projection.kind, &projection.name),
            format: projection
                .format
                .map(|format| format.label().to_string())
                .unwrap_or_default(),
        })
    };
    let desired = render()?;

    // Double-emit determinism: a second
    // projection over the same surface must be byte-identical. Nondeterministic
    // authoring (a timestamp, an unordered map surfacing into a field) is a loud
    // failure here, never a silent churn the next `emit` would rewrite.
    let second_pass = render()?;
    if second_pass != desired {
        return Err(DriftError::Nondeterministic {
            path: disk_path.clone(),
        });
    }

    let hash = sha256_hex(desired.as_bytes());
    if current.as_deref() == Some(desired.as_bytes()) {
        return Ok((row(EmitOutcome::Unchanged), hash));
    }

    if !dry_run {
        write_placement(&disk_path, &desired)?;
    }
    Ok((row(EmitOutcome::Emitted), hash))
}

/// Re-emit the desired projection deterministically through the canonical write face
/// `format` names — **the one write dispatch**, the read side's [`read_file_unit`] match
/// mirrored: a `json-document` kind renders its fields as the whole JSON artifact, every
/// other file kind renders a `---`-delimited frontmatter block over its body. Both
/// determinism passes route through here, so a per-call-site format match cannot exist to
/// disagree with this one.
///
/// The authored content is *generated*, not patched — a hand-edited
/// field is not preserved (that is drift, routed to the authored source). Install's
/// metadata comments are the one exception the caller feeds in: they ride `install`,
/// never `emit`, so emit round-trips the ones
/// already on disk rather than dropping them. A JSON document carries none by
/// construction — install places its metadata as a frontmatter comment or a markdown
/// banner, neither of which a JSON artifact's bytes can hold — so that face takes no
/// `placements`. It renders `fields` alone: a JSON document has no prose slot, so `body`
/// reaches this face empty or not at all — [`emit`]'s projection loop refuses a
/// `json-document` member carrying one ([`DriftError::BodyHasNoHome`]) rather than let
/// this arm drop it. An artifact with no fields (a rule that
/// carries no `paths`/unknown keys, a memory `CLAUDE.md`) projects to its body alone —
/// no frontmatter block, so install's metadata there is a block-level HTML-comment
/// banner heading the body, round-tripped the same way.
///
/// `None` when `format` names a **read face only** (`toml-document`): there is no write
/// face to render through, and inventing one here would be the silent degrade
/// [`DriftError::FormatHasNoWriteFace`] exists to refuse. The match over [`Format`] is
/// exhaustive so that a format joining the vocabulary must answer here rather than
/// inherit the frontmatter fall-through by default; each caller raises the refusal
/// naming its own subject.
#[must_use]
pub fn project_bytes(
    format: Option<Format>,
    fields: &[(String, JsonValue)],
    body: &str,
    placements: &[String],
) -> Option<String> {
    match format {
        Some(Format::JsonDocument) => {
            return Some(crate::json_manifest::write_document(
                &fields.iter().cloned().collect(),
            ));
        }
        Some(Format::TomlDocument) => return None,
        Some(Format::YamlFrontmatter) | None => {}
    }
    if fields.is_empty() {
        // A frontmatterless projection: install's banner, if any, heads the body with
        // one blank line between; otherwise the body alone.
        let mut out = String::new();
        for line in placements {
            out.push_str(line);
            out.push_str("\n\n");
        }
        out.push_str(body);
        return Some(out);
    }
    let mut frontmatter = String::new();
    for line in placements {
        frontmatter.push_str(line);
        frontmatter.push('\n');
    }
    for (key, value) in fields {
        frontmatter.push_str(&render_field(key, value));
    }
    Some(format!("---\n{frontmatter}---\n{body}"))
}

/// Render one frontmatter field as `key: <value>\n`. The value is emitted as
/// compact JSON, which is valid YAML flow — a double-quoted string, a bare number
/// or bool, a `[..]` sequence — so it round-trips back to the same JSON on the next
/// parse (keeping the re-emitted projection idempotent).
fn render_field(key: &str, value: &JsonValue) -> String {
    // Serializing a `serde_json::Value` is infallible in practice; fall back to a
    // null literal rather than panic on the unreachable error path.
    let rendered = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
    format!("{key}: {rendered}\n")
}

/// Render an emit report for the terminal: one `<outcome>  <kind>  <name>` line per
/// entry in the report's stable order, then a one-line tally.
#[must_use]
pub fn render_emit(report: &EmitReport) -> String {
    let mut out = String::new();
    let (mut emitted, mut unchanged, mut reaped, mut orphan_drift, mut member_reaped) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    for entry in &report.entries {
        match entry.outcome {
            EmitOutcome::Emitted => emitted += 1,
            EmitOutcome::Unchanged => unchanged += 1,
            EmitOutcome::Reaped => reaped += 1,
            EmitOutcome::OrphanDrift => orphan_drift += 1,
            EmitOutcome::MemberReaped => member_reaped += 1,
        }
        out.push_str(&format!(
            "{:<13}  {:<5}  {}\n",
            entry.outcome.label(),
            entry.kind,
            entry.name
        ));
    }
    out.push_str(&format!(
        "\n{emitted} emitted, {unchanged} unchanged, {reaped} reaped, {orphan_drift} orphan-drift, {member_reaped} member-reaped\n"
    ));
    out
}

// ---------------------------------------------------------------------------
// place — the whole-file direction
// ---------------------------------------------------------------------------

/// One placement's outcome from [`place`] — its own three-state merge, distinct from
/// [`EmitOutcome`]. A placement is merged into a file temper shares with the human, so
/// it keeps `Conflicted`; emit, which regenerates a projection whole, does not. The
/// two-projectors seam stays until `install` rides emit's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyOutcome {
    /// The placement was written (created or re-placed) to match `desired`, or would
    /// be under `--dry-run`.
    Applied,
    /// `desired` already sat on disk byte-for-byte; nothing to write.
    Unchanged,
    /// The placement drifted from its recorded baseline *and* differs from `desired`
    /// — a human changed it out from under temper, surfaced rather than clobbered.
    Conflicted,
}

impl ApplyOutcome {
    /// The lower-case label used in the rendered report and stable for tests.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            ApplyOutcome::Applied => "applied",
            ApplyOutcome::Unchanged => "unchanged",
            ApplyOutcome::Conflicted => "conflicted",
        }
    }
}

/// Project `desired` onto `path` under a three-state merge — the whole-file
/// placement direction, for artifacts temper *places* rather than emits.
/// It carries its own
/// [`ApplyOutcome`] and reuses [`DriftError`] so `install` builds on this write-back
/// direction; unlike [`emit`], which regenerates a projection whole, a placement
/// merges into a file it shares with the human, so it keeps conflict detection (the
/// two-projectors seam stays until `install` rides emit's projection).
///
/// The three states are the engine's own: **desired** (the caller's bytes),
/// **last-applied** (the fingerprint of the file as temper last wrote it, from
/// `last_applied`), and **real on-disk**. The merge:
///
/// - target **absent** ⇒ [`ApplyOutcome::Applied`] — the placement is *created*
///   (an `install` onto a harness that does not carry it yet, or re-placing one a
///   human deleted): a placement has no prior on-disk source to have been deleted,
///   so writing it is the whole point.
/// - real **equals** desired ⇒ [`ApplyOutcome::Unchanged`] (the idempotent no-op).
/// - real **differs**, and either no baseline is recorded (`last_applied` is
///   `None`) or real still hashes to it ⇒ [`ApplyOutcome::Applied`], desired
///   written.
/// - real **differs** and has drifted from a recorded baseline ⇒
///   [`ApplyOutcome::Conflicted`]: a human changed the placement out from under
///   temper, so the merge surfaces the choice and writes nothing.
///
/// A `None` `last_applied` is the *idempotent-placement* mode: when `desired` is a
/// pure function of the current file (temper's own gate wiring merged into it),
/// temper keeps no fingerprint of its own — re-running re-derives the invariant —
/// so a present-but-different file is a clean merge target, never a conflict. A
/// caller that records a fingerprint gets full conflict detection by passing
/// `Some`. Nothing is written under `dry_run`; the outcome is computed all the same.
pub fn place(
    path: &Path,
    desired: &str,
    last_applied: Option<&str>,
    dry_run: bool,
) -> Result<ApplyOutcome, DriftError> {
    let real = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            // Absent: create it (fresh install / re-place). There is nothing on disk
            // to conflict with, so the placement is always written.
            if !dry_run {
                write_placement(path, desired)?;
            }
            return Ok(ApplyOutcome::Applied);
        }
        Err(source) => {
            return Err(DriftError::Read {
                path: path.to_path_buf(),
                source,
            });
        }
    };

    if real == desired.as_bytes() {
        return Ok(ApplyOutcome::Unchanged);
    }

    // The file differs from desired. With no recorded baseline the merge trusts the
    // projection (an idempotent placement); with one, a drift away from it is a
    // human edit the merge must surface rather than clobber.
    let drifted_from_baseline =
        last_applied.is_some_and(|baseline| sha256_hex(&canonicalize_eol(&real)) != baseline);
    if drifted_from_baseline {
        return Ok(ApplyOutcome::Conflicted);
    }

    if !dry_run {
        write_placement(path, desired)?;
    }
    Ok(ApplyOutcome::Applied)
}

/// Write a placement's bytes to `path`, creating any missing parent directories.
/// Both failures surface as [`DriftError::Write`] so a placement that cannot be
/// written **errors loudly** rather than silently skipping.
fn write_placement(path: &Path, desired: &str) -> Result<(), DriftError> {
    crate::fs_util::write_creating_parents(path, desired.as_bytes()).map_err(|source| {
        DriftError::Write {
            path: path.to_path_buf(),
            source,
        }
    })
}

// ---------------------------------------------------------------------------
// config.stale — the freshness fact the gate reads
// ---------------------------------------------------------------------------

/// The diagnostic `rule` id every freshness finding reports under.
const CONFIG_STALE_RULE: &str = "config.stale";

/// The `config.stale` freshness findings for a surface `workspace_dir`:
/// a
/// committed projection whose bytes no longer match the emit fingerprint the lock
/// recorded — the authored source changed and `emit` has not run, or the emitted
/// output was hand-edited. One finding
/// per drifted row, pointing at the projection that moved.
///
/// **Advisory** (`warn`): under the default `warn` enforcement mode the guard warns-and-routes
/// rather than blocks, and temper fabricates no
/// hard gate the author did not declare — a stale projection is a
/// nudge to re-emit.
///
/// Read off `<workspace_dir>/lock.toml` — every `[[<kind>]]` row (built-in and custom):
/// each row's `source_path` is re-hashed and compared to its `emit_hash`. A row without
/// an `emit_hash` (a lock predating the fingerprint) or a `source_path` that cannot be
/// read is **skipped** — the safe direction, since
/// absent evidence must never *forge* a staleness finding (a removed source is the drift
/// engine's `removed` state, not this freshness fact). A missing or malformed lock
/// yields no findings for the same reason.
///
/// An in-place member carries **no lock row** (`install` writes no copy tree, no lock — the
/// landscape file is its own source), so it contributes no freshness fact here: an
/// in-place member cannot drift.
#[must_use]
pub fn config_stale(workspace_dir: &Path) -> Vec<crate::check::Diagnostic> {
    let doc = read_lock_document_for_emit(workspace_dir);
    config_stale_from_doc(&doc, workspace_dir)
}

/// Staleness findings from the given lock document without re-reading from disk.
/// Reuses a parsed lock document to avoid duplicate read+parse operations when gate()
/// already holds the document. Findings are harness-relative to `workspace_dir`.
pub fn config_stale_from_doc(
    doc: &DocumentMut,
    workspace_dir: &Path,
) -> Vec<crate::check::Diagnostic> {
    let mut findings = Vec::new();
    let harness_root = harness_root_of(workspace_dir);
    for raw in walk_lock_rows_from_doc(doc) {
        let (Some(name), Some(source_path), Some(emit_hash)) =
            (raw.name, raw.source_path, raw.emit_hash)
        else {
            continue;
        };
        // Only a present-and-differing projection is stale: a source that is gone
        // (or otherwise unreadable) is the `removed`/drift axis, never forged here.
        // The row is harness-relative, so it resolves under the harness this check
        // was aimed at, whatever the cwd.
        let Ok(bytes) = fs::read(harness_root.join(&source_path)) else {
            continue;
        };
        if sha256_hex(&canonicalize_eol(&bytes)) != emit_hash {
            findings.push(crate::check::Diagnostic::warn(
                CONFIG_STALE_RULE,
                &source_path,
                format!(
                    "committed projection `{source_path}` (member `{name}`) does not match the lock's emit fingerprint — the authored source changed and `emit` has not run, or the projection was hand-edited; re-emit to reconcile"
                ),
            ));
        }
    }
    findings
}

// ---------------------------------------------------------------------------
// prose source dependencies — the content a layout import or composed-prose include
// fingerprints (one shape, two families)
// ---------------------------------------------------------------------------

/// One prose source dependency the lock fingerprints — a layout region's `import` or a
/// composed-prose `include`, resolved and hashed at emit ([`resolve_source_dependency`]).
/// It rides the lock under its family's own `[[declaration.<family>]]` array, which the
/// reap/freshness readers (keyed on `name`/`emit_hash`) never see: a referenced target is
/// a *source* dependency, not an emit-owned projection, so it is fingerprinted for drift
/// yet never reaped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutImportRow {
    /// The referencing member's own `kind:name` address.
    pub member: String,
    /// The resolved target member's `kind:name` address, or empty when the reference
    /// resolves to a plain repository file that is not a member (a content dependency
    /// with no member edge).
    pub target: String,
    /// The target's on-disk path, lock-normalized — the byte source the fingerprint
    /// hashes and drift re-hashes.
    pub source_path: String,
    /// The SHA-256 of the target's bytes at emit — a moved target re-hashes differently
    /// and surfaces as drift.
    pub import_hash: String,
}

/// The lock family key layout imports fingerprint under.
const LAYOUT_IMPORT_FAMILY: &str = "layout_import";
/// The lock family key composed-prose includes fingerprint under.
const INCLUDE_FAMILY: &str = "include";

/// Write a source-dependency `family` into a lock document's `[declaration]` table as
/// `[[declaration.<family>]]` — one table per resolved reference, in emit order. Called
/// after [`Declarations::write_into`] so the `[declaration]` table already exists for a
/// program with any declaration at all; the table is created when absent so a
/// dependency-only lock still round-trips. An empty set writes nothing (an empty
/// `ArrayOfTables` vanishes on the round-trip, the same discipline every declaration
/// family keeps).
pub(crate) fn write_source_deps(doc: &mut DocumentMut, family: &str, rows: &[LayoutImportRow]) {
    if rows.is_empty() {
        return;
    }
    let decl = doc
        .as_table_mut()
        .entry("declaration")
        .or_insert_with(|| Item::Table(Table::new()));
    let Some(table) = decl.as_table_mut() else {
        return;
    };
    let mut array = ArrayOfTables::new();
    for row in rows {
        let mut entry = Table::new();
        entry["member"] = value(row.member.clone());
        if !row.target.is_empty() {
            entry["target"] = value(row.target.clone());
        }
        entry["source_path"] = value(row.source_path.clone());
        entry["import_hash"] = value(row.import_hash.clone());
        array.push(entry);
    }
    table.insert(family, Item::ArrayOfTables(array));
}

/// Lift one source-dependency row off its `[[declaration.<family>]]` table — the
/// `member`/`source_path`/`import_hash` columns required, `target` optional.
fn source_dep_row(row: &Table) -> Result<LayoutImportRow, RowError> {
    Ok(LayoutImportRow {
        member: req_str(row, "member")?,
        target: opt_str(row, "target")?.unwrap_or_default(),
        source_path: req_str(row, "source_path")?,
        import_hash: req_str(row, "import_hash")?,
    })
}

/// Extract every source-dependency row under `family_key` from an already-parsed lock
/// document — the fingerprinted content dependencies emit wrote, read back for the drift
/// comparison and the reference-edge lift. A missing or malformed row is surfaced loud.
///
/// # Errors
///
/// Returns a [`DriftError::LockRow`] if a present dependency row is malformed.
pub(crate) fn source_deps_from_doc(
    doc: &DocumentMut,
    family_key: &str,
) -> Result<Vec<LayoutImportRow>, DriftError> {
    let Some(table) = doc.get("declaration").and_then(Item::as_table_like) else {
        return Ok(Vec::new());
    };
    Ok(family(table, family_key, source_dep_row)?)
}

/// Every source-dependency row a lock at `workspace_dir` carries under `family_key` — the
/// fingerprinted content dependencies emit wrote, read back for the drift comparison and
/// the reference-edge lift. A missing lock or an absent family yields none; a present
/// row missing a required column, or the wrong type in one, is surfaced loud.
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock exists but cannot be read or parsed, or if a
/// present dependency row is malformed.
fn source_deps(workspace_dir: &Path, family_key: &str) -> Result<Vec<LayoutImportRow>, DriftError> {
    let path = workspace_dir.join(crate::LOCK_FILENAME);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => return Err(DriftError::LockRead { path, source }),
    };
    increment_lock_reads();
    let doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse {
            path: path.clone(),
            source,
        })?;
    increment_lock_parses();
    source_deps_from_doc(&doc, family_key)
}

/// Every layout-import row a lock at `workspace_dir` carries — the layout sources'
/// fingerprinted content dependencies, for the drift comparison and the import-edge lift.
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock cannot be read/parsed or a present row is malformed.
pub fn layout_imports(workspace_dir: &Path) -> Result<Vec<LayoutImportRow>, DriftError> {
    source_deps(workspace_dir, LAYOUT_IMPORT_FAMILY)
}

/// Every layout-import row from an already-parsed lock document — the layout sources'
/// fingerprinted content dependencies, for the drift comparison and the import-edge lift.
///
/// # Errors
///
/// Returns a [`DriftError::LockRow`] if a present row is malformed.
pub fn layout_imports_from_doc(doc: &DocumentMut) -> Result<Vec<LayoutImportRow>, DriftError> {
    source_deps_from_doc(doc, LAYOUT_IMPORT_FAMILY)
}

/// Every composed-prose include row a lock at `workspace_dir` carries — the include
/// targets' fingerprinted content dependencies, for the drift comparison and the
/// include-edge lift (folded into the same `import`-locus edge set as a layout import).
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock cannot be read/parsed or a present row is malformed.
pub fn includes(workspace_dir: &Path) -> Result<Vec<LayoutImportRow>, DriftError> {
    source_deps(workspace_dir, INCLUDE_FAMILY)
}

/// Every composed-prose include row from an already-parsed lock document — the include
/// targets' fingerprinted content dependencies, for the drift comparison and the
/// include-edge lift (folded into the same `import`-locus edge set as a layout import).
///
/// # Errors
///
/// Returns a [`DriftError::LockRow`] if a present row is malformed.
pub fn includes_from_doc(doc: &DocumentMut) -> Result<Vec<LayoutImportRow>, DriftError> {
    source_deps_from_doc(doc, INCLUDE_FAMILY)
}

/// The drift findings for a workspace's source dependencies under `family`: a
/// fingerprinted target whose bytes no longer match the lock's `import_hash` — the target
/// moved and `emit` has not re-run — or one no longer readable, the dependency gone. One
/// `warn` finding per drifted dependency (under `rule`, its target described as a
/// `noun`), the same advisory posture [`config_stale`] takes over a committed projection:
/// the drift is surfaced, never a hard gate the author did not declare.
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock cannot be read/parsed or a present row is malformed.
/// The drift findings for source dependencies under `family` from an already-parsed
/// document: a fingerprinted target whose bytes no longer match the lock's `import_hash` —
/// the target moved and `emit` has not re-run — or one no longer readable, the dependency
/// gone. One `warn` finding per drifted dependency (under `rule`, its target described as
/// a `noun`), the same advisory posture [`config_stale`] takes over a committed projection:
/// the drift is surfaced, never a hard gate the author did not declare.
///
/// # Errors
///
/// Returns a [`DriftError`] if a present row is malformed.
pub fn source_dep_stale_from_doc(
    doc: &DocumentMut,
    harness_root: &Path,
    family: &str,
    rule: &str,
    noun: &str,
) -> Result<Vec<crate::check::Diagnostic>, DriftError> {
    let mut findings = Vec::new();
    for row in source_deps_from_doc(doc, family)? {
        match fs::read(harness_root.join(&row.source_path)) {
            Ok(bytes) if sha256_hex(&canonicalize_eol(&bytes)) == row.import_hash => {}
            Ok(_) => findings.push(crate::check::Diagnostic::warn(
                rule,
                &row.source_path,
                format!(
                    "{noun} target `{}` (referenced by `{}`) no longer matches the lock's fingerprint — the target changed and `emit` has not run; re-emit to reconcile",
                    row.source_path, row.member
                ),
            )),
            Err(_) => findings.push(crate::check::Diagnostic::warn(
                rule,
                &row.source_path,
                format!(
                    "{noun} target `{}` (referenced by `{}`) is no longer readable — the fingerprinted dependency moved or was removed; re-emit to reconcile",
                    row.source_path, row.member
                ),
            )),
        }
    }
    Ok(findings)
}

fn source_dep_stale(
    workspace_dir: &Path,
    family: &str,
    rule: &str,
    noun: &str,
) -> Result<Vec<crate::check::Diagnostic>, DriftError> {
    let path = workspace_dir.join(crate::LOCK_FILENAME);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => return Err(DriftError::LockRead { path, source }),
    };
    increment_lock_reads();
    let doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse {
            path: path.clone(),
            source,
        })?;
    increment_lock_parses();
    let harness_root = harness_root_of(workspace_dir);
    source_dep_stale_from_doc(&doc, &harness_root, family, rule, noun)
}

/// The drift findings for a workspace's layout imports — a moved or unreadable import
/// target, surfaced as a `warn` under `layout.import-stale`.
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock cannot be read/parsed or a present row is malformed.
pub fn layout_import_stale(
    workspace_dir: &Path,
) -> Result<Vec<crate::check::Diagnostic>, DriftError> {
    source_dep_stale(
        workspace_dir,
        LAYOUT_IMPORT_FAMILY,
        "layout.import-stale",
        "layout import",
    )
}

/// The drift findings for layout imports from an already-parsed lock document — a moved
/// or unreadable import target, surfaced as a `warn` under `layout.import-stale`.
///
/// # Errors
///
/// Returns a [`DriftError`] if a present row is malformed.
pub fn layout_import_stale_from_doc(
    doc: &DocumentMut,
    harness_root: &Path,
) -> Result<Vec<crate::check::Diagnostic>, DriftError> {
    source_dep_stale_from_doc(
        doc,
        harness_root,
        LAYOUT_IMPORT_FAMILY,
        "layout.import-stale",
        "layout import",
    )
}

/// The drift findings for a workspace's composed-prose includes — a moved or unreadable
/// include target, surfaced as a `warn` under `prose.include-stale`.
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock cannot be read/parsed or a present row is malformed.
pub fn include_stale(workspace_dir: &Path) -> Result<Vec<crate::check::Diagnostic>, DriftError> {
    source_dep_stale(
        workspace_dir,
        INCLUDE_FAMILY,
        "prose.include-stale",
        "prose include",
    )
}

/// The drift findings for composed-prose includes from an already-parsed lock document —
/// a moved or unreadable include target, surfaced as a `warn` under `prose.include-stale`.
///
/// # Errors
///
/// Returns a [`DriftError`] if a present row is malformed.
pub fn include_stale_from_doc(
    doc: &DocumentMut,
    harness_root: &Path,
) -> Result<Vec<crate::check::Diagnostic>, DriftError> {
    source_dep_stale_from_doc(
        doc,
        harness_root,
        INCLUDE_FAMILY,
        "prose.include-stale",
        "prose include",
    )
}

// ---------------------------------------------------------------------------
// emit-owned paths — the lock-grounded basis for `install`'s guard/note/modeline
// placements
// ---------------------------------------------------------------------------

/// One member the lock declares **emit-owned** — a real projection, not a lifted
/// member's own authored file.
pub struct EmitOwnedEntry {
    /// The member's kind (bare name — `"skill"`, `"rule"`, `"memory"`).
    pub kind: String,
    /// The member's name.
    pub name: String,
    /// The projected artifact's path as the lock spells it: relative to the harness root.
    /// A consumer reaching disk joins it onto the root it was aimed at; the guard matches
    /// it as a suffix of an absolute `file_path` (`install::matches_projection`).
    pub path: PathBuf,
}

/// Every path a lock at `workspace_dir` declares **emit-owned** — the constituency
/// `install`'s guard/note/modeline placements bind to, replacing the raw discovery
/// walk they once targeted. Every row the lock carries is emit-owned — whole
/// conversion means there is no other kind of row.
/// A missing or malformed lock yields no targets — the same "no lock, nothing to
/// bind" absence [`config_stale`] treats identically.
#[must_use]
pub fn emit_owned_targets(workspace_dir: &Path) -> Vec<EmitOwnedEntry> {
    walk_lock_rows(workspace_dir)
        .into_iter()
        .filter_map(|raw| {
            let (Some(name), Some(source_path)) = (raw.name, raw.source_path) else {
                return None;
            };
            Some(EmitOwnedEntry {
                kind: raw.kind,
                name,
                path: PathBuf::from(source_path),
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// declaration rows — the program's erased declarations
// ---------------------------------------------------------------------------

/// The lock's **declaration-row family** — the composed program's erased declarations,
/// beside the
/// per-member provenance and emit-fingerprint rows. Seven sub-families: the program's
/// [kind facts](KindFactRow), its [clauses](ClauseRow), its [requirements](RequirementRow),
/// its assembly facts, its
/// [`satisfies`](SatisfiesRow) fill edges, its [`mention`](MentionRow) edges, and its
/// [`nested_member`](NestedMemberRow) rows.
///
/// Written into the lock by [`emit`] off the SDK's own payload ([`Declarations::write_into`])
/// and read back here ([`read_declarations`]) for the gate's one disk-vs-lock comparison —
/// `import`'s own extraction still writes this family for the `check` path it feeds
/// (`GATE-READ-LOCK-DEMOLITION`, next in the chain, moves that read onto the lock too).
/// Each family's columns are owned scalars (or small owned collections for a set-scope
/// facet) so the read and write sides are the same shape: the lock is the vocabulary,
/// not a typed IR. `#[derive(Deserialize)]` doubles this shape as the SDK payload's own
/// wire format — the same rows, whether they arrive off disk or off the seam's JSON pipe.
///
/// Not `Eq`: its [`ClauseRow`]s may carry `f64` `range` bounds.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, ts_rs::TS)]
pub struct Declarations {
    /// The kind facts — one per kind in the program.
    pub kinds: Vec<KindFactRow>,
    /// The clauses of every kind's effective contract.
    pub clauses: Vec<ClauseRow>,
    /// The named requirements the assembly declares.
    pub requirements: Vec<RequirementRow>,
    /// The assembly-scope facts — the root member's declared enforcement `mode`,
    /// edges.
    pub assembly: Vec<AssemblyFactRow>,
    /// The member→requirement fill edges — every imported member's `satisfies` keys,
    /// so the roster/coverage
    /// tiers ride the lock rather than re-importing the harness.
    pub satisfies: Vec<SatisfiesRow>,
    /// The authored `n` mention edges — every member's already-resolved prose
    /// mentions, so the reference graph carries them alongside every other declared
    /// edge locus.
    pub mentions: Vec<MentionRow>,
    /// The composed-prose includes — a seam-inbound family only: `emit` resolves each
    /// against disk, splices the target's bytes into the host's projection, and lowers
    /// it to a fingerprinted `include` source dependency (never written back into this
    /// declaration table, so a lock round-trip reads it empty).
    #[serde(default)]
    pub includes: Vec<IncludeRow>,
    /// The host members' declared embedded-member facts — captured as declaration
    /// rows rather than a second copy the engine reads back off the rendered fence
    /// (0018, "the projection is not the database").
    pub nested_members: Vec<NestedMemberRow>,
    /// The fields-only registration members the SDK erased for the manifest write face —
    /// seam-inbound carrying their folded fields, so `emit` routes each host manifest whole
    /// through the canonical write face. The lock's `registration` family records only each
    /// member's identity and collection address; the fields live in the projected manifest
    /// artifact, never a second copy read back (0018), so a lock round-trip reads them
    /// fieldless.
    #[serde(default)]
    pub registrations: Vec<RegistrationRow>,
    /// The harness-level settings residue — seam-inbound opaque `settings.json` keys with
    /// no member home, folded into their manifest's residue at emit. Like `includes`, never
    /// written into this declaration table, so a lock round-trip reads none.
    #[serde(default)]
    pub settings: Vec<SettingsRow>,
}

/// One kind's declaration row — its identity and declared runtime facts.
/// The optional facts are omitted from the lock when the kind declares none, so the row
/// round-trips to exactly what was written.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct KindFactRow {
    /// The bare kind name.
    pub name: String,
    /// The declared provider authority, when the kind qualifies by one.
    #[serde(default)]
    pub provider: Option<String>,
    /// The `governs` locus root directory. Absent together with
    /// [`governs_glob`](KindFactRow::governs_glob) for a **nested file** kind: its members'
    /// paths compose from their host's unit and the host template's pattern, so it governs
    /// no glob of its own and two kinds still never share one.
    #[serde(default)]
    pub governs_root: Option<String>,
    /// The `governs` locus filename glob, absent for a nested file kind (see
    /// [`governs_root`](KindFactRow::governs_root)).
    #[serde(default)]
    pub governs_glob: Option<String>,
    /// The file locus's declared **commitment class** label — `local` for a per-machine,
    /// uncommitted locus: the kind is declared and reviewed, its members' documents are
    /// not. Absent for the committed class every
    /// shipped kind takes, so an ordinary row stays byte-identical — the same tolerant
    /// round-trip the rest of the optional facts take.
    ///
    /// A local member's rows never enter the lock, so this column is the *whole* of a
    /// local kind's residue here: the row declares the kind, and the documents it governs
    /// derive at read time under it.
    #[serde(default)]
    pub commitment: Option<String>,
    /// The declared projection format label, when declared.
    #[serde(default)]
    pub format: Option<String>,
    /// The declared unit-shape label, when declared.
    #[serde(default)]
    pub unit_shape: Option<String>,
    /// The declared registration channel set's wire labels, in declaration order.
    /// Empty for a kind that declares none, the same tolerant round-trip
    /// [`templates`](KindFactRow::templates) takes.
    #[serde(default)]
    #[ts(as = "Option<Vec<String>>", optional)]
    pub registration: Vec<String>,
    /// The kind's declared nesting templates — one [`TemplateRow`] per inner layer of
    /// nested members it hosts. Empty for
    /// a kind that nests nothing, the tolerant round-trip a lockless/template-less
    /// kind takes.
    #[serde(default)]
    #[ts(as = "Option<Vec<TemplateRow>>", optional)]
    pub templates: Vec<TemplateRow>,
    /// The declared content: absent for a `file`-content kind (the default the whole
    /// built-in set takes, so those rows stay byte-identical), a [`LayoutRow`] for a
    /// kind whose body is a declared layout over its heading tree.
    #[serde(default)]
    pub content: Option<LayoutRow>,
    /// The **fields-only** body-shape marker — `fields` for a no-body-slot kind (a hook,
    /// an MCP server); absent for a body-bearing kind, whose body is `file` or a `content`
    /// layout. The tolerant `#[serde(default)]` round-trip the rest of the optional facts
    /// take, so a body-bearing kind's row stays byte-identical.
    #[serde(default)]
    pub shape: Option<String>,
    /// The declared **collection address** — for a registration member surfacing inside a
    /// host manifest, which manifest and which key path it keys at. Absent for a
    /// file-locus kind, so an ordinary row stays byte-identical.
    #[serde(default)]
    pub collection_address: Option<CollectionAddressRow>,
    /// Advisory authoring counsel for the kind as a whole — teaching at authoring time via
    /// `schema` hover or `explain`, carrying no predicate or severity (decision 0045).
    #[serde(default)]
    pub guidance: Option<String>,
    /// External-fact source backing the guidance — a doc URL plus retrieved date.
    #[serde(default)]
    pub cite: Option<String>,
}

/// A kind's declared **nesting template** row — one inner layer of nested members the
/// kind hosts: the child kind, plus the `path` pattern (relative to the parent's unit)
/// when that layer's children are files (`model/representation.md`, "kind"). The child
/// kind alone means an embedded layer: the children live in the host's own body, so no
/// path addresses them.
///
/// A declared template is the kind's own nesting *fact*, never a resolution rule: a
/// host's actual embedded members are resolved off [`Declarations::nested_members`] by
/// address, and nothing discovers a file child off its `path` pattern.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct TemplateRow {
    /// The child kind this layer templates — the `member.<kind>` an embedded child's
    /// fence info string carries, or the kind a file child's unit is read as.
    pub kind: String,
    /// The path pattern a file child's unit sits at, relative to the parent's unit
    /// (a skill's `*.md`). Absent for an embedded layer, whose children have no unit of
    /// their own.
    #[serde(default)]
    pub path: Option<String>,
}

/// A kind's declared **collection address** row — the manifest a registration member
/// surfaces in and the key path it keys at, the presence-coupled pair
/// [`KindFactRow::collection_address`] carries. Absent from a [`KindFactRow`] means the
/// kind owns its own file locus.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct CollectionAddressRow {
    /// The host manifest the registration surfaces in (`settings.json`, `.mcp.json`).
    pub manifest: String,
    /// The manifest key path the registration keys at (`hooks.<Event>`, `mcpServers.*`), a
    /// closed vocabulary the engine's kind lift rejects an unknown value from.
    pub key_path: String,
    /// The entry's declared shape — whether it's an object, scalar, or group-array.
    #[serde(default)]
    pub entry_shape: Option<String>,
}

/// A kind's declared **layout** — the ordered region rows a `layout`-content kind's body
/// is read as. Absent from a [`KindFactRow`] means the kind is `file`-content; a present
/// (even empty) layout means the body is a declared template.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct LayoutRow {
    /// The layout's regions, in declared document order.
    #[serde(default)]
    pub regions: Vec<LayoutRegionRow>,
}

/// One [`LayoutRow`] region — one of the three corpus primitives, flattened to a
/// discriminator plus each primitive's own optional columns (the same discriminator +
/// optional-columns shape [`AssemblyFactRow`] takes). `prose` carries an optional
/// `import`; `field` a `slot`; `collection` a `member_kind` and an optional `key`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct LayoutRegionRow {
    /// The primitive discriminator: `prose`, `field`, or `collection`.
    pub region: String,
    /// A `prose` region's import reference, when it imports a file's contents rather
    /// than carrying its own verbatim words.
    #[serde(default)]
    pub import: Option<String>,
    /// A `field` region's named field slot.
    #[serde(default)]
    pub slot: Option<String>,
    /// A `collection` region's child member kind.
    #[serde(default)]
    pub member_kind: Option<String>,
    /// A `collection` region's explicit identity key, when declared.
    #[serde(default)]
    pub key: Option<String>,
}

/// One clause of a kind's effective contract, reduced to the columns the lock records:
/// which kind it governs, the predicate's key, the field it targets (when it names one),
/// its declared severity, its guidance and cite — the clause's four channels
/// —
/// and, per predicate, its own argument: the node-set/edge-scope predicates
/// carry
/// their bounds/target, and the node-scope predicates that need more than
/// `field`/`severity` (`min_len`/`max_len`/`extent`'s bound, `extent`'s unit,
/// `allowed_chars`'s charset, `forbidden_keys`'s keys, `deny`'s values, `type`'s declared
/// kind) carry theirs too — so a kind's
/// own floor clause round-trips losslessly, not identity+severity alone.
/// `unique`'s field rides the shared `field`
/// column (the same slot `required`/`min_len`/… target); the rest carry their own
/// optional columns since a plain field/severity pair cannot express them.
///
/// Not `Eq`: the `range` column carries `f64` bounds ([`RangeBoundRow`]), so the
/// whole clause-row family it rides is `PartialEq`-only — the same trade
/// [`crate::contract::Predicate`] already makes for its own `Range` bounds.
#[derive(Debug, Clone, Deserialize, PartialEq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct ClauseRow {
    /// The clause's **address** — the deterministic, human-legible label
    /// [`crate::contract::clause_label`] derives from the row's identity columns, and
    /// the row's identity in the lock: every finding this clause produces prints it as
    /// the diagnostic `rule` id, `explain` narrates it, and the dial names a clause by
    /// it.
    ///
    /// `None` on a payload row and `Some` on a lock row: emit is the one writer
    /// ([`emit`]), stamping every row as it composes the lock, so the seam has no label
    /// to author and no way to author a wrong one.
    #[serde(default)]
    pub label: Option<String>,
    /// The kind whose contract carries the clause. `None` when this row is nested
    /// inside a [`RequirementRow`]'s own [`clauses`](RequirementRow::clauses) — a
    /// requirement's set-scope demand names no kind of its own; it ranges over
    /// whatever kind the requirement's own row already carries.
    #[serde(default)]
    pub kind: Option<String>,
    /// The predicate's clause key (`required`, `max_len`, …).
    pub predicate: String,
    /// The field (or marker) the predicate constrains, when it names one.
    #[serde(default)]
    pub field: Option<String>,
    /// The clause's declared severity (`required` / `advisory`).
    pub severity: String,
    /// The just-in-time teaching channel — the best-practice prose the predicate
    /// cannot encode, quoted at the point of a failing finding.
    #[serde(default)]
    pub guidance: Option<String>,
    /// The external-fact source backing the clause — a doc URL plus retrieved date,
    /// carried as data.
    #[serde(default)]
    pub cite: Option<String>,
    /// The `count` clause's satisfier-set-size bound, when the predicate is `count`.
    #[serde(default)]
    pub count: Option<CountBoundRow>,
    /// The `membership` clause's target requirement name, when the predicate is
    /// `membership`.
    #[serde(default)]
    pub target: Option<String>,
    /// The `degree` clause's in/out edge-count bound, when the predicate is `degree`.
    #[serde(default)]
    pub degree: Option<DegreeBoundRow>,
    /// The `mention-reachable` clause's **target-side gate field**, when the predicate
    /// is `mention-reachable`. The one predicate taking two field arguments: its
    /// source-side scope field rides the shared [`field`](ClauseRow::field) column, and
    /// this column carries the other end — the field read off the *mentioned* member,
    /// which `field` alone cannot express.
    #[serde(default)]
    pub gate: Option<String>,
    /// The `type` clause's declared source kinds, when the predicate is `type` — the
    /// lattice names (`string`/`integer`/`number`/`boolean`/`null`/`list`/`map`) that
    /// [`crate::extract::ValueType::from_name`] decodes. Carried as names rather than
    /// as [`crate::extract::ValueType`]s: the lattice is a feature-side type, and the
    /// row family decodes its arguments at the boundary.
    ///
    /// A **set**, since a `type` clause declares one: the column is an array of names
    /// in lattice order, and a one-element array is the single-kind clause. A lock
    /// written by an older engine spells that case as a bare string, which
    /// [`ClauseRow::from_table`] reads as the one-element set it means; the next `emit`
    /// rewrites the file whole in the array form, which is the upgrade — a committed
    /// lock is re-emitted from its source, never patched in place.
    #[serde(default)]
    pub value_type: Option<Vec<String>>,
    /// The `shape` clause's declared shape, when the predicate is `shape` — the closed
    /// set's own spelling (`hyphen-placement`/`no-xml-tags`) that
    /// [`crate::contract::Shape::from_name`] decodes. Carried as a name rather than as a
    /// [`crate::contract::Shape`], the trade `value_type` already makes: the row family
    /// carries its arguments as names and decodes them at the boundary.
    ///
    /// One name, never a set — a clause names exactly one shape — so this is a plain
    /// string column, and no read-side skew tolerance answers it.
    #[serde(default)]
    pub shape: Option<String>,
    /// The `min_len`/`max_len`/`extent` clause's scalar bound, when the predicate
    /// is one of those three.
    #[serde(default)]
    pub bound: Option<BoundRow>,
    /// The `extent` clause's declared unit (`lines`/`characters`), when the predicate is
    /// `extent` — the closed set [`crate::contract::ExtentUnit::from_name`] decodes, an
    /// unknown value refused at load. Carried as a name, the trade the whole row family
    /// makes for its closed-vocabulary arguments.
    #[serde(default)]
    pub unit: Option<String>,
    /// The `allowed_chars` clause's declared character class, when the predicate is
    /// `allowed_chars`.
    #[serde(default)]
    pub charset: Option<CharsetRow>,
    /// The `forbidden_keys` clause's forbidden key list, when the predicate is
    /// `forbidden_keys`.
    #[serde(default)]
    pub keys: Option<Vec<String>>,
    /// The `deny` clause's forbidden value list, when the predicate is `deny`.
    #[serde(default)]
    pub values: Option<Vec<String>>,
    /// The `range` clause's inclusive numeric bound, when the predicate is `range`.
    #[serde(default)]
    pub range: Option<RangeBoundRow>,
    /// The `section_contains` clause's heading prefix and required marker, when the
    /// predicate is `section_contains`.
    #[serde(default)]
    pub section: Option<SectionContainsRow>,
    /// The `require_sections` clause's required heading list, when the predicate is
    /// `require_sections`.
    #[serde(default)]
    pub sections: Option<Vec<String>>,
    /// The `when` clause's guard predicate key (`enum` or `type`), when the predicate
    /// is `when`. The guard predicate's own arguments (field, values, etc.) ride the
    /// shared columns as if it were a bare guard row.
    #[serde(default)]
    pub guard_predicate: Option<String>,
    /// The `when` clause's nested body rows, when the predicate is `when` — the
    /// clauses evaluated where the guard holds.
    #[serde(default)]
    pub body: Option<Vec<ClauseRow>>,
}

/// A `range` clause row's inclusive numeric bound — `f64` so one predicate spans both
/// integer and fractional fields. Not `Eq`: `f64` is only `PartialEq`, which is why
/// the whole clause-row family it rides is `PartialEq`-only.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, ts_rs::TS)]
pub struct RangeBoundRow {
    /// The inclusive lower bound.
    pub min: f64,
    /// The inclusive upper bound.
    pub max: f64,
}

/// A `section_contains` clause row's arguments — the heading-text prefix selecting the
/// governed sections and the marker every governed section's body must carry.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct SectionContainsRow {
    /// The heading-text prefix that selects the sections the clause governs.
    pub heading: String,
    /// The marker text every governed section's body must contain.
    pub marker: String,
}

/// A node-scope clause row's scalar bound — `min_len`'s `min`, `max_len`/`extent`'s
/// `max`, each endpoint optional so the row carries only what the predicate declared.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct BoundRow {
    /// The inclusive lower bound, when the predicate declares one (`min_len`).
    #[serde(default)]
    pub min: Option<usize>,
    /// The inclusive upper bound, when the predicate declares one (`max_len`/`extent`).
    #[serde(default)]
    pub max: Option<usize>,
}

/// An `allowed_chars` clause row's declared character class — the wire form of
/// [`crate::contract::Charset`]: inclusive `"<lo>-<hi>"` range specs plus a literal
/// string of individually permitted characters.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct CharsetRow {
    /// The inclusive character ranges, each a two-character `"<lo>-<hi>"` spec.
    #[serde(default)]
    #[ts(as = "Option<Vec<String>>", optional)]
    pub ranges: Vec<String>,
    /// The individually permitted characters, when any are declared.
    #[serde(default)]
    pub chars: Option<String>,
}

/// One named requirement's declaration row,
/// carrying the scalar facets plus the requirement's own **clause rows** — the
/// set-scope demands
/// the roster/graph checks range over. No facet columns: a demand's severity,
/// argument, and — for `unique`/`membership` — targeted field ride the nested
/// [`ClauseRow`], the identical row shape a kind's own floor clauses use.
///
/// Not `Eq`: its nested [`ClauseRow`]s may carry `f64` `range` bounds.
#[derive(Debug, Clone, Deserialize, PartialEq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct RequirementRow {
    /// The requirement's name.
    pub name: String,
    /// The kind that may fill it, when typed by one.
    #[serde(default)]
    pub kind: Option<String>,
    /// Whether an unfilled requirement blocks the gate.
    #[serde(default)]
    pub required: bool,
    /// The requirement's set-/edge-scope demands, in declaration order — a
    /// `count`/`unique`/`membership`/`degree` [`ClauseRow`] per clause, each
    /// carrying its own severity. Empty ⇒ no set-scope demand.
    #[serde(default)]
    pub clauses: Vec<ClauseRow>,
    /// The typed verifier for the behavioral remainder, when declared — a
    /// species-tagged [`Verifier`], resolved at admissibility, never run.
    #[serde(default)]
    pub verifier: Option<crate::compose::Verifier>,
    /// The authored intent the requirement exists to carry, when declared —
    /// carried verbatim, never interpreted.
    #[serde(default)]
    pub prose: Option<String>,
}

/// A requirement row's `count` bound — the satisfier-set size's inclusive `[min, max]`.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct CountBoundRow {
    /// The inclusive lower bound on the satisfier-set size.
    pub min: usize,
    /// The inclusive upper bound on the satisfier-set size.
    pub max: usize,
}

/// A requirement row's `degree` bound — the in/out edge-count bound every satisfier
/// must land in.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct DegreeBoundRow {
    /// The bound on a satisfier's incoming edge count, when constrained.
    #[serde(default)]
    pub incoming: Option<EdgeBoundRow>,
    /// The bound on a satisfier's outgoing edge count, when constrained.
    #[serde(default)]
    pub outgoing: Option<EdgeBoundRow>,
}

/// One direction's inclusive `[min, max]` edge-count bound, each endpoint optional.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct EdgeBoundRow {
    /// The inclusive lower bound. `None` ⇒ no lower bound.
    #[serde(default)]
    pub min: Option<usize>,
    /// The inclusive upper bound. `None` ⇒ unbounded above.
    #[serde(default)]
    pub max: Option<usize>,
}

/// One member→requirement fill edge's declaration row — the `satisfies` join the
/// roster/coverage tiers need, carried on the lock rather than re-imported.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct SatisfiesRow {
    /// The filling member's own `kind:name` address — the same qualified label a
    /// [`MentionRow`] carries, so a same-named member of another kind never collides.
    pub member: String,
    /// The requirement key the member opts into filling.
    pub requirement: String,
}

/// One authored `n` mention edge's declaration row — the citing member's own
/// `kind:name` address and the address its mention names (another member's
/// `kind:name`, or a bare requirement name). Recorded unconditionally, carrying no
/// resolution state of its own: `emit` refuses a mention naming no declared kind
/// before a byte is written, while a mention naming a declared kind with no composed
/// member defers — its row rides the lock for `check` to resolve against the
/// discovered corpus.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct MentionRow {
    /// The citing member's own `kind:name` address.
    pub member: String,
    /// The address the mention names.
    pub target: String,
}

/// One composed-prose include the SDK declares — the host member's own `kind:name`
/// address and the include target's path, resolved by the SDK against the stating
/// module (never the workspace) to an absolute path. A seam-inbound row only: `emit`
/// resolves it against disk ([`resolve_source_dependency`]), splices the target's bytes
/// into the host projection at the body's include slot, and lowers it to a fingerprinted
/// `include` source dependency — this row itself never reaches the lock.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct IncludeRow {
    /// The host member's own `kind:name` address.
    pub member: String,
    /// The include target's SDK-resolved absolute path.
    pub source_path: String,
}

/// One fields-only registration member the SDK erased for the manifest write face — a
/// hook, an MCP server — carried across the seam so `emit` routes its host manifest whole
/// through the canonical write face ([`crate::json_manifest::write_manifest`]) rather than
/// the unrepresented in-place splice. `kind`/`key` are the member's identity; `manifest`/
/// `key_path` name the collection address it surfaces at; `fields` are its folded typed
/// fields — the entry value the write face places under `key`.
///
/// **Seam-inbound with `fields`.** The lock's `registration` declaration family records
/// only the identity and address: the fields live in the projected manifest artifact, never
/// a second copy the engine reads back (0018, "the projection is not the database"), so a
/// row read back off the lock carries an empty `fields`.
#[derive(Debug, Clone, Deserialize, PartialEq, ts_rs::TS)]
pub struct RegistrationRow {
    /// The registration kind's bare name — `hook`, `mcp-server` — joining `declarations.kinds`.
    pub kind: String,
    /// The member's key among its collection's entries — a hook's event, a server's name.
    pub key: String,
    /// The host manifest the registration surfaces in (`settings.json`, `.mcp.json`).
    pub manifest: String,
    /// The manifest key-path label the registration keys at (`hooks.<Event>`, `mcpServers.*`).
    pub key_path: String,
    /// The member's folded typed fields — seam-inbound only, dropped from the lock row.
    #[serde(default)]
    #[ts(type = "Array<[string, unknown]>")]
    pub fields: Vec<(String, JsonValue)>,
}

/// One harness-level settings-residue key the SDK erased for the manifest write face — an
/// opaque top-level key of the manifest it names (Claude Code's `settings.json`) with no
/// typed member kind of its own yet. Carried across the seam so `emit` folds it into that
/// manifest's opaque residue beside the collection segments its registration members build.
///
/// **Seam-inbound with `value`.** Like a composed-prose include, this row is consumed at
/// emit and never written into the lock's declaration table: the value lives in the
/// projected manifest artifact, never a second copy the engine reads back (0018), so a lock
/// round-trip carries none.
#[derive(Debug, Clone, Deserialize, PartialEq, ts_rs::TS)]
pub struct SettingsRow {
    /// The host manifest the residue key surfaces in (`settings.json`).
    pub manifest: String,
    /// The residue key — an opaque top-level manifest key with no member home.
    pub key: String,
    /// The key's opaque JSON value, placed verbatim into the manifest's residue.
    #[ts(type = "unknown")]
    pub value: JsonValue,
}

/// One host member's declared embedded-member value's declaration row — its
/// identity (the host's own `kind:name` address, the embedded child kind, and its
/// key) plus its leaves and sibling collections: the same composed value
/// `blocks()` renders into the host's `member.<kind> <key>` fence. The sole fact
/// source the read side consumes (`crate::builtin_kind::features`, matched by
/// `host` address) — never a second copy of a value the engine reads back off its
/// own rendering (0018, "the projection is not the database").
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
pub struct NestedMemberRow {
    /// The host member's own `kind:name` address.
    pub host: String,
    /// The embedded child kind this value instantiates.
    pub kind: String,
    /// The value's key — the identity a leaf address carries.
    pub key: String,
    /// Prose leaves, keyed by field name.
    #[serde(default)]
    pub leaves: BTreeMap<String, String>,
    /// Sibling collections, one row per entry, in authored order — the SDK's
    /// collection-name-keyed wire shape flattened by [`deserialize_collections`].
    /// The Rust side flattens the map to a `Vec`; the seam type carries the wire
    /// shape the flatten reads, a map of collection name to its ordered entries.
    #[serde(default, deserialize_with = "deserialize_collections")]
    #[ts(as = "std::collections::BTreeMap<String, Vec<CollectionEntryWire>>")]
    pub collections: Vec<CollectionEntryRow>,
    /// The declared edge fields this value's **format placed** — which of them the
    /// format selected while `emit` rendered the value, sorted. The engine never sees
    /// a format and never reads a rendering back, so an edge's placement reaches it
    /// here or not at all; the declared set it is measured against is the `assembly`
    /// family's `edge` facts for [`kind`](Self::kind).
    ///
    /// `None` and `Some(vec![])` are distinct, and the distinction is the whole point:
    /// `Some(vec![])` is a format that placed no edge (a `format-places-edges` finding
    /// per declared edge), while `None` is a value **no format rendered** — a member
    /// embedded in a layout document is read off its host's declared layout, so it has
    /// no format to omit anything and the clause has nothing to decide. Absent from a
    /// row whose value no format rendered, so an ordinary row stays byte-identical.
    #[serde(default)]
    #[ts(optional)]
    pub placed_edges: Option<Vec<String>>,
    /// The value's **rendered extent** in lines — the line count of the block `emit`
    /// projected for this member, captured off the same render, so an `extent` clause bound
    /// to the embedded kind budgets real data instead of a hardcoded zero.
    ///
    /// `None` is a member **no format rendered** — one embedded in a layout document, read
    /// off its host's declared layout rather than projected — which has no rendered span to
    /// measure, so its `extent` stays undecidable. Absent from such a row, so an ordinary
    /// row stays byte-identical (the [`placed_edges`](Self::placed_edges) precedent).
    #[serde(default)]
    #[ts(optional)]
    pub rendered_lines: Option<usize>,
    /// The value's **rendered extent** in characters — the second unit an `extent` clause
    /// measures in, captured off the same render. `None` on the same terms as
    /// [`rendered_lines`](Self::rendered_lines).
    #[serde(default)]
    #[ts(optional)]
    pub rendered_chars: Option<usize>,
}

/// One entry belonging to one of a [`NestedMemberRow`]'s sibling collections: the
/// collection name, the entry's own key, and its leaf fields — the row's
/// flattened, order-preserving shape (`to_table`/`from_table` serialize the whole
/// column as one array, the discipline every other array-shaped declaration
/// family gets from `toml_edit`).
#[derive(Debug, Clone, PartialEq, Eq, ts_rs::TS)]
pub struct CollectionEntryRow {
    /// The collection this entry belongs to.
    pub collection: String,
    /// The entry's key among its collection's siblings.
    pub key: String,
    /// The entry's own leaf fields, field name → authored string.
    pub leaves: BTreeMap<String, String>,
}

/// One collection entry's wire shape as the SDK payload carries it, nested under
/// its owning collection name — [`deserialize_collections`] copies the collection
/// name onto each entry it flattens into a [`CollectionEntryRow`].
#[derive(Debug, Clone, Deserialize, ts_rs::TS)]
pub struct CollectionEntryWire {
    /// The entry's key among its collection's siblings.
    pub key: String,
    /// The entry's own leaf fields, field name → resolved string.
    #[serde(default)]
    pub leaves: BTreeMap<String, String>,
}

/// Deserialize a [`NestedMemberRow`]'s `collections` column off the SDK payload's
/// wire shape: a map of collection name to an authored-order array of `{key,
/// leaves}` entries. A hand-written visitor rather than an intermediate `Map`
/// type, so the entries' authored order survives untouched by any incidental
/// reordering a keyed map's own iteration would introduce.
fn deserialize_collections<'de, D>(deserializer: D) -> Result<Vec<CollectionEntryRow>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct CollectionsVisitor;

    impl<'de> serde::de::Visitor<'de> for CollectionsVisitor {
        type Value = Vec<CollectionEntryRow>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a map of collection name to an ordered array of entries")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut rows = Vec::new();
            while let Some((collection, entries)) =
                map.next_entry::<String, Vec<CollectionEntryWire>>()?
            {
                for entry in entries {
                    rows.push(CollectionEntryRow {
                        collection: collection.clone(),
                        key: entry.key,
                        leaves: entry.leaves,
                    });
                }
            }
            Ok(rows)
        }
    }

    deserializer.deserialize_map(CollectionsVisitor)
}

/// One assembly-scope fact — the root member's own declarations plus the
/// graph edges the harness binds: a `fact` discriminator (`mode`, `edge`)
/// plus the columns that fact carries. Absent columns are omitted from the
/// lock, so each row round-trips to exactly what its producer wrote.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(optional_fields)]
pub struct AssemblyFactRow {
    /// The fact discriminator: `mode` or `edge`.
    pub fact: String,
    /// The scalar value a `mode` fact carries (the root member's declared
    /// enforcement mode).
    #[serde(default)]
    pub value: Option<String>,
    /// An `edge` fact's source kind.
    #[serde(default)]
    pub from: Option<String>,
    /// An `edge` fact's reference field.
    #[serde(default)]
    pub field: Option<String>,
    /// An `edge` fact's target kinds — the non-empty set the field may resolve into.
    #[serde(default)]
    pub to: Option<Vec<String>>,
}

impl Declarations {
    /// Serialize the declaration families into `doc` under an implicit `[declaration]`
    /// table — `[[declaration.kind]]`, `[[declaration.clause]]`, `[[declaration.requirement]]`,
    /// `[[declaration.assembly]]`, `[[declaration.satisfies]]`, `[[declaration.mention]]`,
    /// `[[declaration.nested_member]]` —
    /// each family in its producer's order so a re-emit is
    /// byte-identical. An empty family writes no array (an empty `ArrayOfTables`
    /// vanishes on the toml round-trip, so omitting it keeps write and re-parse symmetric),
    /// and an all-empty set writes no `[declaration]` table at all.
    pub(crate) fn write_into(&self, doc: &mut DocumentMut) {
        let mut table = Table::new();
        // Implicit: only the `[[declaration.<family>]]` sub-headers render, never a bare
        // `[declaration]` line.
        table.set_implicit(true);
        insert_family(
            &mut table,
            "kind",
            self.kinds.iter().map(KindFactRow::to_table),
        );
        insert_family(
            &mut table,
            "clause",
            self.clauses.iter().map(ClauseRow::to_table),
        );
        insert_family(
            &mut table,
            "requirement",
            self.requirements.iter().map(RequirementRow::to_table),
        );
        insert_family(
            &mut table,
            "assembly",
            self.assembly.iter().map(AssemblyFactRow::to_table),
        );
        insert_family(
            &mut table,
            "satisfies",
            self.satisfies.iter().map(SatisfiesRow::to_table),
        );
        insert_family(
            &mut table,
            "mention",
            self.mentions.iter().map(MentionRow::to_table),
        );
        insert_family(
            &mut table,
            "nested_member",
            self.nested_members.iter().map(NestedMemberRow::to_table),
        );
        insert_family(
            &mut table,
            "registration",
            self.registrations.iter().map(RegistrationRow::to_table),
        );
        if !table.is_empty() {
            doc["declaration"] = Item::Table(table);
        }
    }
}

/// Read and parse a workspace's lock.toml, incrementing the read/parse counters to pin
/// hoisting (once per run, shared across all call sites). A missing lock yields an empty
/// document; a malformed lock is an error. The parsed document can be passed to
/// `*_from_doc` functions to avoid re-reading the lock.
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock exists but cannot be read or parsed as TOML.
pub fn read_lock_document(workspace_dir: &Path) -> miette::Result<DocumentMut> {
    let path = workspace_dir.join(crate::LOCK_FILENAME);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            increment_lock_reads();
            increment_lock_parses();
            return Ok(DocumentMut::new());
        }
        Err(source) => return Err(DriftError::LockRead { path, source }.into()),
    };
    increment_lock_reads();
    let doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse {
            path: path.clone(),
            source,
        })?;
    increment_lock_parses();
    Ok(doc)
}

/// Read the lock's declaration-row family back into a typed [`Declarations`]:
/// the gate's read side over the
/// rows the extraction wrote. A missing or malformed lock, or one with no `[declaration]`
/// table (any pre-recut lock), yields an empty set rather than an error — absent evidence
/// forges no finding, the same tolerance
/// [`config_stale`] takes.
///
/// # Errors
///
/// Returns a [`DriftError`] if the lock exists but cannot be read or parsed as TOML.
pub fn read_declarations(workspace_dir: &Path) -> miette::Result<Declarations> {
    let path = workspace_dir.join(crate::LOCK_FILENAME);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Declarations::default());
        }
        Err(source) => return Err(DriftError::LockRead { path, source }.into()),
    };
    Ok(parse_declarations(&path, &text)?)
}

/// Parse a lock document's declaration-row family off already-read `text` — the
/// shared parser [`read_declarations`] and the embedded built-in lock
/// ([`crate::builtin_lock`]) both delegate to, so a malformed committed lock and a
/// malformed embed report through the identical [`DriftError::LockParse`]. `path`
/// labels the diagnostic only; the embedded lock has no on-disk workspace to root
/// it at, so it passes its own module path as a stand-in.
///
/// # Errors
///
/// Returns a [`DriftError::LockParse`] if `text` is not valid TOML, or a
/// [`DriftError::LockRow`] if a present declaration row is malformed.
pub fn parse_declarations(path: &Path, text: &str) -> Result<Declarations, DriftError> {
    let doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse {
            path: path.to_path_buf(),
            source,
        })?;
    Ok(declarations_from_doc(&doc)?)
}

/// Extract the seven declaration families off a parsed lock's `[declaration]` table. A
/// present row that fails its lift — a required column absent, a column the wrong type,
/// a malformed nested element — is a [`LockRowError`]; an absent family or an absent
/// optional column is legitimate absence.
///
/// # Errors
///
/// Returns a [`LockRowError`] naming the family of the first present-but-malformed row.
fn declarations_from_doc(doc: &DocumentMut) -> Result<Declarations, LockRowError> {
    let Some(table) = doc.get("declaration").and_then(Item::as_table_like) else {
        return Ok(Declarations::default());
    };
    Ok(Declarations {
        kinds: family(table, "kind", KindFactRow::from_table)?,
        clauses: family(table, "clause", ClauseRow::from_table)?,
        requirements: family(table, "requirement", RequirementRow::from_table)?,
        assembly: family(table, "assembly", AssemblyFactRow::from_table)?,
        satisfies: family(table, "satisfies", SatisfiesRow::from_table)?,
        mentions: family(table, "mention", MentionRow::from_table)?,
        // Includes are seam-inbound only — lowered to the fingerprinted `include` source
        // dependency at emit, never written into this declaration table, so a lock
        // round-trip reads none.
        includes: Vec::new(),
        nested_members: family(table, "nested_member", NestedMemberRow::from_table)?,
        registrations: family(table, "registration", RegistrationRow::from_table)?,
        // Settings residue is seam-inbound only — folded into its manifest's opaque residue
        // at emit, never written into this declaration table, so a lock round-trip reads none.
        settings: Vec::new(),
    })
}

/// Push a family's rows as an `[[declaration.<key>]]` array-of-tables, but only when
/// non-empty (an empty array vanishes on the toml round-trip).
fn insert_family(table: &mut Table, key: &str, rows: impl Iterator<Item = Table>) {
    let mut array = ArrayOfTables::new();
    for row in rows {
        array.push(row);
    }
    if !array.is_empty() {
        table.insert(key, Item::ArrayOfTables(array));
    }
}

/// Read one `[[declaration.<key>]]` family off the lock's declaration table, parsing each
/// present row through `parse`. An absent family is empty; a present family that is not
/// an array of tables, or one whose row fails its lift, is a [`LockRowError`] naming the
/// family.
fn family<T>(
    table: &dyn TableLike,
    key: &str,
    parse: impl Fn(&Table) -> Result<T, RowError>,
) -> Result<Vec<T>, LockRowError> {
    let Some(item) = table.get(key) else {
        return Ok(Vec::new());
    };
    let array = item
        .as_array_of_tables()
        .ok_or_else(|| LockRowError::FamilyShape {
            family: key.to_string(),
        })?;
    array
        .iter()
        .map(|row| parse(row).map_err(|err| err.at(key)))
        .collect()
}

/// A required string column — absent or non-string is a [`RowError`].
fn req_str(table: &dyn TableLike, column: &str) -> Result<String, RowError> {
    opt_str(table, column)?.ok_or_else(|| RowError::missing(column))
}

/// An optional string column — absent is `Ok(None)`, a present non-string is a [`RowError`].
fn opt_str(table: &dyn TableLike, column: &str) -> Result<Option<String>, RowError> {
    match table.get(column) {
        None => Ok(None),
        Some(item) => item
            .as_str()
            .map(|text| Some(text.to_string()))
            .ok_or_else(|| RowError::wrong(column, "string")),
    }
}

/// An optional boolean column — absent is `Ok(None)`, a present non-boolean is a [`RowError`].
fn opt_bool(table: &dyn TableLike, column: &str) -> Result<Option<bool>, RowError> {
    match table.get(column) {
        None => Ok(None),
        Some(item) => item
            .as_bool()
            .map(Some)
            .ok_or_else(|| RowError::wrong(column, "boolean")),
    }
}

/// An optional array-of-strings column — absent is `Ok(None)`; a present non-array, or one
/// carrying a non-string element, is a [`RowError`] (a tolerant row, never a tolerant element).
fn opt_str_array(table: &dyn TableLike, column: &str) -> Result<Option<Vec<String>>, RowError> {
    let Some(item) = table.get(column) else {
        return Ok(None);
    };
    let array = item
        .as_array()
        .ok_or_else(|| RowError::wrong(column, "array"))?;
    let mut out = Vec::with_capacity(array.len());
    for element in array.iter() {
        out.push(
            element
                .as_str()
                .ok_or_else(|| RowError::wrong(column, "array of strings"))?
                .to_string(),
        );
    }
    Ok(Some(out))
}

/// An optional string-array column that also reads a bare string as the one-element
/// array it means — absent is `Ok(None)`, any other shape a [`RowError`].
///
/// The one column read this way is `value_type`, and the tolerance is a version skew,
/// not a spelling choice: an engine that predates the `type` predicate's set widening
/// wrote one lattice name as a bare string, and the row means exactly the one-element
/// set. Reading it is what an upgraded engine owes a lock a prior version committed;
/// the next `emit` writes the file whole in the canonical array form, so the tolerance
/// never becomes a second spelling temper itself emits.
fn opt_str_or_str_array(
    table: &dyn TableLike,
    column: &str,
) -> Result<Option<Vec<String>>, RowError> {
    match table.get(column).and_then(Item::as_str) {
        Some(one) => Ok(Some(vec![one.to_string()])),
        None => opt_str_array(table, column),
    }
}

/// An optional nested table column — absent is `Ok(None)`, a present non-table is a [`RowError`].
fn opt_table<'a>(
    table: &'a dyn TableLike,
    column: &str,
) -> Result<Option<&'a dyn TableLike>, RowError> {
    match table.get(column) {
        None => Ok(None),
        Some(item) => item
            .as_table_like()
            .map(Some)
            .ok_or_else(|| RowError::wrong(column, "table")),
    }
}

impl KindFactRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("name", value(self.name.clone()));
        if let Some(provider) = &self.provider {
            table.insert("provider", value(provider.clone()));
        }
        if let Some(root) = &self.governs_root {
            table.insert("governs_root", value(root.clone()));
        }
        if let Some(glob) = &self.governs_glob {
            table.insert("governs_glob", value(glob.clone()));
        }
        if let Some(commitment) = &self.commitment {
            table.insert("commitment", value(commitment.clone()));
        }
        if let Some(format) = &self.format {
            table.insert("format", value(format.clone()));
        }
        if let Some(unit_shape) = &self.unit_shape {
            table.insert("unit_shape", value(unit_shape.clone()));
        }
        if !self.registration.is_empty() {
            table.insert("registration", value(string_array(&self.registration)));
        }
        if !self.templates.is_empty() {
            table.insert("templates", value(template_array(&self.templates)));
        }
        if let Some(content) = &self.content {
            table.insert("content", value(content_table(content)));
        }
        if let Some(shape) = &self.shape {
            table.insert("shape", value(shape.clone()));
        }
        if let Some(address) = &self.collection_address {
            table.insert(
                "collection_address",
                value(collection_address_table(address)),
            );
        }
        if let Some(guidance) = &self.guidance {
            table.insert("guidance", value(guidance.clone()));
        }
        if let Some(cite) = &self.cite {
            table.insert("cite", value(cite.clone()));
        }
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            name: req_str(table, "name")?,
            provider: opt_str(table, "provider")?,
            governs_root: opt_str(table, "governs_root")?,
            governs_glob: opt_str(table, "governs_glob")?,
            commitment: opt_str(table, "commitment")?,
            format: opt_str(table, "format")?,
            unit_shape: opt_str(table, "unit_shape")?,
            registration: opt_str_array(table, "registration")?.unwrap_or_default(),
            templates: templates_from_table(table)?,
            content: match opt_table(table, "content")? {
                Some(content) => Some(content_from_table(content)?),
                None => None,
            },
            shape: opt_str(table, "shape")?,
            collection_address: match opt_table(table, "collection_address")? {
                Some(address) => Some(collection_address_from_table(address)?),
                None => None,
            },
            guidance: opt_str(table, "guidance")?,
            cite: opt_str(table, "cite")?,
        })
    }

    #[cfg(test)]
    fn round_trip_through_table(&self) -> Result<Self, RowError> {
        let table = self.to_table();
        Self::from_table(&table)
    }
}

/// Build a [`KindFactRow`]'s `templates` column's wire form: an array carrying one
/// inline table per declared template — the child `kind`, plus a file layer's `path`
/// pattern when declared — the same array-of-inline-tables discipline the `content`
/// column's regions take.
fn template_array(templates: &[TemplateRow]) -> Array {
    let mut array = Array::new();
    for template in templates {
        let mut inline = InlineTable::new();
        inline.insert("kind", Value::from(template.kind.clone()));
        if let Some(path) = &template.path {
            inline.insert("path", Value::from(path.clone()));
        }
        array.push(Value::InlineTable(inline));
    }
    array
}

/// Read a `templates` column back off its array — an absent column is an empty set; a
/// present non-array, an element outside both spellings, or a template missing its
/// required `kind` is a [`RowError`], the required-column discipline the rest of the row
/// family holds.
///
/// Two spellings read: the canonical `{ kind = "…", path = "…" }` inline table, and a
/// bare `"…"` string an older engine wrote for a path-less template. The string is the
/// same fact, losslessly — a path-less template is exactly what an admitted embedded
/// kind mints — so it normalizes at read and the next emit rewrites the column
/// canonically; the committed file is never patched in place. An element that is
/// neither is a row no SDK version could have emitted, and still refuses loud.
fn templates_from_table(table: &dyn TableLike) -> Result<Vec<TemplateRow>, RowError> {
    let Some(item) = table.get("templates") else {
        return Ok(Vec::new());
    };
    let array = item
        .as_array()
        .ok_or_else(|| RowError::wrong("templates", "array"))?;
    let mut out = Vec::with_capacity(array.len());
    for element in array.iter() {
        if let Some(kind) = element.as_str() {
            out.push(TemplateRow {
                kind: kind.to_string(),
                path: None,
            });
            continue;
        }
        let inline = element
            .as_inline_table()
            .ok_or_else(|| RowError::wrong("templates", "array of tables or strings"))?;
        out.push(TemplateRow {
            kind: req_str(inline, "kind")?,
            path: opt_str(inline, "path")?,
        });
    }
    Ok(out)
}

/// An `edge` fact's `to` column, read across both spellings a committed lock can carry:
/// the array a current emit writes, and the bare string a pre-set lock wrote — the
/// lossless spelling of the one-element set, read as `["<kind>"]`. The file is never
/// patched; the next emit rewrites it whole in the canonical array form.
///
/// Absent is `Ok(None)` — the column stays optional on the row, and an `edge` fact that
/// omits it is [`crate::main`]'s required-column refusal, not this reader's. A present
/// `to` that is neither a string nor an array of strings is a [`RowError`], so the gate
/// stays tight against a genuinely corrupt lock rather than tolerant of any shape.
fn edge_to_from_table(table: &dyn TableLike) -> Result<Option<Vec<String>>, RowError> {
    let Some(item) = table.get("to") else {
        return Ok(None);
    };
    if let Some(kind) = item.as_str() {
        return Ok(Some(vec![kind.to_string()]));
    }
    opt_str_array(table, "to")
}

/// Build a [`KindFactRow`]'s `collection_address` column's wire form: a `{ manifest =
/// "…", key_path = "…" }` inline table, the presence-coupled pair carried as one column.
fn collection_address_table(address: &CollectionAddressRow) -> InlineTable {
    let mut table = InlineTable::new();
    table.insert("manifest", Value::from(address.manifest.clone()));
    table.insert("key_path", Value::from(address.key_path.clone()));
    table
}

/// Read a `collection_address` column back off its inline table — a missing `manifest` or
/// `key_path` is a [`RowError`], the same required-column discipline the rest of the row
/// family holds.
fn collection_address_from_table(table: &dyn TableLike) -> Result<CollectionAddressRow, RowError> {
    Ok(CollectionAddressRow {
        manifest: req_str(table, "manifest")?,
        key_path: req_str(table, "key_path")?,
        entry_shape: opt_str(table, "entry_shape")?,
    })
}

/// Build a [`KindFactRow`]'s `content` column's wire form: a `{ regions = [...] }` inline
/// table whose array carries one inline table per region, each an order-preserving
/// discriminator + optional columns — the same array-of-inline-tables discipline a
/// [`NestedMemberRow`]'s `collections` column takes.
fn content_table(content: &LayoutRow) -> InlineTable {
    let mut regions = Array::new();
    for region in &content.regions {
        let mut inline = InlineTable::new();
        inline.insert("region", Value::from(region.region.clone()));
        if let Some(import) = &region.import {
            inline.insert("import", Value::from(import.clone()));
        }
        if let Some(slot) = &region.slot {
            inline.insert("slot", Value::from(slot.clone()));
        }
        if let Some(member_kind) = &region.member_kind {
            inline.insert("member_kind", Value::from(member_kind.clone()));
        }
        if let Some(key) = &region.key {
            inline.insert("key", Value::from(key.clone()));
        }
        regions.push(Value::InlineTable(inline));
    }
    let mut table = InlineTable::new();
    table.insert("regions", Value::Array(regions));
    table
}

/// Read a `content` column back off its inline table — an absent `regions` array is
/// empty; a present non-array, a non-inline-table element, or a region missing its
/// required `region` discriminator is a [`RowError`].
fn content_from_table(table: &dyn TableLike) -> Result<LayoutRow, RowError> {
    let regions = match table.get("regions") {
        None => Vec::new(),
        Some(item) => {
            let array = item
                .as_array()
                .ok_or_else(|| RowError::wrong("regions", "array"))?;
            let mut out = Vec::with_capacity(array.len());
            for element in array.iter() {
                let inline = element
                    .as_inline_table()
                    .ok_or_else(|| RowError::wrong("regions", "array of tables"))?;
                out.push(LayoutRegionRow {
                    region: req_str(inline, "region")?,
                    import: opt_str(inline, "import")?,
                    slot: opt_str(inline, "slot")?,
                    member_kind: opt_str(inline, "member_kind")?,
                    key: opt_str(inline, "key")?,
                });
            }
            out
        }
    };
    Ok(LayoutRow { regions })
}

impl ClauseRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        if let Some(label) = &self.label {
            table.insert("label", value(label.clone()));
        }
        if let Some(kind) = &self.kind {
            table.insert("kind", value(kind.clone()));
        }
        table.insert("predicate", value(self.predicate.clone()));
        if let Some(field) = &self.field {
            table.insert("field", value(field.clone()));
        }
        table.insert("severity", value(self.severity.clone()));
        if let Some(guidance) = &self.guidance {
            table.insert("guidance", value(guidance.clone()));
        }
        if let Some(cite) = &self.cite {
            table.insert("cite", value(cite.clone()));
        }
        if let Some(count) = &self.count {
            table.insert("count", value(count_bound_table(count)));
        }
        if let Some(target) = &self.target {
            table.insert("target", value(target.clone()));
        }
        if let Some(degree) = &self.degree {
            table.insert("degree", value(degree_bound_table(degree)));
        }
        if let Some(gate) = &self.gate {
            table.insert("gate", value(gate.clone()));
        }
        if let Some(value_type) = &self.value_type {
            table.insert("value_type", value(string_array(value_type)));
        }
        if let Some(shape) = &self.shape {
            table.insert("shape", value(shape.clone()));
        }
        if let Some(bound) = &self.bound {
            table.insert("bound", value(bound_table(bound)));
        }
        if let Some(unit) = &self.unit {
            table.insert("unit", value(unit.clone()));
        }
        if let Some(charset) = &self.charset {
            table.insert("charset", value(charset_table(charset)));
        }
        if let Some(keys) = &self.keys {
            table.insert("keys", value(string_array(keys)));
        }
        if let Some(values) = &self.values {
            table.insert("values", value(string_array(values)));
        }
        if let Some(range) = &self.range {
            table.insert("range", value(range_bound_table(range)));
        }
        if let Some(section) = &self.section {
            table.insert("section", value(section_contains_table(section)));
        }
        if let Some(sections) = &self.sections {
            table.insert("sections", value(string_array(sections)));
        }
        if let Some(guard_predicate) = &self.guard_predicate {
            table.insert("guard_predicate", value(guard_predicate.clone()));
        }
        if let Some(body) = &self.body {
            let mut array = ArrayOfTables::new();
            for row in body {
                array.push(row.to_table());
            }
            table.insert("body", Item::ArrayOfTables(array));
        }
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            // Required on the way in: emit stamps every row it writes, so a committed
            // row with no label is a corrupt lock, never a row to judge unaddressably.
            label: Some(req_str(table, "label")?),
            kind: opt_str(table, "kind")?,
            predicate: req_str(table, "predicate")?,
            field: opt_str(table, "field")?,
            severity: req_str(table, "severity")?,
            guidance: opt_str(table, "guidance")?,
            cite: opt_str(table, "cite")?,
            count: match opt_table(table, "count")? {
                Some(count) => Some(count_bound_from_table(count)?),
                None => None,
            },
            target: opt_str(table, "target")?,
            degree: match opt_table(table, "degree")? {
                Some(degree) => Some(degree_bound_from_table(degree)?),
                None => None,
            },
            gate: opt_str(table, "gate")?,
            value_type: opt_str_or_str_array(table, "value_type")?,
            shape: opt_str(table, "shape")?,
            bound: match opt_table(table, "bound")? {
                Some(bound) => Some(bound_from_table(bound)?),
                None => None,
            },
            unit: opt_str(table, "unit")?,
            charset: match opt_table(table, "charset")? {
                Some(charset) => Some(charset_from_table(charset)?),
                None => None,
            },
            keys: opt_str_array(table, "keys")?,
            values: opt_str_array(table, "values")?,
            range: match opt_table(table, "range")? {
                Some(range) => Some(range_bound_from_table(range)?),
                None => None,
            },
            section: match opt_table(table, "section")? {
                Some(section) => Some(section_contains_from_table(section)?),
                None => None,
            },
            sections: opt_str_array(table, "sections")?,
            guard_predicate: opt_str(table, "guard_predicate")?,
            body: match table.get("body") {
                None => None,
                Some(item) => {
                    let array = item
                        .as_array_of_tables()
                        .ok_or_else(|| RowError::wrong("body", "array of tables"))?;
                    Some(
                        array
                            .iter()
                            .map(ClauseRow::from_table)
                            .collect::<Result<Vec<_>, _>>()?,
                    )
                }
            },
        })
    }
}

impl RequirementRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("name", value(self.name.clone()));
        if let Some(kind) = &self.kind {
            table.insert("kind", value(kind.clone()));
        }
        table.insert("required", value(self.required));
        if !self.clauses.is_empty() {
            let mut array = ArrayOfTables::new();
            for clause in &self.clauses {
                array.push(clause.to_table());
            }
            table.insert("clauses", Item::ArrayOfTables(array));
        }
        if let Some(verifier) = &self.verifier {
            table.insert("verifier", value(verifier_table(verifier)));
        }
        if let Some(prose) = &self.prose {
            table.insert("prose", value(prose.clone()));
        }
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            name: req_str(table, "name")?,
            kind: opt_str(table, "kind")?,
            required: opt_bool(table, "required")?.unwrap_or(false),
            clauses: match table.get("clauses") {
                None => Vec::new(),
                Some(item) => {
                    let array = item
                        .as_array_of_tables()
                        .ok_or_else(|| RowError::wrong("clauses", "array of tables"))?;
                    array
                        .iter()
                        .map(ClauseRow::from_table)
                        .collect::<Result<Vec<_>, _>>()?
                }
            },
            verifier: match opt_table(table, "verifier")? {
                Some(verifier) => Some(verifier_from_table(verifier)?),
                None => None,
            },
            prose: opt_str(table, "prose")?,
        })
    }
}

/// Serialize a requirement's typed verifier to its species sub-table — `species`
/// tags the variant, then the variant's own payload column (`path` / `events`). The
/// same spelling the serde reader lifts the SDK-emitted JSON through, so the lock and
/// the seam agree byte-for-byte.
fn verifier_table(verifier: &crate::compose::Verifier) -> InlineTable {
    use crate::compose::Verifier;
    let mut table = InlineTable::new();
    match verifier {
        Verifier::Script { path } => {
            table.insert("species", Value::from("script"));
            table.insert("path", Value::from(path.clone()));
        }
        Verifier::Telemetry { events } => {
            table.insert("species", Value::from("telemetry"));
            table.insert("events", Value::Array(string_array(events)));
        }
    }
    table
}

/// Read a verifier back off its species sub-table — an absent or unknown `species`,
/// or a variant missing its payload column, is a [`RowError`] (the lock is
/// tool-written, so a shape the closed species set cannot admit is a corrupt lock).
fn verifier_from_table(table: &dyn TableLike) -> Result<crate::compose::Verifier, RowError> {
    use crate::compose::Verifier;
    match req_str(table, "species")?.as_str() {
        "script" => Ok(Verifier::Script {
            path: req_str(table, "path")?,
        }),
        "telemetry" => Ok(Verifier::Telemetry {
            events: opt_str_array(table, "events")?.unwrap_or_default(),
        }),
        _ => Err(RowError::wrong("species", "`script` or `telemetry`")),
    }
}

/// An optional integer column as a `usize` — absent is `Ok(None)`; a present non-integer
/// or negative value is a [`RowError`].
fn opt_usize(table: &dyn TableLike, column: &str) -> Result<Option<usize>, RowError> {
    let Some(item) = table.get(column) else {
        return Ok(None);
    };
    let raw = item
        .as_integer()
        .ok_or_else(|| RowError::wrong(column, "integer"))?;
    let n = usize::try_from(raw).map_err(|_| RowError::wrong(column, "non-negative integer"))?;
    Ok(Some(n))
}

/// A required integer column as a `usize` — absent is a missing-column [`RowError`].
fn req_usize(table: &dyn TableLike, column: &str) -> Result<usize, RowError> {
    opt_usize(table, column)?.ok_or_else(|| RowError::missing(column))
}

/// A required numeric column as an `f64` — a TOML integer widens to float so an authored
/// `1` reads the same as `1.0`. Absent is a missing-column [`RowError`]; a present
/// non-numeric value is a wrong-type one.
fn req_f64(table: &dyn TableLike, column: &str) -> Result<f64, RowError> {
    let item = table.get(column).ok_or_else(|| RowError::missing(column))?;
    if let Some(float) = item.as_float() {
        Ok(float)
    } else if let Some(int) = item.as_integer() {
        Ok(int as f64)
    } else {
        Err(RowError::wrong(column, "number"))
    }
}

fn range_bound_table(range: &RangeBoundRow) -> InlineTable {
    let mut table = InlineTable::new();
    table.insert("min", Value::from(range.min));
    table.insert("max", Value::from(range.max));
    table
}

fn range_bound_from_table(table: &dyn TableLike) -> Result<RangeBoundRow, RowError> {
    Ok(RangeBoundRow {
        min: req_f64(table, "min")?,
        max: req_f64(table, "max")?,
    })
}

fn section_contains_table(section: &SectionContainsRow) -> InlineTable {
    let mut table = InlineTable::new();
    table.insert("heading", Value::from(section.heading.clone()));
    table.insert("marker", Value::from(section.marker.clone()));
    table
}

fn section_contains_from_table(table: &dyn TableLike) -> Result<SectionContainsRow, RowError> {
    Ok(SectionContainsRow {
        heading: req_str(table, "heading")?,
        marker: req_str(table, "marker")?,
    })
}

fn count_bound_table(count: &CountBoundRow) -> InlineTable {
    let mut table = InlineTable::new();
    table.insert(
        "min",
        Value::from(i64::try_from(count.min).unwrap_or(i64::MAX)),
    );
    table.insert(
        "max",
        Value::from(i64::try_from(count.max).unwrap_or(i64::MAX)),
    );
    table
}

fn count_bound_from_table(table: &dyn TableLike) -> Result<CountBoundRow, RowError> {
    Ok(CountBoundRow {
        min: req_usize(table, "min")?,
        max: req_usize(table, "max")?,
    })
}

fn degree_bound_table(degree: &DegreeBoundRow) -> InlineTable {
    let mut table = InlineTable::new();
    if let Some(incoming) = &degree.incoming {
        table.insert("incoming", Value::InlineTable(edge_bound_table(incoming)));
    }
    if let Some(outgoing) = &degree.outgoing {
        table.insert("outgoing", Value::InlineTable(edge_bound_table(outgoing)));
    }
    table
}

fn degree_bound_from_table(table: &dyn TableLike) -> Result<DegreeBoundRow, RowError> {
    Ok(DegreeBoundRow {
        incoming: match opt_table(table, "incoming")? {
            Some(incoming) => Some(edge_bound_from_table(incoming)?),
            None => None,
        },
        outgoing: match opt_table(table, "outgoing")? {
            Some(outgoing) => Some(edge_bound_from_table(outgoing)?),
            None => None,
        },
    })
}

fn edge_bound_table(bound: &EdgeBoundRow) -> InlineTable {
    let mut table = InlineTable::new();
    if let Some(min) = bound.min {
        table.insert("min", Value::from(i64::try_from(min).unwrap_or(i64::MAX)));
    }
    if let Some(max) = bound.max {
        table.insert("max", Value::from(i64::try_from(max).unwrap_or(i64::MAX)));
    }
    table
}

fn edge_bound_from_table(table: &dyn TableLike) -> Result<EdgeBoundRow, RowError> {
    Ok(EdgeBoundRow {
        min: opt_usize(table, "min")?,
        max: opt_usize(table, "max")?,
    })
}

fn bound_table(bound: &BoundRow) -> InlineTable {
    let mut table = InlineTable::new();
    if let Some(min) = bound.min {
        table.insert("min", Value::from(i64::try_from(min).unwrap_or(i64::MAX)));
    }
    if let Some(max) = bound.max {
        table.insert("max", Value::from(i64::try_from(max).unwrap_or(i64::MAX)));
    }
    table
}

fn bound_from_table(table: &dyn TableLike) -> Result<BoundRow, RowError> {
    Ok(BoundRow {
        min: opt_usize(table, "min")?,
        max: opt_usize(table, "max")?,
    })
}

fn charset_table(charset: &CharsetRow) -> InlineTable {
    let mut table = InlineTable::new();
    if !charset.ranges.is_empty() {
        table.insert("ranges", Value::Array(string_array(&charset.ranges)));
    }
    if let Some(chars) = &charset.chars {
        table.insert("chars", Value::from(chars.clone()));
    }
    table
}

fn charset_from_table(table: &dyn TableLike) -> Result<CharsetRow, RowError> {
    Ok(CharsetRow {
        ranges: opt_str_array(table, "ranges")?.unwrap_or_default(),
        chars: opt_str(table, "chars")?,
    })
}

/// Build a TOML array off owned strings — the `keys`/`values`/charset-`ranges`
/// columns' wire form.
fn string_array(values: &[String]) -> Array {
    let mut array = Array::new();
    for value in values {
        array.push(value.clone());
    }
    array
}

impl AssemblyFactRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("fact", value(self.fact.clone()));
        if let Some(value_col) = &self.value {
            table.insert("value", value(value_col.clone()));
        }
        if let Some(from) = &self.from {
            table.insert("from", value(from.clone()));
        }
        if let Some(field) = &self.field {
            table.insert("field", value(field.clone()));
        }
        if let Some(to) = &self.to {
            table.insert("to", value(Array::from_iter(to.iter().cloned())));
        }
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            fact: req_str(table, "fact")?,
            value: opt_str(table, "value")?,
            from: opt_str(table, "from")?,
            field: opt_str(table, "field")?,
            to: edge_to_from_table(table)?,
        })
    }
}

impl SatisfiesRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("member", value(self.member.clone()));
        table.insert("requirement", value(self.requirement.clone()));
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            member: req_str(table, "member")?,
            requirement: req_str(table, "requirement")?,
        })
    }
}

impl MentionRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("member", value(self.member.clone()));
        table.insert("target", value(self.target.clone()));
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            member: req_str(table, "member")?,
            target: req_str(table, "target")?,
        })
    }
}

impl RegistrationRow {
    /// The lock row records identity and collection address only — the folded `fields` are
    /// the projected manifest artifact, never a second copy the engine reads back (0018).
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("kind", value(self.kind.clone()));
        table.insert("key", value(self.key.clone()));
        table.insert("manifest", value(self.manifest.clone()));
        table.insert("key_path", value(self.key_path.clone()));
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            kind: req_str(table, "kind")?,
            key: req_str(table, "key")?,
            manifest: req_str(table, "manifest")?,
            key_path: req_str(table, "key_path")?,
            // The fields live in the projected manifest artifact, so a lock row carries none.
            fields: Vec::new(),
        })
    }
}

impl NestedMemberRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("host", value(self.host.clone()));
        table.insert("kind", value(self.kind.clone()));
        table.insert("key", value(self.key.clone()));
        if !self.leaves.is_empty() {
            table.insert("leaves", value(string_map_table(&self.leaves)));
        }
        if !self.collections.is_empty() {
            table.insert("collections", value(collections_array(&self.collections)));
        }
        if let Some(placed) = &self.placed_edges {
            table.insert("placed_edges", value(string_array(placed)));
        }
        if let Some(lines) = self.rendered_lines {
            table.insert(
                "rendered_lines",
                value(i64::try_from(lines).unwrap_or(i64::MAX)),
            );
        }
        if let Some(chars) = self.rendered_chars {
            table.insert(
                "rendered_chars",
                value(i64::try_from(chars).unwrap_or(i64::MAX)),
            );
        }
        table
    }

    fn from_table(table: &Table) -> Result<Self, RowError> {
        Ok(Self {
            host: req_str(table, "host")?,
            kind: req_str(table, "kind")?,
            key: req_str(table, "key")?,
            placed_edges: opt_str_array(table, "placed_edges")?,
            rendered_lines: opt_usize(table, "rendered_lines")?,
            rendered_chars: opt_usize(table, "rendered_chars")?,
            leaves: match opt_table(table, "leaves")? {
                Some(leaves) => string_map_from_table(leaves)?,
                None => BTreeMap::new(),
            },
            collections: match table.get("collections") {
                None => Vec::new(),
                Some(item) => {
                    let array = item
                        .as_array()
                        .ok_or_else(|| RowError::wrong("collections", "array"))?;
                    collections_from_array(array)?
                }
            },
        })
    }
}

/// Build an inline table off an owned string map — a [`NestedMemberRow`]'s `leaves`
/// column's wire form.
fn string_map_table(map: &BTreeMap<String, String>) -> InlineTable {
    let mut table = InlineTable::new();
    for (key, text) in map {
        table.insert(key.as_str(), Value::from(text.clone()));
    }
    table
}

/// Read a string map back off a declaration row column — a non-string value under any
/// key is a [`RowError`] naming that key.
fn string_map_from_table(table: &dyn TableLike) -> Result<BTreeMap<String, String>, RowError> {
    let mut out = BTreeMap::new();
    for (key, item) in table.iter() {
        let text = item
            .as_str()
            .ok_or_else(|| RowError::wrong(key, "string"))?;
        out.insert(key.to_string(), text.to_string());
    }
    Ok(out)
}

/// Build a [`NestedMemberRow`]'s `collections` column's wire form: an
/// order-preserving array of `{collection, key, leaves}` inline tables, one per
/// entry — the same array-shaped discipline the other declaration families get
/// from an `[[declaration.<family>]]` array-of-tables, one level further in since
/// this column lives inside a single row rather than at the top of the lock.
fn collections_array(collections: &[CollectionEntryRow]) -> Array {
    let mut array = Array::new();
    for entry in collections {
        let mut inline = InlineTable::new();
        inline.insert("collection", Value::from(entry.collection.clone()));
        inline.insert("key", Value::from(entry.key.clone()));
        inline.insert(
            "leaves",
            Value::InlineTable(string_map_table(&entry.leaves)),
        );
        array.push(Value::InlineTable(inline));
    }
    array
}

/// Read a `collections` column back off its order-preserving array — a non-inline-table
/// element, or one missing its required `collection`/`key` column, is a [`RowError`].
fn collections_from_array(array: &Array) -> Result<Vec<CollectionEntryRow>, RowError> {
    let mut out = Vec::with_capacity(array.len());
    for element in array.iter() {
        let inline = element
            .as_inline_table()
            .ok_or_else(|| RowError::wrong("collections", "array of tables"))?;
        out.push(CollectionEntryRow {
            collection: req_str(inline, "collection")?,
            key: req_str(inline, "key")?,
            leaves: match opt_table(inline, "leaves")? {
                Some(leaves) => string_map_from_table(leaves)?,
                None => BTreeMap::new(),
            },
        });
    }
    Ok(out)
}

/// Lift [`NestedMemberRow`]s into typed [`crate::extract::EmbeddedMember`]s that match
/// a given host address. Filters rows to the host's address and applies
/// `embedded_member_from_row` to each, expanding one-layer-deep collection nesting and
/// projecting to the typed member shape. The projection mirrors the retired fence fold:
/// a row's flat, ordered `collections` column expands into nested members, one per
/// entry, in order.
///
/// A row can only be read off the lock file, which is written by code under our
/// control (the `emit` crate). Row data never enters from user input — the address
/// match admits only rows a prior harness authored (never hand-edited,
/// "not the database"): a row exists only because an SDK program declared it, so the
/// address match is the whole admissibility check, no declared-template leniency layer
/// on top.
#[must_use]
pub(crate) fn nested_members_from_rows(
    host: &str,
    rows: &[NestedMemberRow],
) -> Vec<crate::extract::EmbeddedMember> {
    rows.iter()
        .filter(|row| row.host == host)
        .map(embedded_member_from_row)
        .collect()
}

/// Lift one [`NestedMemberRow`] into its typed
/// [`crate::extract::EmbeddedMember`]: the row's flat, ordered `collections` column expands one
/// layer deep into nested `EmbeddedMember`s, one per entry, in the row's own
/// order — the same one-layer shape the retired fence fold produced.
fn embedded_member_from_row(row: &NestedMemberRow) -> crate::extract::EmbeddedMember {
    use crate::extract::EmbeddedMember;
    EmbeddedMember {
        kind: row.kind.clone(),
        key: row.key.clone(),
        leaves: row.leaves.clone(),
        members: row
            .collections
            .iter()
            .map(|entry| crate::extract::EmbeddedMemberCollectionEntry {
                collection: entry.collection.clone(),
                key: entry.key.clone(),
                member: EmbeddedMember {
                    kind: entry.collection.clone(),
                    key: entry.key.clone(),
                    leaves: entry.leaves.clone(),
                    members: Vec::new(),
                },
            })
            .collect(),
    }
}

pub fn requirement_from_row(
    row: &RequirementRow,
) -> Result<compose::Requirement, compose::ClauseRowError> {
    Ok(compose::Requirement {
        name: row.name.clone(),
        prose: row.prose.clone(),
        kind: row.kind.clone(),
        required: row.required,
        clauses: row
            .clauses
            .iter()
            .map(clause_from_row)
            .collect::<Result<Vec<_>, _>>()?,
        verifier: row.verifier.clone(),
    })
}

/// Lift one of a requirement row's nested [`ClauseRow`]s into a
/// [`contract::Clause`] — the mirror of [`requirement_from_row`] for the set-/edge-scope
/// demand it carries, via the shared [`compose::clause_from_row`] lift. A
/// requirement-nested row's guidance/source isn't carried the same way as a
/// kind-level clause's, so both are overwritten to `None` on success rather than
/// passed through.
///
/// # Errors
///
/// Propagates the [`compose::ClauseRowError`] the shared lift raises for a row the
/// closed vocabulary cannot admit — rejected loud, never a silently dropped clause.
pub fn clause_from_row(row: &ClauseRow) -> Result<contract::Clause, compose::ClauseRowError> {
    compose::clause_from_row(row).map(|clause| contract::Clause {
        guidance: None,
        source: None,
        ..clause
    })
}

/// The assembly's declared edges off the lock's `assembly` fact family — every
/// `fact = "edge"` row. A present edge row missing a required `field`/`from`/`to` column
/// is a load error naming the assembly family, never a silently absent edge — as is a
/// `to` that names no kind at all.
///
/// # Errors
///
/// Returns a [`LockRowError`] when a present edge fact omits a required column or
/// declares an empty target set.
pub fn edges_from_declarations(
    declarations: &Declarations,
) -> Result<Vec<compose::Edge>, LockRowError> {
    declarations
        .assembly
        .iter()
        .filter(|fact| fact.fact == "edge")
        .map(|fact| {
            let to: Vec<String> = edge_column(fact.to.clone(), "to")?;
            // An edge declaring no target kind can never resolve — loading it would
            // silently narrow the gate to a route it can never judge.
            if to.is_empty() {
                return Err(LockRowError::WrongType {
                    family: "assembly".to_string(),
                    column: "to".to_string(),
                    want: "non-empty set of target kinds".to_string(),
                });
            }
            Ok(compose::Edge {
                field: edge_column(fact.field.clone(), "field")?,
                from: edge_column(fact.from.clone(), "from")?,
                to,
            })
        })
        .collect()
}

/// One required column off a present `edge` assembly fact — an absent one is a load error
/// naming the assembly family, the same reject a malformed row takes at load.
fn edge_column<T>(value: Option<T>, column: &str) -> Result<T, LockRowError> {
    value.ok_or_else(|| LockRowError::MissingColumn {
        family: "assembly".to_string(),
        column: column.to_string(),
    })
}

/// The lock's already-resolved `mention` rows, lifted into [`graph::ResolvedEdge`]s —
/// the mention-family mirror of [`edges_from_declarations`]: no field lookup (a mention
/// is resolved once, at emit), just the address parse [`graph::resolved_mention_edges`]
/// runs.
pub fn mention_edges_from_declarations(declarations: &Declarations) -> Vec<graph::ResolvedEdge> {
    let mentions: Vec<graph::MentionDeclaration> = declarations
        .mentions
        .iter()
        .map(|row| graph::MentionDeclaration {
            member: row.member.clone(),
            target: row.target.clone(),
        })
        .collect();
    graph::resolved_mention_edges(&mentions)
}

/// Extract import edges from an already-parsed lock document, avoiding redundant
/// reads/parses.
///
/// # Errors
///
/// Returns a [`DriftError`] when a present source-dependency row is malformed.
pub fn import_edges_from_doc(doc: &DocumentMut) -> miette::Result<Vec<graph::ResolvedEdge>> {
    let layouts = layout_imports_from_doc(doc)?;
    let includes = includes_from_doc(doc)?;
    let imports: Vec<graph::ImportDeclaration> = layouts
        .into_iter()
        .chain(includes)
        .filter(|row| !row.target.is_empty())
        .map(|row| graph::ImportDeclaration {
            member: row.member,
            target: row.target,
        })
        .collect();
    Ok(graph::resolved_import_edges(&imports))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::tmpdir;

    #[test]
    fn to_lock_path_normalizes_a_backslash_joined_path() {
        // A Windows `Path::join` inserts `\` at the join boundary; simulate that
        // shape directly (Unix `Path` never inserts `\`, so a real join can't
        // reproduce it here) and assert the lock row still comes out `/`-separated.
        let path = PathBuf::from("harness\\dir\\file.md");
        assert_eq!(to_lock_path(&path), "harness/dir/file.md");
    }

    #[test]
    fn project_bytes_heads_a_frontmatterless_body_with_its_preserved_banner() {
        let banner =
            "<!-- temper: managed projection — edit the owning .temper/ module. -->".to_string();
        let body = "# Project\n\nMemory body.\n";

        // With no placements a frontmatterless projection is its body alone; with
        // install's banner it heads the body, one blank line between — the bytes
        // install placed, so a re-emit reports Unchanged instead of stripping it.
        assert_eq!(project_bytes(None, &[], body, &[]).as_deref(), Some(body));
        assert_eq!(
            project_bytes(None, &[], body, std::slice::from_ref(&banner)),
            Some(format!("{banner}\n\n{body}"))
        );
    }

    #[test]
    fn project_bytes_renders_nothing_for_a_read_only_format() {
        // The write dispatch's floor: a `toml-document` member has no write face, and the
        // fall-through below this arm would otherwise hand it a frontmatter block — a
        // member's fields on disk in a format its author never declared. `None` is what
        // makes each caller raise its own refusal instead.
        assert_eq!(
            project_bytes(
                Some(Format::TomlDocument),
                &[("name".to_string(), JsonValue::from("dial"))],
                "",
                &[]
            ),
            None
        );
    }

    #[test]
    fn place_creates_an_absent_target() {
        let dir = tmpdir("place-absent");
        let target = dir.join("nested").join("settings.json");

        // Absent target: written (creating parent dirs) and reported Applied.
        let outcome = place(&target, "{}\n", None, false).unwrap();
        assert_eq!(outcome, ApplyOutcome::Applied);
        assert_eq!(fs::read_to_string(&target).unwrap(), "{}\n");
    }

    #[test]
    fn place_is_idempotent_and_dry_run_writes_nothing() {
        let dir = tmpdir("place-idem");
        let target = dir.join("workflow.yml");
        place(&target, "name: temper\n", None, false).unwrap();

        // A re-place of the same bytes is the idempotent no-op.
        assert_eq!(
            place(&target, "name: temper\n", None, false).unwrap(),
            ApplyOutcome::Unchanged
        );

        // A dry run of a differing projection reports Applied but writes nothing.
        let outcome = place(&target, "name: changed\n", None, true).unwrap();
        assert_eq!(outcome, ApplyOutcome::Applied);
        assert_eq!(fs::read_to_string(&target).unwrap(), "name: temper\n");
    }

    #[test]
    fn place_conflicts_only_against_a_recorded_baseline() {
        let dir = tmpdir("place-conflict");
        let target = dir.join("file.txt");
        fs::write(&target, "human wrote this").unwrap();
        let baseline = sha256_hex(b"temper last wrote this");

        // The on-disk bytes no longer hash to the recorded baseline, and desired
        // differs too: a genuine world drift, surfaced rather than clobbered.
        let outcome = place(&target, "temper wants this", Some(&baseline), false).unwrap();
        assert_eq!(outcome, ApplyOutcome::Conflicted);
        assert_eq!(fs::read_to_string(&target).unwrap(), "human wrote this");

        // With no baseline the same differing projection is a clean merge target.
        let outcome = place(&target, "temper wants this", None, false).unwrap();
        assert_eq!(outcome, ApplyOutcome::Applied);
        assert_eq!(fs::read_to_string(&target).unwrap(), "temper wants this");
    }

    #[test]
    fn place_ignores_eol_only_changes_against_a_baseline() {
        let dir = tmpdir("place-eol");
        let target = dir.join("file.txt");
        let lf_body = "line one\nline two\n";
        let crlf_body = "line one\r\nline two\r\n";

        // Baseline is hashed from LF content.
        let baseline = sha256_hex(lf_body.as_bytes());
        fs::write(&target, crlf_body).unwrap();

        // The on-disk CRLF bytes, when canonicalized to LF, match the baseline:
        // no conflict, clean apply.
        let outcome = place(&target, lf_body, Some(&baseline), false).unwrap();
        assert_eq!(outcome, ApplyOutcome::Applied);
        assert_eq!(fs::read_to_string(&target).unwrap(), lf_body);

        // A genuine (non-EOL) edit still conflicts.
        fs::write(&target, "human edited this").unwrap();
        let outcome = place(&target, lf_body, Some(&baseline), false).unwrap();
        assert_eq!(outcome, ApplyOutcome::Conflicted);
        assert_eq!(fs::read_to_string(&target).unwrap(), "human edited this");
    }

    #[test]
    fn strip_verbatim_prefix_strips_the_windows_disk_form() {
        let stripped = strip_verbatim_prefix(Path::new(r"\\?\C:\repo\.temper\harness.ts"));
        assert_eq!(stripped, PathBuf::from(r"C:\repo\.temper\harness.ts"));
    }

    #[test]
    fn strip_verbatim_prefix_strips_the_windows_unc_form() {
        let stripped = strip_verbatim_prefix(Path::new(r"\\?\UNC\server\share\harness.ts"));
        assert_eq!(stripped, PathBuf::from(r"\\server\share\harness.ts"));
    }

    #[test]
    fn strip_verbatim_prefix_leaves_a_non_verbatim_path_untouched() {
        let stripped = strip_verbatim_prefix(Path::new("/repo/.temper/harness.ts"));
        assert_eq!(stripped, PathBuf::from("/repo/.temper/harness.ts"));
    }

    fn decision_row() -> NestedMemberRow {
        NestedMemberRow {
            host: "decision:05-surface-authority".to_string(),
            kind: "decision".to_string(),
            key: "surface-authority".to_string(),
            leaves: BTreeMap::from([(
                "chosen".to_string(),
                "the composition surface is canonical".to_string(),
            )]),
            collections: vec![CollectionEntryRow {
                collection: "rejected".to_string(),
                key: "baked-projection".to_string(),
                leaves: BTreeMap::from([(
                    "because".to_string(),
                    "a stamping projector breaks law 5".to_string(),
                )]),
            }],
            placed_edges: None,
            rendered_lines: None,
            rendered_chars: None,
        }
    }

    #[test]
    fn nested_members_from_rows_matches_only_the_named_host_address() {
        let rows = vec![decision_row()];

        let matched = nested_members_from_rows("decision:05-surface-authority", &rows);
        assert_eq!(matched.len(), 1);
        assert!(nested_members_from_rows("decision:some-other", &rows).is_empty());
    }

    #[test]
    fn nested_members_from_rows_lifts_leaves_and_one_layer_of_collections() {
        let rows = vec![decision_row()];
        let members = nested_members_from_rows("decision:05-surface-authority", &rows);

        let member = &members[0];
        assert_eq!(member.kind, "decision");
        assert_eq!(member.key, "surface-authority");
        assert_eq!(
            member.leaves.get("chosen").map(String::as_str),
            Some("the composition surface is canonical")
        );

        let entry = member
            .members
            .iter()
            .find(|entry| entry.collection == "rejected" && entry.key == "baked-projection")
            .expect("the collection entry is lifted");
        assert_eq!(
            entry.member.leaves.get("because").map(String::as_str),
            Some("a stamping projector breaks law 5")
        );
    }

    #[test]
    fn kind_fact_row_with_guidance_and_cite_round_trips_unchanged() {
        let row = KindFactRow {
            name: "rule".to_string(),
            provider: None,
            governs_root: Some(".claude/rules".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: Some("yaml-frontmatter".to_string()),
            unit_shape: Some("file".to_string()),
            registration: vec!["paths-match(paths)".to_string()],
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
            guidance: Some("Best practices for authoring rules.".to_string()),
            cite: Some("code.claude.com/docs/rules, retrieved 2026-07-20".to_string()),
        };

        let round_tripped = row
            .round_trip_through_table()
            .expect("round-trip succeeded");
        assert_eq!(row, round_tripped);
    }

    #[test]
    fn kind_fact_row_omits_absent_guidance_and_cite_columns() {
        let row = KindFactRow {
            name: "rule".to_string(),
            provider: None,
            governs_root: Some(".claude/rules".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: Some("yaml-frontmatter".to_string()),
            unit_shape: Some("file".to_string()),
            registration: vec!["paths-match(paths)".to_string()],
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
            guidance: None,
            cite: None,
        };

        let table = row.to_table();
        assert!(!table.contains_key("guidance"));
        assert!(!table.contains_key("cite"));

        let round_tripped = row
            .round_trip_through_table()
            .expect("round-trip succeeded");
        assert_eq!(row, round_tripped);
    }

    #[test]
    fn peek_and_validate_seam_version_accepts_matching_version() {
        let json = r#"{"version": 2, "declarations": {}, "members": []}"#;
        assert!(peek_and_validate_seam_version(json).is_ok());
    }

    #[test]
    fn peek_and_validate_seam_version_rejects_mismatched_version() {
        let json = r#"{"version": 999, "declarations": {}, "members": []}"#;
        let err = peek_and_validate_seam_version(json).unwrap_err();
        assert!(format!("{err}").contains("999"));
        assert!(format!("{err}").contains(&SEAM_VERSION.to_string()));
    }

    #[test]
    fn peek_and_validate_seam_version_rejects_mismatched_version_with_incompatible_shape() {
        let json = r#"{"version": 999, "invalid": "shape"}"#;
        let err = peek_and_validate_seam_version(json).unwrap_err();
        assert!(format!("{err}").contains("999"));
        assert!(format!("{err}").contains(&SEAM_VERSION.to_string()));
    }
}
