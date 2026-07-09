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

use crate::hash::sha256_hex;
use crate::import::{RollupEntry, write_rollup};
use crate::install;

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
}

// ---------------------------------------------------------------------------
// emit — the write direction
// ---------------------------------------------------------------------------

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
}

impl EmitOutcome {
    /// The lower-case label used in the rendered report and stable for tests.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            EmitOutcome::Emitted => "emitted",
            EmitOutcome::Unchanged => "unchanged",
            EmitOutcome::Reaped => "reaped",
            EmitOutcome::OrphanDrift => "orphan-drift",
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
/// member that lives at a path locus (`sdk/src/emit.ts` `PayloadMember`). An
/// embedded member never appears here (it carries no standalone projection); its
/// facts ride the [`NestedMemberRow`] family instead.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PayloadMember {
    /// The kind's bare name — joins this payload's own `declarations.kinds` family.
    pub kind: String,
    /// Identity within the kind.
    pub name: String,
    /// The kind's typed fields, flat and ordered — the projected frontmatter.
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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Payload {
    /// The pinned seam version this payload was compiled against.
    pub version: u32,
    /// The seven declaration families.
    pub declarations: Declarations,
    /// Every projected member.
    pub members: Vec<PayloadMember>,
}

/// The desired projection of one member: its identity, the harness-rooted path it
/// projects to, and its fields/body.
struct Projection {
    kind: String,
    name: String,
    source_path: PathBuf,
    /// The desired header fields in canonical order (known fields first, then the
    /// preserved unknown keys). The whole set is re-emitted into a fresh
    /// frontmatter block — the projection is regenerated, never patched.
    fields: Vec<(String, JsonValue)>,
    /// The desired body — the surface body, projected byte-faithfully.
    body: String,
}

/// Render a path for a lock row's `source_path`: always `/`-separated, regardless
/// of host. `lock.toml` is committed, and `Path::join` inserts the host separator
/// at each join boundary (backslash on Windows) — left alone, that forks the
/// byte-committed lock by host.
pub(crate) fn to_lock_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// The harness-relative locus a member of `facts` named `name` projects onto — the
/// Rust port of the retired SDK `projectionPath`: a directory unit lands
/// its entry file under `<root>/<name>/`; a lone file replaces the glob's `*` with the
/// name (an any-depth glob, a memory kind's `**/CLAUDE.md`, lands the root `<name>.md`).
fn member_projection_path(facts: &KindFactRow, name: &str) -> PathBuf {
    let relative = if facts.unit_shape.as_deref() == Some("directory") {
        let entry = facts
            .governs_glob
            .split_once('/')
            .map_or(facts.governs_glob.as_str(), |(_, rest)| rest);
        format!("{name}/{entry}")
    } else if facts.governs_glob.contains("**") {
        format!("{name}.md")
    } else {
        facts.governs_glob.replacen('*', name, 1)
    };
    if facts.governs_root == "." {
        PathBuf::from(relative)
    } else {
        Path::new(&facts.governs_root).join(relative)
    }
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

    let harness_root = workspace_dir.parent().unwrap_or_else(|| Path::new("."));
    let kind_facts: BTreeMap<&str, &KindFactRow> = payload
        .declarations
        .kinds
        .iter()
        .map(|row| (row.name.as_str(), row))
        .collect();

    let mut projections = Vec::with_capacity(payload.members.len());
    for member in &payload.members {
        let facts =
            kind_facts
                .get(member.kind.as_str())
                .ok_or_else(|| DriftError::UnknownKind {
                    kind: member.kind.clone(),
                    member: member.name.clone(),
                })?;
        let source_path = harness_root.join(member_projection_path(facts, &member.name));
        projections.push(Projection {
            kind: member.kind.clone(),
            name: member.name.clone(),
            source_path,
            fields: member.fields.clone(),
            body: member.body.clone(),
        });
    }

    let mut entries = Vec::with_capacity(projections.len());
    let mut rollups: BTreeMap<String, Vec<RollupEntry>> = BTreeMap::new();
    for projection in &projections {
        let (entry, hash) = emit_one(projection, options.dry_run)?;
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

    // Total runs in reverse too: a member the prior lock knew and the current
    // payload no longer owns leaves its projection stranded on disk unless emit
    // reaps it here. The new lock is about to be rewritten whole from `rollups`
    // alone, so this is the one point where a dropped member's row is still on
    // hand to compare against.
    let owned_paths: BTreeSet<String> = projections
        .iter()
        .map(|projection| to_lock_path(&projection.source_path))
        .collect();
    for row in read_prior_provenance(workspace_dir) {
        if owned_paths.contains(&row.source_path) {
            continue;
        }
        if let Some(entry) = reap_or_report_orphan(&row, options.dry_run)? {
            entries.push(entry);
        }
    }

    if !options.dry_run {
        write_rollup(
            workspace_dir,
            &rollups,
            &BTreeMap::new(),
            &payload.declarations,
        )?;
    }

    Ok(EmitReport { entries })
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

/// Every provenance row the prior lock at `workspace_dir` carries, across every
/// kind (built-in and custom) — the anchor [`emit`]'s reap step diffs the current
/// payload's owned paths against to find a lock-known projection with no current
/// owner. A row missing a required column, or a missing/malformed lock, yields no
/// rows — the same tolerant-read absence [`config_stale`]/[`emit_owned_targets`]
/// take: nothing to compare against forges no reap, no drift finding.
fn read_prior_provenance(workspace_dir: &Path) -> Vec<ProvenanceRow> {
    let path = workspace_dir.join("lock.toml");
    let Ok(text) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let Ok(doc) = text.parse::<DocumentMut>() else {
        return Vec::new();
    };

    let mut rows = Vec::new();
    for (kind, item) in doc.as_table().iter() {
        let Some(table_rows) = item.as_array_of_tables() else {
            continue;
        };
        for row in table_rows.iter() {
            let (Some(name), Some(source_path), Some(emit_hash)) = (
                row.get("name").and_then(Item::as_str),
                row.get("source_path").and_then(Item::as_str),
                row.get("emit_hash").and_then(Item::as_str),
            ) else {
                continue;
            };
            rows.push(ProvenanceRow {
                kind: kind.to_string(),
                name: name.to_string(),
                source_path: source_path.to_string(),
                emit_hash: emit_hash.to_string(),
            });
        }
    }
    rows
}

/// Reap or report one lock-known projection whose owning member is gone: the
/// on-disk bytes are hashed and compared against the row's recorded `emit_hash`
/// — the safety line that keeps a hand-edited file from ever being silently
/// deleted (temper wrote every byte of a matching file, so removing it, or under
/// `--dry-run` reporting that it would be removed, loses nothing authored; a
/// mismatch leaves the file in place and reports the drift instead). A file
/// already absent is neither reaped nor reported: there is nothing left to act
/// on, so this returns `None`.
fn reap_or_report_orphan(
    row: &ProvenanceRow,
    dry_run: bool,
) -> Result<Option<EmitEntry>, DriftError> {
    let path = PathBuf::from(&row.source_path);
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(DriftError::Read { path, source }),
    };

    let outcome = if sha256_hex(&bytes) == row.emit_hash {
        if !dry_run {
            fs::remove_file(&path).map_err(|source| DriftError::Remove {
                path: path.clone(),
                source,
            })?;
        }
        EmitOutcome::Reaped
    } else {
        EmitOutcome::OrphanDrift
    };
    Ok(Some(EmitEntry {
        kind: row.kind.clone(),
        name: row.name.clone(),
        source_path: path,
        outcome,
    }))
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
fn emit_one(projection: &Projection, dry_run: bool) -> Result<(EmitEntry, String), DriftError> {
    let row = |outcome| EmitEntry {
        kind: projection.kind.clone(),
        name: projection.name.clone(),
        source_path: projection.source_path.clone(),
        outcome,
    };

    // Read the committed projection first — never to merge authored content, but to
    // tell `Emitted` from the idempotent no-op *and* to carry install's frontmatter
    // placements (the schema modeline, the managed-by note) through the whole-file
    // re-emit. Those metadata lines ride `install`, never `emit`, so a re-emit round-trips the ones
    // already on disk instead of clobbering them. An absent source carries no
    // placements and is not a conflict: emit writes it.
    let current = match fs::read(&projection.source_path) {
        Ok(bytes) => Some(bytes),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(source) => {
            return Err(DriftError::Read {
                path: projection.source_path.clone(),
                source,
            });
        }
    };
    let placements = current
        .as_deref()
        .map(|bytes| install::placement_lines(&String::from_utf8_lossy(bytes)))
        .unwrap_or_default();

    let desired = normalize_lf(&project_bytes(
        &projection.fields,
        &projection.body,
        &placements,
    ));

    // Double-emit determinism: a second
    // projection over the same surface must be byte-identical. Nondeterministic
    // authoring (a timestamp, an unordered map surfacing into a field) is a loud
    // failure here, never a silent churn the next `emit` would rewrite.
    let second_pass = normalize_lf(&project_bytes(
        &projection.fields,
        &projection.body,
        &placements,
    ));
    if second_pass != desired {
        return Err(DriftError::Nondeterministic {
            path: projection.source_path.clone(),
        });
    }

    let hash = sha256_hex(desired.as_bytes());
    if current.as_deref() == Some(desired.as_bytes()) {
        return Ok((row(EmitOutcome::Unchanged), hash));
    }

    if !dry_run {
        if let Some(parent) = projection.source_path.parent() {
            fs::create_dir_all(parent).map_err(|source| DriftError::Write {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        fs::write(&projection.source_path, desired.as_bytes()).map_err(|source| {
            DriftError::Write {
                path: projection.source_path.clone(),
                source,
            }
        })?;
    }
    Ok((row(EmitOutcome::Emitted), hash))
}

/// Re-emit the desired projection deterministically: a fresh `---`-delimited
/// frontmatter block carrying install's preserved `placements` (the schema modeline,
/// the managed-by note — in on-disk order), then every desired field in order, then
/// the surface body byte-for-byte.
///
/// The authored content is *generated*, not patched — a hand-edited
/// field is not preserved (that is drift, routed to the authored source). Install's
/// metadata comments are the one exception the caller feeds in: they ride `install`,
/// never `emit`, so emit round-trips the ones
/// already on disk rather than dropping them. An artifact with no fields (a rule that
/// carries no `paths`/unknown keys) projects to its body alone — no frontmatter block,
/// and so no place a modeline/note could have been installed.
fn project_bytes(fields: &[(String, JsonValue)], body: &str, placements: &[String]) -> String {
    if fields.is_empty() {
        return body.to_string();
    }
    let mut frontmatter = String::new();
    for line in placements {
        frontmatter.push_str(line);
        frontmatter.push('\n');
    }
    for (key, value) in fields {
        frontmatter.push_str(&render_field(key, value));
    }
    format!("---\n{frontmatter}---\n{body}")
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
    let (mut emitted, mut unchanged, mut reaped, mut orphan_drift) = (0u32, 0u32, 0u32, 0u32);
    for entry in &report.entries {
        match entry.outcome {
            EmitOutcome::Emitted => emitted += 1,
            EmitOutcome::Unchanged => unchanged += 1,
            EmitOutcome::Reaped => reaped += 1,
            EmitOutcome::OrphanDrift => orphan_drift += 1,
        }
        out.push_str(&format!(
            "{:<10}  {:<5}  {}\n",
            entry.outcome.label(),
            entry.kind,
            entry.name
        ));
    }
    out.push_str(&format!(
        "\n{emitted} emitted, {unchanged} unchanged, {reaped} reaped, {orphan_drift} orphan-drift\n"
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
    let drifted_from_baseline = last_applied.is_some_and(|baseline| sha256_hex(&real) != baseline);
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
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| DriftError::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, desired.as_bytes()).map_err(|source| DriftError::Write {
        path: path.to_path_buf(),
        source,
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
    let path = workspace_dir.join("lock.toml");
    let Ok(text) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let Ok(doc) = text.parse::<DocumentMut>() else {
        return Vec::new();
    };

    let mut findings = Vec::new();
    // The lock's top-level keys are kind names, each an array of provenance rows; ranging
    // over every one covers built-in and custom kinds alike without a hardcoded set.
    for (_kind, item) in doc.as_table().iter() {
        let Some(rows) = item.as_array_of_tables() else {
            continue;
        };
        for row in rows.iter() {
            let (Some(name), Some(source_path), Some(emit_hash)) = (
                row.get("name").and_then(Item::as_str),
                row.get("source_path").and_then(Item::as_str),
                row.get("emit_hash").and_then(Item::as_str),
            ) else {
                continue;
            };
            // Only a present-and-differing projection is stale: a source that is gone
            // (or otherwise unreadable) is the `removed`/drift axis, never forged here.
            let Ok(bytes) = fs::read(source_path) else {
                continue;
            };
            if sha256_hex(&bytes) != emit_hash {
                findings.push(crate::check::Diagnostic::warn(
                    CONFIG_STALE_RULE,
 source_path,
                    format!(
                        "committed projection `{source_path}` (member `{name}`) does not match the lock's emit fingerprint — the authored source changed and `emit` has not run, or the projection was hand-edited; re-emit to reconcile"
                    ),
 ));
            }
        }
    }
    findings
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
    /// The projected artifact's on-disk path.
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
    let path = workspace_dir.join("lock.toml");
    let Ok(text) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let Ok(doc) = text.parse::<DocumentMut>() else {
        return Vec::new();
    };

    let mut targets = Vec::new();
    for (kind, item) in doc.as_table().iter() {
        let Some(rows) = item.as_array_of_tables() else {
            continue;
        };
        for row in rows.iter() {
            let (Some(name), Some(source_path)) = (
                row.get("name").and_then(Item::as_str),
                row.get("source_path").and_then(Item::as_str),
            ) else {
                continue;
            };
            targets.push(EmitOwnedEntry {
                kind: kind.to_string(),
                name: name.to_string(),
                path: PathBuf::from(source_path),
            });
        }
    }
    targets
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
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
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
    /// The host members' declared embedded-member facts — captured as declaration
    /// rows rather than a second copy the engine reads back off the rendered fence
    /// (0018, "the projection is not the database").
    pub nested_members: Vec<NestedMemberRow>,
}

/// One kind's declaration row — its identity and declared runtime facts.
/// The optional facts are omitted from the lock when the kind declares none, so the row
/// round-trips to exactly what was written.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct KindFactRow {
    /// The bare kind name.
    pub name: String,
    /// The declared provider authority, when the kind qualifies by one.
    #[serde(default)]
    pub provider: Option<String>,
    /// The `governs` locus root directory.
    pub governs_root: String,
    /// The `governs` locus filename glob.
    pub governs_glob: String,
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
    pub registration: Vec<String>,
    /// The host kind's declared nesting templates — the embedded child kind names it
    /// folds embedded members of. Empty for
    /// a kind that nests nothing, the tolerant round-trip a lockless/template-less
    /// kind takes.
    #[serde(default)]
    pub templates: Vec<String>,
}

/// One clause of a kind's effective contract, reduced to the columns the lock records:
/// which kind it governs, the predicate's key, the field it targets (when it names one),
/// its declared severity, its guidance and cite — the clause's four channels
/// —
/// and, per predicate, its own argument: the node-set/edge-scope predicates
/// carry
/// their bounds/target, and the node-scope predicates that need more than
/// `field`/`severity` (`min_len`/`max_len`/`max_lines`'s bound, `allowed_chars`'s
/// charset, `forbidden_keys`'s keys, `deny`'s values) carry theirs too — so a kind's
/// own floor clause round-trips losslessly, not identity+severity alone.
/// `unique`'s field rides the shared `field`
/// column (the same slot `required`/`min_len`/… target); the rest carry their own
/// optional columns since a plain field/severity pair cannot express them.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ClauseRow {
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
    /// The `min_len`/`max_len`/`max_lines` clause's scalar bound, when the predicate
    /// is one of those three.
    #[serde(default)]
    pub bound: Option<BoundRow>,
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
}

/// A node-scope clause row's scalar bound — `min_len`'s `min`, `max_len`/`max_lines`'s
/// `max`, each endpoint optional so the row carries only what the predicate declared.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct BoundRow {
    /// The inclusive lower bound, when the predicate declares one (`min_len`).
    #[serde(default)]
    pub min: Option<usize>,
    /// The inclusive upper bound, when the predicate declares one (`max_len`/`max_lines`).
    #[serde(default)]
    pub max: Option<usize>,
}

/// An `allowed_chars` clause row's declared character class — the wire form of
/// [`crate::contract::Charset`]: inclusive `"<lo>-<hi>"` range specs plus a literal
/// string of individually permitted characters.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CharsetRow {
    /// The inclusive character ranges, each a two-character `"<lo>-<hi>"` spec.
    #[serde(default)]
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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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
    /// The external verifier for the behavioral remainder, when declared.
    #[serde(default)]
    pub verified_by: Option<String>,
}

/// A requirement row's `count` bound — the satisfier-set size's inclusive `[min, max]`.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct CountBoundRow {
    /// The inclusive lower bound on the satisfier-set size.
    pub min: usize,
    /// The inclusive upper bound on the satisfier-set size.
    pub max: usize,
}

/// A requirement row's `degree` bound — the in/out edge-count bound every satisfier
/// must land in.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct DegreeBoundRow {
    /// The bound on a satisfier's incoming edge count, when constrained.
    #[serde(default)]
    pub incoming: Option<EdgeBoundRow>,
    /// The bound on a satisfier's outgoing edge count, when constrained.
    #[serde(default)]
    pub outgoing: Option<EdgeBoundRow>,
}

/// One direction's inclusive `[min, max]` edge-count bound, each endpoint optional.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SatisfiesRow {
    /// The filling member's id.
    pub member: String,
    /// The requirement key the member opts into filling.
    pub requirement: String,
}

/// One authored `n` mention edge's declaration row — the citing member's own
/// `kind:name` address and the address its mention names (another member's
/// `kind:name`, or a bare requirement name), already resolved at emit. Recorded
/// unconditionally: a dangling mention never reaches the lock (`emit` refuses
/// first), so this row carries no resolution state of its own.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MentionRow {
    /// The citing member's own `kind:name` address.
    pub member: String,
    /// The address the mention names.
    pub target: String,
}

/// One host member's declared embedded-member value's declaration row — its
/// identity (the host's own `kind:name` address, the embedded child kind, and its
/// key) plus its leaves and sibling collections: the same composed value
/// `blocks()` renders into the host's `member.<kind> <key>` fence. The sole fact
/// source the read side consumes (`crate::builtin_kind::features`, matched by
/// `host` address) — never a second copy of a value the engine reads back off its
/// own rendering (0018, "the projection is not the database").
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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
    #[serde(default, deserialize_with = "deserialize_collections")]
    pub collections: Vec<CollectionEntryRow>,
}

