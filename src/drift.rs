//! `emit` — the drift engine.
//!
//! specs/architecture/20-surface.md, "Content-faithful, deterministically emitted (law 5)";
//! "Decision: `re-add` is retired — hand-edits route to the source" (a direct edit
//! to emitted output is drift routed to the authored source, never merged back).
//!
//! [`emit`] compiles the surface out, re-emitting each projection **whole** from
//! the authored source and byte-deterministically — verified by a double-emit
//! comparison, so nondeterministic authoring is a loud failure, never a silent
//! churn. A hand-edited projection is overwritten: it is drift routed to the
//! source, surfaced by `config.stale`/the guard, not a merge. [`place`] is the
//! whole-file placement merge for artifacts temper *places* rather than emits
//! (specs/architecture/50-distribution.md, `install`); it keeps its own
//! three-state conflict detection until `install` rides emit's projection.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;
use toml_edit::{
    Array, ArrayOfTables, DocumentMut, InlineTable, Item, Table, TableLike, Value, value,
};

use crate::builtin_kind;
use crate::check::Workspace;
use crate::hash::sha256_hex;
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
    /// The artifact kind — `"skill"` or `"rule"`.
    pub kind: &'static str,
    /// The artifact name (its surface name).
    pub name: String,
    /// The on-disk source path the projection targeted.
    pub source_path: PathBuf,
    /// What `emit` did (or would do, under `--dry-run`) for this artifact.
    pub outcome: EmitOutcome,
}

/// The typed result of an [`emit`]: every artifact's outcome, in the workspace's
/// stable load order (skills then rules, each name-sorted). Renders nothing itself
/// — [`render_emit`] turns it into text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitReport {
    /// Every projected artifact, across the built-in kinds.
    pub entries: Vec<EmitEntry>,
}

/// The desired projection of one surface artifact: its identity, the emit
/// fingerprint the lock currently records, the ordered header fields the surface
/// wants the frontmatter to express, and the byte-faithful body.
struct Projection {
    kind: &'static str,
    name: String,
    source_path: PathBuf,
    /// The emit fingerprint the lock currently records for this source (or the
    /// source hash as a baseline when no `emit` has advanced it yet). Compared
    /// against the re-emitted projection only to decide whether an already-matching
    /// projection needs its stale lock fingerprint reconciled — never to merge.
    emit_hash: String,
    /// The desired header fields in canonical order (known fields first, then the
    /// preserved unknown keys). The whole set is re-emitted into a fresh
    /// frontmatter block — the projection is regenerated, never patched.
    fields: Vec<(String, JsonValue)>,
    /// The desired body — the surface body, projected byte-faithfully.
    body: String,
}

/// Compile the loaded `workspace` surface out onto the harness sources, re-emitting
/// each projection **whole** from the authored source and byte-deterministically.
///
/// `workspace_dir` is the surface root — where the lock (`lock.toml`) carrying the
/// emit fingerprints lives. Each artifact is written to its recorded
/// `provenance.source_path` (where `import` read it from). Emit regenerates the
/// projection whole rather than merging on-disk bytes: a hand-edited projection is
/// overwritten (that edit is drift routed to the source, `specs/architecture/20-surface.md`),
/// and every projection is double-emit verified (`emit_one`). Nothing is written
/// under `options.dry_run`.
///
/// **In-place members are skipped.** emit compiles the authored library (the copy-tree
/// [`Workspace`]) out; an in-place member is its own source with no projection to compile
/// (`specs/architecture/20-surface.md`, "In-place members cannot drift"), so it never enters the
/// [`Workspace`] emit ranges over — only document/module-carried members carry a lock row
/// and a projection.
pub fn emit(
    workspace: &Workspace,
    workspace_dir: &Path,
    options: EmitOptions,
) -> miette::Result<EmitReport> {
    let kinds = embedded_kind_names();
    let emit_hashes = load_emit_hash(workspace_dir, &kinds)?;

    let mut projections = Vec::new();
    for skill in workspace.skills() {
        projections.push(Projection {
            kind: "skill",
            name: skill.id.clone(),
            source_path: skill.provenance.source_path.clone(),
            emit_hash: fingerprint(&emit_hashes, skill.provenance.source_path.as_path())
                .unwrap_or_else(|| skill.provenance.source_hash.clone()),
            fields: skill.fields.clone(),
            body: skill.body.clone(),
        });
    }
    for rule in workspace.rules() {
        projections.push(Projection {
            kind: "rule",
            name: rule.id.clone(),
            source_path: rule.provenance.source_path.clone(),
            emit_hash: fingerprint(&emit_hashes, rule.provenance.source_path.as_path())
                .unwrap_or_else(|| rule.provenance.source_hash.clone()),
            fields: rule.fields.clone(),
            body: rule.body.clone(),
        });
    }

    let mut entries = Vec::new();
    // source_path -> new emit fingerprint to record (an Emitted write, or a stale
    // lock reconciled on an Unchanged projection).
    let mut updates: Vec<(PathBuf, String)> = Vec::new();
    for projection in &projections {
        let (entry, update) = emit_one(projection, options.dry_run)?;
        if let Some(fingerprint) = update {
            updates.push((projection.source_path.clone(), fingerprint));
        }
        entries.push(entry);
    }

    if !options.dry_run && !updates.is_empty() {
        update_lock(workspace_dir, &kinds, &updates)?;
    }

    Ok(EmitReport { entries })
}

