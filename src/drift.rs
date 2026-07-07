//! `emit` — the drift engine.
//!
//! specs/architecture/20-surface.md, "The seam — one implementation"; "Content-faithful,
//! deterministically emitted (law 5)"; "Decision: `re-add` is retired — hand-edits route
//! to the source" (a direct edit to emitted output is drift routed to the authored
//! source, never merged back).
//!
//! [`emit_program`] runs the SDK program (`node <workspace>/harness.ts`) and hands its
//! JSON payload to [`emit`], the sole compiler of every projection and the whole lock —
//! no harness re-supply, the payload IS the source. Each projection is re-emitted
//! **whole** and byte-deterministically — verified by a double-emit comparison, so
//! nondeterministic authoring is a loud failure, never a silent churn. A hand-edited
//! projection is overwritten: it is drift routed to the source, surfaced by
//! `config.stale`/the guard, not a merge. [`place`] is the whole-file placement merge
//! for artifacts temper *places* rather than emits (specs/architecture/50-distribution.md,
//! `install`); it keeps its own three-state conflict detection until `install` rides
//! emit's projection.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;
use serde_json::Value as JsonValue;
use toml_edit::{ArrayOfTables, DocumentMut, InlineTable, Item, Table, TableLike, Value, value};

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
    /// nothing to compile from (`specs/architecture/20-surface.md`, "The seam — one
    /// implementation").
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

    /// The SDK program exited non-zero — a refusal (`20-surface.md`, "Emit refuses
    /// before it writes") or an authoring error; its stderr carries the reason.
    #[error("the SDK program {path} exited with a failure:\n{stderr}")]
    #[diagnostic(code(temper::drift::sdk_program_failed))]
    SdkProgramFailed {
        /// The harness entry path that failed.
        path: PathBuf,
        /// The program's captured stderr.
        stderr: String,
    },

    /// The SDK program's stdout was not valid UTF-8 — the JSON pipe is text
    /// (`20-surface.md`, "The seam").
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
    /// engine have drifted out of the lockstep the seam requires (`20-surface.md`,
    /// "The SDK pins its engine version").
    #[error(
        "the SDK program's payload declares seam version {got}; this engine reads version {SEAM_VERSION}"
    )]
    #[diagnostic(code(temper::drift::seam_version))]
    UnsupportedSeamVersion {
        /// The version the payload declared.
        got: u32,
    },

    /// A projected member's payload names a kind absent from the payload's own
    /// `declarations.kinds` family — the engine is kind-blind (`15-kinds.md`) and has
    /// nowhere to read that kind's locus/format/unit-shape from.
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
// emit — the write direction (`specs/architecture/20-surface.md`, law 5)
// ---------------------------------------------------------------------------

/// Options controlling an [`emit`] run.
#[derive(Debug, Clone, Copy, Default)]
pub struct EmitOptions {
    /// When set, compute every projection and report it but write nothing — neither
    /// the re-emitted harness sources nor the updated lock fingerprints.
    pub dry_run: bool,
    /// Refuse network access — the CI posture (`specs/architecture/20-surface.md`, CLI
    /// surface). `emit` performs no network I/O today (it compiles a materialized
    /// surface), so this changes nothing yet; it is accepted for CLI-surface / CI
    /// parity and reserved for the altitude's package-fetch step.
    pub frozen: bool,
}

/// One artifact's outcome from an [`emit`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitOutcome {
    /// The projection was re-emitted whole to match the surface (or, under
    /// `--dry-run`, would have been): its bytes differed from disk, or the source
    /// was absent. Emit regenerates from the authored source, so a hand-edited
    /// projection is overwritten — that edit is drift routed to the source, never a
    /// merge (`specs/architecture/20-surface.md`, `re-add` retired).
    Emitted,
    /// The re-emitted projection already sat on disk byte-for-byte; nothing to
    /// write. The idempotent no-op — a re-run of a clean emit lands here for every
    /// artifact.
    Unchanged,
}