/// One entry belonging to one of a [`NestedMemberRow`]'s sibling collections: the
/// collection name, the entry's own key, and its leaf fields — the row's
/// flattened, order-preserving shape (`to_table`/`from_table` serialize the whole
/// column as one array, the discipline every other array-shaped declaration
/// family gets from `toml_edit`).
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, Deserialize)]
struct CollectionEntryWire {
    key: String,
    #[serde(default)]
    leaves: BTreeMap<String, String>,
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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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
    /// An `edge` fact's target kind.
    #[serde(default)]
    pub to: Option<String>,
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
        if !table.is_empty() {
            doc["declaration"] = Item::Table(table);
        }
    }
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
    let path = workspace_dir.join("lock.toml");
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
/// Returns a [`DriftError::LockParse`] if `text` is not valid TOML.
pub fn parse_declarations(path: &Path, text: &str) -> Result<Declarations, DriftError> {
    let doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse {
            path: path.to_path_buf(),
            source,
        })?;
    Ok(declarations_from_doc(&doc))
}

/// Extract the seven declaration families off a parsed lock's `[declaration]` table. A row
/// missing a required column is skipped rather than erroring — a generated section is never
/// malformed, and a hand-edited broken row degrades to absent, the tolerant read the other
/// lock readers take.
fn declarations_from_doc(doc: &DocumentMut) -> Declarations {
    let Some(table) = doc.get("declaration").and_then(Item::as_table_like) else {
        return Declarations::default();
    };
    Declarations {
        kinds: family(table, "kind", KindFactRow::from_table),
        clauses: family(table, "clause", ClauseRow::from_table),
        requirements: family(table, "requirement", RequirementRow::from_table),
        assembly: family(table, "assembly", AssemblyFactRow::from_table),
        satisfies: family(table, "satisfies", SatisfiesRow::from_table),
        mentions: family(table, "mention", MentionRow::from_table),
        nested_members: family(table, "nested_member", NestedMemberRow::from_table),
    }
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
/// table through `parse` and dropping any malformed row.
fn family<T>(table: &dyn TableLike, key: &str, parse: impl Fn(&Table) -> Option<T>) -> Vec<T> {
    table
        .get(key)
        .and_then(Item::as_array_of_tables)
        .map(|array| array.iter().filter_map(parse).collect())
        .unwrap_or_default()
}

/// One required/optional string column off a declaration row — `None` when absent (or not
/// a string), which a required column treats as a malformed, skipped row.
fn str_col(table: &Table, key: &str) -> Option<String> {
    table.get(key).and_then(Item::as_str).map(str::to_string)
}

impl KindFactRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
        table.insert("name", value(self.name.clone()));
        if let Some(provider) = &self.provider {
            table.insert("provider", value(provider.clone()));
        }
        table.insert("governs_root", value(self.governs_root.clone()));
        table.insert("governs_glob", value(self.governs_glob.clone()));
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
            table.insert("templates", value(string_array(&self.templates)));
        }
        table
    }

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            name: str_col(table, "name")?,
            provider: str_col(table, "provider"),
            governs_root: str_col(table, "governs_root")?,
            governs_glob: str_col(table, "governs_glob")?,
            format: str_col(table, "format"),
            unit_shape: str_col(table, "unit_shape"),
            registration: table
                .get("registration")
                .and_then(string_array_from_item)
                .unwrap_or_default(),
            templates: table
                .get("templates")
                .and_then(string_array_from_item)
                .unwrap_or_default(),
        })
    }
}