/// Re-emit one projection whole, returning its [`EmitEntry`] and the emit fingerprint
/// to record when the bytes moved (or a stale lock needs reconciling).
///
/// The projection is regenerated from the authored surface — never merged against
/// on-disk bytes — so a hand-edited projection is simply overwritten: a direct edit
/// to emitted output is drift routed to the source (`config.stale`/the guard surface
/// it), not a mergeable conflict. The on-disk read decides only `Emitted` vs the
/// idempotent `Unchanged`.
fn emit_one(
    projection: &Projection,
    dry_run: bool,
) -> Result<(EmitEntry, Option<String>), DriftError> {
    let row = |outcome| EmitEntry {
        kind: projection.kind,
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

    let desired = project_bytes(&projection.fields, &projection.body, &placements);

    // Double-emit determinism (`specs/architecture/20-surface.md`, law 5): a second
    // projection over the same surface must be byte-identical. Nondeterministic
    // authoring (a timestamp, an unordered map surfacing into a field) is a loud
    // failure here, never a silent churn the next `emit` would rewrite.
    if project_bytes(&projection.fields, &projection.body, &placements) != desired {
        return Err(DriftError::Nondeterministic {
            path: projection.source_path.clone(),
        });
    }

    if current.as_deref() == Some(desired.as_bytes()) {
        // Already at the projection. Reconcile a stale lock fingerprint (an
        // `emit_hash` predating these bytes) so it stops reading as `config.stale`;
        // otherwise leave the lock alone.
        let hash = sha256_hex(desired.as_bytes());
        let update = (hash != projection.emit_hash).then_some(hash);
        return Ok((row(EmitOutcome::Unchanged), update));
    }

    if !dry_run {
        fs::write(&projection.source_path, desired.as_bytes()).map_err(|source| {
            DriftError::Write {
                path: projection.source_path.clone(),
                source,
            }
        })?;
    }
    Ok((
        row(EmitOutcome::Emitted),
        Some(sha256_hex(desired.as_bytes())),
    ))
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

/// The bare table-key names of the kinds temper actually embeds — the lock's per-kind
/// array-of-tables keys (`[[skill]]`, `[[rule]]`, `[[memory]]`). Derived from
/// `builtin_kind::BUILTIN_KINDS` (`<provider>.<name>`, so the bare key is the segment
/// after the last `.`), never the stale `kind::BUILTIN_KINDS` = [skill, rule] const: a
/// newly-embedded kind's lock rows are fingerprinted the moment it joins the embedded
/// set, with no literal to re-pin (`specs/architecture/15-kinds.md`, "Decision: kinds
/// are declared data over generic extraction, never engine code"). Two providers
/// co-embedding one bare name (the `CLAUDE.md`/`AGENTS.md` memory family) share the one
/// lock key, so the names are deduped.
fn embedded_kind_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = builtin_kind::BUILTIN_KINDS
        .iter()
        .map(|key| key.rsplit('.').next().unwrap_or(key))
        .collect();
    names.sort_unstable();
    names.dedup();
    names
}

/// Read the emit fingerprints from `<workspace_dir>/lock.toml`, keyed by source path.
/// Covers every embedded built-in kind's rows (`kinds` — `[[skill]]`, `[[rule]]`, and a
/// curated `[[memory]]`), so a hand-edited projection of any of them is seen. A row
/// without an `emit_hash` column (a lock predating the field) is simply absent, and the
/// caller falls back to the source hash.
fn load_emit_hash(
    workspace_dir: &Path,
    kinds: &[&str],
) -> Result<std::collections::HashMap<PathBuf, String>, DriftError> {
    let path = workspace_dir.join("lock.toml");
    let text = fs::read_to_string(&path).map_err(|source| DriftError::LockRead {
        path: path.clone(),
        source,
    })?;
    let doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse {
            path: path.clone(),
            source,
        })?;

    let mut map = std::collections::HashMap::new();
    for &kind in kinds {
        let Some(rows) = doc.get(kind).and_then(Item::as_array_of_tables) else {
            continue;
        };
        for row in rows.iter() {
            if let (Some(source_path), Some(fingerprint)) = (
                row.get("source_path").and_then(Item::as_str),
                row.get("emit_hash").and_then(Item::as_str),
            ) {
                map.insert(PathBuf::from(source_path), fingerprint.to_string());
            }
        }
    }
    Ok(map)
}