impl EmitOutcome {
    /// The lower-case label used in the rendered report and stable for tests.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            EmitOutcome::Emitted => "emitted",
            EmitOutcome::Unchanged => "unchanged",
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

/// The typed result of an [`emit`]: every artifact's outcome, in the payload's
/// stable load order (kind-then-name). Renders nothing itself — [`render_emit`]
/// turns it into text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitReport {
    /// Every projected artifact, across every kind the payload names.
    pub entries: Vec<EmitEntry>,
}

/// The engine's pinned seam version — the JSON pipe rides it in lockstep with the
/// SDK's own `SEAM_VERSION` (`sdk/src/declarations.ts`; `specs/architecture/20-surface.md`,
/// "The SDK pins its engine version").
pub const SEAM_VERSION: u32 = 2;

/// One projected member's erased payload — the SDK's whole output surface for a
/// member that lives at a path locus (`sdk/src/emit.ts` `PayloadMember`). A genre
/// member never appears here (it carries no standalone projection).
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
    /// `file()` — absent for `text`/`blocks` prose or no prose. Lets `emit`
    /// tell a lifted member's own file (source == projection) apart from a
    /// generated one (`specs/architecture/20-surface.md`, "surface authority is a
    /// declared posture": "the lock is what names a path a projection").
    #[serde(default)]
    pub source_path: Option<String>,
}

/// The whole seam payload the SDK program prints to stdout
/// (`specs/architecture/20-surface.md`, "The seam — one implementation"): the
/// declaration rows (the lock's five families) and every projected member's erased
/// payload. The engine is the sole compiler of every projection and the whole lock
/// from this one value — no harness re-supply, the payload IS the source.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Payload {
    /// The pinned seam version this payload was compiled against.
    pub version: u32,
    /// The five declaration families.
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
    /// Whether this member's `file()` source resolves to the very path it
    /// projects to — a lifted member, authored territory the guard/note never
    /// claim (`specs/architecture/20-surface.md`, "A member whose `file()` source
    /// is its own projected path is authored territory").
    own_path: bool,
}

/// Whether a member's declared `file()` source (`payload_source`, when present)
/// resolves to the very path it is about to project to (`dest`) — the lift's
/// own-path detection. `dest` may not exist yet (a brand-new projection can
/// never coincide with an existing source), so a failed canonicalize on either
/// side reads as "not the same path" rather than an error: absent evidence must
/// never *forge* a guard claim, but it must never *suppress* one either, so the
/// safe default on any doubt is `false` (emit-owned, guarded).
fn resolves_to_own_path(payload_source: Option<&str>, dest: &Path) -> bool {
    let Some(source) = payload_source else {
        return false;
    };
    let (Ok(source_real), Ok(dest_real)) = (fs::canonicalize(source), fs::canonicalize(dest))
    else {
        return false;
    };
    source_real == dest_real
}

/// The harness-relative locus a member of `facts` named `name` projects onto — the
/// Rust port of the retired SDK `projectionPath` (`sdk/src/project.ts`; the engine is
/// the sole compiler now, `specs/architecture/20-surface.md`): a directory unit lands
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
/// call — the whole seam (`specs/architecture/20-surface.md`, "The seam — one
/// implementation"): `node` executes the authored program, the engine reads the JSON
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