impl ClauseRow {
    fn to_table(&self) -> Table {
        let mut table = Table::new();
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
        if let Some(bound) = &self.bound {
            table.insert("bound", value(bound_table(bound)));
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
        table
    }

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            kind: str_col(table, "kind"),
            predicate: str_col(table, "predicate")?,
            field: str_col(table, "field"),
            severity: str_col(table, "severity")?,
            guidance: str_col(table, "guidance"),
            cite: str_col(table, "cite"),
            count: table
                .get("count")
                .and_then(Item::as_table_like)
                .and_then(count_bound_from_table),
            target: str_col(table, "target"),
            degree: table
                .get("degree")
                .and_then(Item::as_table_like)
                .and_then(degree_bound_from_table),
            bound: table
                .get("bound")
                .and_then(Item::as_table_like)
                .map(bound_from_table),
            charset: table
                .get("charset")
                .and_then(Item::as_table_like)
                .map(charset_from_table),
            keys: table.get("keys").and_then(string_array_from_item),
            values: table.get("values").and_then(string_array_from_item),
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
        if let Some(verified_by) = &self.verified_by {
            table.insert("verified_by", value(verified_by.clone()));
        }
        table
    }

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            name: str_col(table, "name")?,
            kind: str_col(table, "kind"),
            required: table
                .get("required")
                .and_then(Item::as_bool)
                .unwrap_or(false),
            clauses: table
                .get("clauses")
                .and_then(Item::as_array_of_tables)
                .map(|array| array.iter().filter_map(ClauseRow::from_table).collect())
                .unwrap_or_default(),
            verified_by: str_col(table, "verified_by"),
        })
    }
}