/// Look up one source path's emit fingerprint in the loaded map.
fn fingerprint(map: &std::collections::HashMap<PathBuf, String>, source: &Path) -> Option<String> {
    map.get(source).cloned()
}

/// Write the reconciled fingerprints back into `<workspace_dir>/lock.toml` in place,
/// matching each embedded built-in kind's row (`kinds` — `[[skill]]`, `[[rule]]`, and a
/// curated `[[memory]]`) by its `source_path`. Format-preserving via `toml_edit` — only
/// the `emit_hash` values change.
fn update_lock(
    workspace_dir: &Path,
    kinds: &[&str],
    updates: &[(PathBuf, String)],
) -> Result<(), DriftError> {
    let path = workspace_dir.join("lock.toml");
    let text = fs::read_to_string(&path).map_err(|source| DriftError::LockRead {
        path: path.clone(),
        source,
    })?;
    let mut doc = text
        .parse::<DocumentMut>()
        .map_err(|source| DriftError::LockParse {
            path: path.clone(),
            source,
        })?;

    for &kind in kinds {
        let Some(rows) = doc.get_mut(kind).and_then(Item::as_array_of_tables_mut) else {
            continue;
        };
        for row in rows.iter_mut() {
            let Some(source_path) = row.get("source_path").and_then(Item::as_str) else {
                continue;
            };
            if let Some((_, fingerprint)) = updates
                .iter()
                .find(|(path, _)| path.as_os_str().to_string_lossy() == source_path)
            {
                row["emit_hash"] = value(fingerprint.clone());
            }
        }
    }

    fs::write(&path, doc.to_string()).map_err(|source| DriftError::Write { path, source })
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
// declaration rows — the program's erased declarations
// (`specs/architecture/20-surface.md`, "The lock and drift"; `specs/architecture/40-composition.md`)
// ---------------------------------------------------------------------------

/// The lock's **declaration-row family** — the composed program's erased declarations
/// (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary"), beside the
/// per-member provenance and emit-fingerprint rows. Four sub-families: the program's
/// [kind facts](KindFactRow), its [clauses](ClauseRow), its [requirements](RequirementRow),
/// and its assembly facts ([`AssemblyFactRow`], `specs/architecture/40-composition.md`).
///
/// Written into the lock by the extraction (`import`, [`Declarations::write_into`]) and
/// read back here ([`read_declarations`]) for the gate's one disk-vs-lock comparison —
/// the gate read lands next in the chain; `SDK-RECUT-CORPUS-FACE` moves the producer from
/// the current extraction to the SDK. Each family's columns are owned scalars (or small
/// owned collections for a set-scope facet) so the read and write sides are the same
/// shape: the lock is the vocabulary, not a typed IR.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Declarations {
    /// The kind facts — one per kind in the program (`specs/architecture/15-kinds.md`).
    pub kinds: Vec<KindFactRow>,
    /// The clauses of every kind's effective contract (`specs/architecture/10-contracts.md`).
    pub clauses: Vec<ClauseRow>,
    /// The named requirements the assembly declares (`specs/architecture/10-contracts.md`).
    pub requirements: Vec<RequirementRow>,
    /// The assembly-scope facts — authority, reachability, edges
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindFactRow {
    /// The bare kind name.
    pub name: String,
    /// The declared provider authority, when the kind qualifies by one.
    pub provider: Option<String>,
    /// The `governs` locus root directory.
    pub governs_root: String,
    /// The `governs` locus filename glob.
    pub governs_glob: String,
    /// The declared projection format label, when declared.
    pub format: Option<String>,
    /// The declared unit-shape label, when declared.
    pub unit_shape: Option<String>,
    /// The declared activation label, when declared.
    pub activation: Option<String>,
}

/// One clause of a kind's effective contract, reduced to the columns the lock records:
/// which kind it governs, the predicate's key, the field it targets (when it names one),
/// and its declared severity (`specs/architecture/10-contracts.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClauseRow {
    /// The kind whose contract carries the clause.
    pub kind: String,
    /// The predicate's clause key (`required`, `max_len`, …).
    pub predicate: String,
    /// The field (or marker) the predicate constrains, when it names one.
    pub field: Option<String>,
    /// The clause's declared severity (`required` / `advisory`).
    pub severity: String,
}