/// Compile a seam `payload` into every projection and the whole lock — the sole
/// compiler (`specs/architecture/20-surface.md`, "The lock and drift — one
/// vocabulary": "one producer writes all three families"). `workspace_dir` is the
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
        let own_path = resolves_to_own_path(member.source_path.as_deref(), &source_path);
        projections.push(Projection {
            kind: member.kind.clone(),
            name: member.name.clone(),
            source_path,
            fields: member.fields.clone(),
            body: member.body.clone(),
            own_path,
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
                source_path: projection.source_path.to_string_lossy().into_owned(),
                source_hash: hash.clone(),
                emit_hash: hash,
                own_path: projection.own_path,
            });
        entries.push(entry);
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
    // re-emit. Those metadata lines ride `install`, never `emit` (law 5 keeps the
    // projection content-faithful), so a re-emit round-trips the ones already on disk
    // instead of clobbering them (`specs/architecture/20-surface.md`, the two-projectors
    // seam). An absent source carries no placements and is not a conflict: emit writes it.
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

    // An own-path member's `file()` source *is* its projected path — the projection
    // is not derived from typed fields, it is the source (`specs/architecture/
    // 20-surface.md`, "A member whose `file()` source is its own projected path is
    // authored territory: ... a hand edit there is an edit to the source"). Deriving
    // frontmatter from fields and writing it back over that same file would double
    // up the identity field a directory-shaped kind (`skill`) always carries
    // (`orderedFields`, `sdk/src/kind.ts`) atop the file's own, already-authored
    // frontmatter — so own-path skips field-derived rendering and projects the
    // resolved body (the whole file, byte-faithful) verbatim.
    let desired = if projection.own_path {
        projection.body.clone()
    } else {
        project_bytes(&projection.fields, &projection.body, &placements)
    };

    // Double-emit determinism (`specs/architecture/20-surface.md`, law 5): a second
    // projection over the same surface must be byte-identical. Nondeterministic
    // authoring (a timestamp, an unordered map surfacing into a field) is a loud
    // failure here, never a silent churn the next `emit` would rewrite.
    let second_pass = if projection.own_path {
        projection.body.clone()
    } else {
        project_bytes(&projection.fields, &projection.body, &placements)
    };
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
/// The authored content is *generated*, not patched (`specs/architecture/20-surface.md`,
/// "Decision: the projection is re-emitted; the surface is patched") — a hand-edited
/// field is not preserved (that is drift, routed to the authored source). Install's
/// metadata comments are the one exception the caller feeds in: they ride `install`,
/// never `emit` (law 5), so emit round-trips the ones already on disk rather than
/// dropping them (the two-projectors seam). An artifact with no fields (a rule that
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
    let (mut emitted, mut unchanged) = (0u32, 0u32);
    for entry in &report.entries {
        match entry.outcome {
            EmitOutcome::Emitted => emitted += 1,
            EmitOutcome::Unchanged => unchanged += 1,
        }
        out.push_str(&format!(
            "{:<10}  {:<5}  {}\n",
            entry.outcome.label(),
            entry.kind,
            entry.name
        ));
    }
    out.push_str(&format!("\n{emitted} emitted, {unchanged} unchanged\n"));
    out
}

// ---------------------------------------------------------------------------
// place — the whole-file direction (`specs/architecture/50-distribution.md`, `install`)
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
/// placement direction, for artifacts temper *places* rather than emits
/// (`specs/architecture/50-distribution.md`, the `install` gate wiring). It carries its own
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
/// written **errors loudly** rather than silently skipping
/// (`specs/architecture/50-distribution.md`, "Fail-loud delivery").
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
// config.stale — the freshness fact the gate reads (`specs/architecture/20-surface.md`)
// ---------------------------------------------------------------------------

/// The diagnostic `rule` id every freshness finding reports under
/// (`specs/architecture/20-surface.md`, "Drift — one direction, two freshness facts").
const CONFIG_STALE_RULE: &str = "config.stale";

/// The `config.stale` freshness findings for a surface `workspace_dir`
/// (`specs/architecture/20-surface.md`, "`config.stale` — the committed manifest/projection does
/// not match the lock's `source_hash`/`emit_hash` pair"): a committed projection whose
/// bytes no longer match the emit fingerprint the lock recorded — the authored source
/// changed and `emit` has not run, or the emitted output was hand-edited. One finding
/// per drifted row, pointing at the projection that moved.
///
/// **Advisory** (`warn`): under the default `shared` authority the guard warns-and-routes
/// rather than blocks (`specs/architecture/20-surface.md`, "surface authority is a declared
/// posture"), and temper fabricates no hard gate the author did not declare
/// (`00-intent.md` law 4) — a stale projection is a nudge to re-emit.
///
/// Read off `<workspace_dir>/lock.toml` — every `[[<kind>]]` row (built-in and custom):
/// each row's `source_path` is re-hashed and compared to its `emit_hash`. A row without
/// an `emit_hash` (a lock predating the fingerprint) or a `source_path` that cannot be
/// read is **skipped** — law 3's safe direction, since absent evidence must never *forge*
/// a staleness finding (a removed source is the drift engine's `removed` state, not this
/// freshness fact). A missing or malformed lock yields no findings for the same reason.
///
/// An in-place member carries **no lock row** (`init` writes no copy tree, no lock — the
/// landscape file is its own source), so it contributes no freshness fact here: an
/// in-place member cannot drift (`specs/architecture/20-surface.md`).
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
// placements (`specs/architecture/20-surface.md`, "surface authority is a declared
// posture")
// ---------------------------------------------------------------------------