/// One integer column off an inline table-like as a `usize`. Any miss — absent,
/// non-integer, or negative — is `None`.
fn usize_col(table: &dyn TableLike, key: &str) -> Option<usize> {
    table
        .get(key)?
        .as_integer()
        .and_then(|n| usize::try_from(n).ok())
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

fn count_bound_from_table(table: &dyn TableLike) -> Option<CountBoundRow> {
    Some(CountBoundRow {
        min: usize_col(table, "min")?,
        max: usize_col(table, "max")?,
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

fn degree_bound_from_table(table: &dyn TableLike) -> Option<DegreeBoundRow> {
    Some(DegreeBoundRow {
        incoming: table
            .get("incoming")
            .and_then(Item::as_table_like)
            .and_then(edge_bound_from_table),
        outgoing: table
            .get("outgoing")
            .and_then(Item::as_table_like)
            .and_then(edge_bound_from_table),
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

fn edge_bound_from_table(table: &dyn TableLike) -> Option<EdgeBoundRow> {
    Some(EdgeBoundRow {
        min: usize_col(table, "min"),
        max: usize_col(table, "max"),
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

fn bound_from_table(table: &dyn TableLike) -> BoundRow {
    BoundRow {
        min: usize_col(table, "min"),
        max: usize_col(table, "max"),
    }
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

fn charset_from_table(table: &dyn TableLike) -> CharsetRow {
    CharsetRow {
        ranges: table
            .get("ranges")
            .and_then(string_array_from_item)
            .unwrap_or_default(),
        chars: table
            .get("chars")
            .and_then(Item::as_str)
            .map(str::to_string),
    }
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

/// Read a TOML array of strings back off a declaration row column. Any element
/// that is not a string fails the whole column — the same tolerant-row (not
/// tolerant-element) degrade the rest of the lock's array columns take.
fn string_array_from_item(item: &Item) -> Option<Vec<String>> {
    let array = item.as_array()?;
    let mut out = Vec::with_capacity(array.len());
    for value in array.iter() {
        out.push(value.as_str()?.to_string());
    }
    Some(out)
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
            table.insert("to", value(to.clone()));
        }
        table
    }

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            fact: str_col(table, "fact")?,
            value: str_col(table, "value"),
            from: str_col(table, "from"),
            field: str_col(table, "field"),
            to: str_col(table, "to"),
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

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            member: str_col(table, "member")?,
            requirement: str_col(table, "requirement")?,
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

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            member: str_col(table, "member")?,
            target: str_col(table, "target")?,
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
        table
    }

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            host: str_col(table, "host")?,
            kind: str_col(table, "kind")?,
            key: str_col(table, "key")?,
            leaves: table
                .get("leaves")
                .and_then(Item::as_table_like)
                .map(string_map_from_table)
                .unwrap_or_default(),
            collections: table
                .get("collections")
                .and_then(Item::as_array)
                .map(collections_from_array)
                .unwrap_or_default(),
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

/// Read a string map back off a declaration row column — a non-string value drops
/// just that entry, the same tolerant-element-inside-a-tolerant-row discipline the
/// rest of the family takes.
fn string_map_from_table(table: &dyn TableLike) -> BTreeMap<String, String> {
    table
        .iter()
        .filter_map(|(key, item)| {
            item.as_str()
                .map(|text| (key.to_string(), text.to_string()))
        })
        .collect()
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

/// Read a `collections` column back off its order-preserving array — an element
/// that fails to parse as an inline table carrying the expected columns drops
/// just that entry, never the whole row.
fn collections_from_array(array: &Array) -> Vec<CollectionEntryRow> {
    array
        .iter()
        .filter_map(|value| {
            let table = value.as_inline_table()?;
            Some(CollectionEntryRow {
                collection: table.get("collection")?.as_str()?.to_string(),
                key: table.get("key")?.as_str()?.to_string(),
                leaves: table
                    .get("leaves")
                    .and_then(Value::as_inline_table)
                    .map(|table| string_map_from_table(table))
                    .unwrap_or_default(),
            })
        })
        .collect()
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
}