/// One named requirement's declaration row (`specs/architecture/10-contracts.md`), carrying
/// the scalar facets plus the set-scope bounds — `count`/`unique`/`membership`/`degree`
/// (`specs/architecture/45-governance.md`) — the roster/graph checks range over: the lock
/// now carries a requirement's whole shape, not the scalar-only bootstrap slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementRow {
    /// The requirement's name.
    pub name: String,
    /// The kind that may fill it, when typed by one.
    pub kind: Option<String>,
    /// The package the filler must conform to, when bound.
    pub package: Option<String>,
    /// Whether an unfilled requirement blocks the gate.
    pub required: bool,
    /// The set-scope `count` bound on the satisfier-set size, when declared.
    pub count: Option<CountBoundRow>,
    /// The set-scope `unique` field list — each named field's extracted scalar must
    /// not repeat across the satisfiers. Empty when undeclared.
    pub unique: Vec<String>,
    /// The set-scope `membership` predicate, when declared.
    pub membership: Option<MembershipRow>,
    /// The graph-scope `degree` bound on every satisfier's in/out edge count, when
    /// declared.
    pub degree: Option<DegreeBoundRow>,
    /// The external verifier for the behavioral remainder, when declared.
    pub verified_by: Option<String>,
}

/// A requirement row's `count` bound — the satisfier-set size's inclusive `[min, max]`
/// (`specs/architecture/45-governance.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountBoundRow {
    /// The inclusive lower bound on the satisfier-set size.
    pub min: usize,
    /// The inclusive upper bound on the satisfier-set size.
    pub max: usize,
}

/// A requirement row's `membership` predicate — a declared field of every satisfier
/// (S1) must lie in a corpus-derived set drawn from a second satisfier set (S2)
/// (`specs/architecture/45-governance.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipRow {
    /// The field on each S1 satisfier checked against the source set.
    pub field: String,
    /// The source requirement (R2) whose satisfier set (S2) supplies the allowed values.
    pub source: String,
    /// The artifact kind S2 is drawn from.
    pub source_kind: String,
    /// The feature whose extracted scalars over S2 form the allowed set.
    pub source_feature: String,
    /// The optional typed-reference package S2 is narrowed to conform to.
    pub source_package: Option<String>,
}

/// A requirement row's `degree` bound — the in/out edge-count bound every satisfier
/// must land in (`specs/architecture/45-governance.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegreeBoundRow {
    /// The bound on a satisfier's incoming edge count, when constrained.
    pub incoming: Option<EdgeBoundRow>,
    /// The bound on a satisfier's outgoing edge count, when constrained.
    pub outgoing: Option<EdgeBoundRow>,
}

/// One direction's inclusive `[min, max]` edge-count bound, each endpoint optional
/// (`specs/architecture/45-governance.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeBoundRow {
    /// The inclusive lower bound. `None` ⇒ no lower bound.
    pub min: Option<usize>,
    /// The inclusive upper bound. `None` ⇒ unbounded above.
    pub max: Option<usize>,
}

/// One member→requirement fill edge's declaration row — the `satisfies` join the
/// roster/coverage tiers need, carried on the lock rather than re-imported
/// (`specs/architecture/20-surface.md`, "The lock and drift").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfiesRow {
    /// The filling member's id.
    pub member: String,
    /// The requirement key the member opts into filling.
    pub requirement: String,
}