/// One member the lock declares **emit-owned** — a real projection, not a lifted
/// member's own authored file (`specs/architecture/20-surface.md`, "the note and the
/// guard bind only paths the lock declares emit-owned").
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
/// walk they once targeted (`specs/architecture/20-surface.md`, "the guard arrives
/// with its constituency, never before"). A row with no recorded `own_path` (a lock
/// predating the fact, or a member with no `file()` prose) defaults to emit-owned —
/// the safe direction, since a placement wrongly *placed* is a nudge to remove, but
/// one wrongly *withheld* is a silent gap in the gate's write-boundary coverage.
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
            let own_path = row.get("own_path").and_then(Item::as_bool).unwrap_or(false);
            if own_path {
                continue;
            }
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
// (`specs/architecture/20-surface.md`, "The lock and drift"; `specs/architecture/40-composition.md`)
// ---------------------------------------------------------------------------

/// The lock's **declaration-row family** — the composed program's erased declarations
/// (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary"), beside the
/// per-member provenance and emit-fingerprint rows. Five sub-families: the program's
/// [kind facts](KindFactRow), its [clauses](ClauseRow), its [requirements](RequirementRow),
/// its assembly facts ([`AssemblyFactRow`], `specs/architecture/40-composition.md`), and its
/// [`satisfies`](SatisfiesRow) fill edges.
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
    /// The kind facts — one per kind in the program (`specs/architecture/15-kinds.md`).
    pub kinds: Vec<KindFactRow>,
    /// The clauses of every kind's effective contract (`specs/architecture/10-contracts.md`).
    pub clauses: Vec<ClauseRow>,
    /// The named requirements the assembly declares (`specs/architecture/10-contracts.md`).
    pub requirements: Vec<RequirementRow>,
    /// The assembly-scope facts — authority, edges
    /// (`specs/architecture/40-composition.md`; `specs/architecture/45-governance.md`).
    pub assembly: Vec<AssemblyFactRow>,
    /// The member→requirement fill edges — every imported member's `satisfies` keys
    /// (`specs/architecture/20-surface.md`, "The lock and drift"), so the roster/coverage
    /// tiers ride the lock rather than re-importing the harness.
    pub satisfies: Vec<SatisfiesRow>,
}

/// One kind's declaration row — its identity and declared runtime facts
/// (`specs/architecture/15-kinds.md`, "a kind's runtime residue is its five declaration facts").
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
    /// The declared activation label, when declared.
    #[serde(default)]
    pub activation: Option<String>,
}

/// One clause of a kind's effective contract, reduced to the columns the lock records:
/// which kind it governs, the predicate's key, the field it targets (when it names one),
/// its declared severity, and — for the node-set/edge-scope predicates
/// (`count`/`unique`/`membership`/`degree`, `specs/architecture/10-contracts.md`) — the
/// argument channel their bounds/target round-trip through. `unique`'s field rides the
/// shared `field` column (the same slot `required`/`min_len`/… target); the others carry
/// their own optional columns since a plain field/severity pair cannot express them.
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
    /// The `count` clause's satisfier-set-size bound, when the predicate is `count`.
    #[serde(default)]
    pub count: Option<CountBoundRow>,
    /// The `membership` clause's target requirement name, when the predicate is
    /// `membership`. Distinct from the clause's own citation (`Clause::source`) — a
    /// `ClauseRow` carries no citation column at all today.
    #[serde(default)]
    pub target: Option<String>,
    /// The `degree` clause's in/out edge-count bound, when the predicate is `degree`.
    #[serde(default)]
    pub degree: Option<DegreeBoundRow>,
}

/// One named requirement's declaration row (`specs/architecture/10-contracts.md`),
/// carrying the scalar facets plus the requirement's own **clause rows** — the
/// set-scope demands (`count`/`unique`/`membership`/`degree`,
/// `specs/architecture/10-contracts.md`, "Decision: set-scope demands are clauses")
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

/// A requirement row's `count` bound — the satisfier-set size's inclusive `[min, max]`
/// (`specs/architecture/45-governance.md`).
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct CountBoundRow {
    /// The inclusive lower bound on the satisfier-set size.
    pub min: usize,
    /// The inclusive upper bound on the satisfier-set size.
    pub max: usize,
}

/// A requirement row's `degree` bound — the in/out edge-count bound every satisfier
/// must land in (`specs/architecture/45-governance.md`).
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct DegreeBoundRow {
    /// The bound on a satisfier's incoming edge count, when constrained.
    #[serde(default)]
    pub incoming: Option<EdgeBoundRow>,
    /// The bound on a satisfier's outgoing edge count, when constrained.
    #[serde(default)]
    pub outgoing: Option<EdgeBoundRow>,
}

/// One direction's inclusive `[min, max]` edge-count bound, each endpoint optional
/// (`specs/architecture/45-governance.md`).
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
/// roster/coverage tiers need, carried on the lock rather than re-imported
/// (`specs/architecture/20-surface.md`, "The lock and drift").
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SatisfiesRow {
    /// The filling member's id.
    pub member: String,
    /// The requirement key the member opts into filling.
    pub requirement: String,
}

/// One assembly-scope fact — the graph/assembly declarations the harness binds
/// (`specs/architecture/40-composition.md`): a `fact` discriminator (`authority`,
/// `edge`) plus the columns that fact carries. Absent columns are omitted
/// from the lock, so each row round-trips to exactly what its producer wrote.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AssemblyFactRow {
    /// The fact discriminator: `authority` or `edge`.
    pub fact: String,
    /// The scalar value an `authority` fact carries (its posture).
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
    /// `[[declaration.assembly]]`, `[[declaration.satisfies]]` — each family in its producer's order so a re-emit is
    /// byte-identical (law 5). An empty family writes no array (an empty `ArrayOfTables`
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
        if !table.is_empty() {
            doc["declaration"] = Item::Table(table);
        }
    }
}

/// Read the lock's declaration-row family back into a typed [`Declarations`]
/// (`specs/architecture/20-surface.md`, "The lock and drift"): the gate's read side over the
/// rows the extraction wrote. A missing or malformed lock, or one with no `[declaration]`
/// table (any pre-recut lock), yields an empty set rather than an error — absent evidence
/// forges no finding (law 3), the same tolerance [`config_stale`] takes.
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
    let doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse { path, source })?;
    Ok(declarations_from_doc(&doc))
}

/// Extract the four declaration families off a parsed lock's `[declaration]` table. A row
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
        if let Some(activation) = &self.activation {
            table.insert("activation", value(activation.clone()));
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
            activation: str_col(table, "activation"),
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
        if let Some(count) = &self.count {
            table.insert("count", value(count_bound_table(count)));
        }
        if let Some(target) = &self.target {
            table.insert("target", value(target.clone()));
        }
        if let Some(degree) = &self.degree {
            table.insert("degree", value(degree_bound_table(degree)));
        }
        table
    }

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            kind: str_col(table, "kind"),
            predicate: str_col(table, "predicate")?,
            field: str_col(table, "field"),
            severity: str_col(table, "severity")?,
            count: table
                .get("count")
                .and_then(Item::as_table_like)
                .and_then(count_bound_from_table),
            target: str_col(table, "target"),
            degree: table
                .get("degree")
                .and_then(Item::as_table_like)
                .and_then(degree_bound_from_table),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-drift-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
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
}