/// One assembly-scope fact — the graph/assembly declarations the harness binds
/// (`specs/architecture/40-composition.md`): a `fact` discriminator (`authority`,
/// `reachability`, `edge`) plus the columns that fact carries. Absent columns are omitted
/// from the lock, so each row round-trips to exactly what its producer wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyFactRow {
    /// The fact discriminator: `authority`, `reachability`, or `edge`.
    pub fact: String,
    /// The scalar value an `authority`/`reachability` fact carries (its posture/severity).
    pub value: Option<String>,
    /// An `edge` fact's source kind.
    pub from: Option<String>,
    /// An `edge` fact's reference field.
    pub field: Option<String>,
    /// An `edge` fact's target kind.
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
        table.insert("kind", value(self.kind.clone()));
        table.insert("predicate", value(self.predicate.clone()));
        if let Some(field) = &self.field {
            table.insert("field", value(field.clone()));
        }
        table.insert("severity", value(self.severity.clone()));
        table
    }

    fn from_table(table: &Table) -> Option<Self> {
        Some(Self {
            kind: str_col(table, "kind")?,
            predicate: str_col(table, "predicate")?,
            field: str_col(table, "field"),
            severity: str_col(table, "severity")?,
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
        if let Some(package) = &self.package {
            table.insert("package", value(package.clone()));
        }
        table.insert("required", value(self.required));
        if let Some(count) = &self.count {
            table.insert("count", value(count_bound_table(count)));
        }
        if !self.unique.is_empty() {
            table.insert("unique", value(str_array(&self.unique)));
        }
        if let Some(membership) = &self.membership {
            table.insert("membership", value(membership_table(membership)));
        }
        if let Some(degree) = &self.degree {
            table.insert("degree", value(degree_bound_table(degree)));
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
            package: str_col(table, "package"),
            required: table
                .get("required")
                .and_then(Item::as_bool)
                .unwrap_or(false),
            count: table
                .get("count")
                .and_then(Item::as_table_like)
                .and_then(count_bound_from_table),
            unique: table
                .get("unique")
                .and_then(Item::as_array)
                .map(array_strings)
                .unwrap_or_default(),
            membership: table
                .get("membership")
                .and_then(Item::as_table_like)
                .and_then(membership_from_table),
            degree: table
                .get("degree")
                .and_then(Item::as_table_like)
                .and_then(degree_bound_from_table),
            verified_by: str_col(table, "verified_by"),
        })
    }
}

/// A TOML string [`Array`] over owned strings — the shape a requirement row's `unique`
/// field list re-emits as (mirrors `compose::str_array`, kept row-local since the
/// lock's declaration rows are their own flattened vocabulary, not compose's typed
/// model).
fn str_array(items: &[String]) -> Array {
    let mut array = Array::new();
    for item in items {
        array.push(item.as_str());
    }
    array
}

/// Read a TOML string array back into owned strings, dropping any non-string element.
fn array_strings(array: &Array) -> Vec<String> {
    array
        .iter()
        .filter_map(|item| item.as_str().map(str::to_string))
        .collect()
}

/// One integer column off an inline table-like as a `usize`. Any miss — absent,
/// non-integer, or negative — is `None`.
fn usize_col(table: &dyn TableLike, key: &str) -> Option<usize> {
    table
        .get(key)?
        .as_integer()
        .and_then(|n| usize::try_from(n).ok())
}

/// One required/optional string column off an inline table-like — `None` when absent
/// (or not a string).
fn str_col_like(table: &dyn TableLike, key: &str) -> Option<String> {
    table.get(key)?.as_str().map(str::to_string)
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

fn membership_table(membership: &MembershipRow) -> InlineTable {
    let mut table = InlineTable::new();
    table.insert("field", Value::from(membership.field.clone()));
    table.insert("source", Value::from(membership.source.clone()));
    table.insert("source_kind", Value::from(membership.source_kind.clone()));
    table.insert(
        "source_feature",
        Value::from(membership.source_feature.clone()),
    );
    if let Some(source_package) = &membership.source_package {
        table.insert("source_package", Value::from(source_package.clone()));
    }
    table
}

fn membership_from_table(table: &dyn TableLike) -> Option<MembershipRow> {
    Some(MembershipRow {
        field: str_col_like(table, "field")?,
        source: str_col_like(table, "source")?,
        source_kind: str_col_like(table, "source_kind")?,
        source_feature: str_col_like(table, "source_feature")?,
        source_package: str_col_like(table, "source_package"),
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
    use crate::import;
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

    const SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

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
    fn an_in_place_harness_has_no_lock_and_cannot_drift() {
        // `init` writes the manifest over members IN PLACE — no `.temper/` copy tree, no
        // lock (`specs/architecture/20-surface.md`, the on-ramp). The landscape file is its own
        // source, so there is no emit fingerprint to diverge from: `config_stale` reads
        // the workspace lock and finds none, so an in-place member yields no freshness
        // finding. This is the drift-free half of "In-place members cannot drift."
        let harness = tmpdir("inplace-no-drift");
        let skill = harness.join(".claude").join("skills").join("coordinate");
        fs::create_dir_all(&skill).unwrap();
        fs::write(skill.join("SKILL.md"), SKILL).unwrap();

        import::init(&harness).unwrap();

        // The manifest lands in place; no copy tree and no lock are written.
        assert!(harness.join("temper.toml").is_file());
        assert!(!harness.join(".temper").exists());
        assert!(!harness.join("lock.toml").exists());

        // No lock ⇒ no freshness finding, even after the source is edited (a live
        // re-extraction picks the edit up; there is nothing to be stale against).
        assert!(config_stale(&harness).is_empty());
        let edited = fs::read_to_string(skill.join("SKILL.md")).unwrap() + "\nExtra.\n";
        fs::write(skill.join("SKILL.md"), edited).unwrap();
        assert!(config_stale(&harness).is_empty());
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
    fn embedded_kind_names_derives_from_the_live_embed_not_the_stale_const() {
        // The enumerated set is the bare tail of every `builtin_kind::BUILTIN_KINDS` key,
        // deduped — so a curated addition rides in without editing a literal here.
        let names = embedded_kind_names();
        for expected in builtin_kind::BUILTIN_KINDS
            .iter()
            .map(|key| key.rsplit('.').next().unwrap_or(key))
        {
            assert!(
                names.contains(&expected),
                "embedded_kind_names() must cover the embedded `{expected}` kind"
            );
        }
        assert!(names.contains(&"skill") && names.contains(&"rule"));
        // Deduped: a bare name co-embedded by two providers is one lock key, listed once.
        let mut sorted = names.clone();
        sorted.dedup();
        assert_eq!(names, sorted);
    }

    #[test]
    fn load_and_update_lock_cover_a_memory_row_leaving_skill_rule_intact() {
        let dir = tmpdir("memory-lock");
        // A lock carrying a `[[memory]]` row beside the built-in skill/rule rows — the
        // shape once a `memory` KIND.md joins the embedded tree. The bug: the old loops
        // iterated the stale `[skill, rule]` const, so this memory row was never seen.
        fs::write(
            dir.join("lock.toml"),
            "[[skill]]\n\
name = \"coordinate\"\n\
source_path = \"/h/.claude/skills/coordinate/SKILL.md\"\n\
emit_hash = \"skill-old\"\n\
\n\
[[rule]]\n\
name = \"rust\"\n\
source_path = \"/h/.claude/rules/rust.md\"\n\
emit_hash = \"rule-old\"\n\
\n\
[[memory]]\n\
name = \"root\"\n\
source_path = \"/h/CLAUDE.md\"\n\
emit_hash = \"memory-old\"\n",
        )
        .unwrap();

        let kinds = ["skill", "rule", "memory"];
        let map = load_emit_hash(&dir, &kinds).unwrap();
        // Every embedded kind's row is fingerprinted, memory included.
        assert_eq!(
            map.get(Path::new("/h/CLAUDE.md")).map(String::as_str),
            Some("memory-old")
        );
        assert_eq!(
            map.get(Path::new("/h/.claude/skills/coordinate/SKILL.md"))
                .map(String::as_str),
            Some("skill-old")
        );

        // The update loop rewrites the memory row's fingerprint; the rule row, absent from
        // `updates`, keeps its bytes untouched — skill/rule behavior is unchanged.
        let updates = vec![
            (PathBuf::from("/h/CLAUDE.md"), "memory-new".to_string()),
            (
                PathBuf::from("/h/.claude/skills/coordinate/SKILL.md"),
                "skill-new".to_string(),
            ),
        ];
        update_lock(&dir, &kinds, &updates).unwrap();

        let reloaded = load_emit_hash(&dir, &kinds).unwrap();
        assert_eq!(
            reloaded.get(Path::new("/h/CLAUDE.md")).map(String::as_str),
            Some("memory-new")
        );
        assert_eq!(
            reloaded
                .get(Path::new("/h/.claude/skills/coordinate/SKILL.md"))
                .map(String::as_str),
            Some("skill-new")
        );
        assert_eq!(
            reloaded
                .get(Path::new("/h/.claude/rules/rust.md"))
                .map(String::as_str),
            Some("rule-old")
        );
    }
}
