//! `temper diff` / `emit` — the drift engine.
//!
//! specs/architecture/20-surface.md, "Content-faithful, deterministically emitted (law 5)";
//! "Decision: `re-add` is retired — hand-edits route to the source" (a direct edit
//! to emitted output is drift routed to the authored source, never merged back).
//!
//! [`diff`] classifies every artifact into four read-only states (in-sync /
//! drifted / added / removed); [`emit`] compiles the surface out, re-emitting each
//! projection **whole** from the authored source and byte-deterministically —
//! verified by a double-emit comparison, so nondeterministic authoring is a loud
//! failure, never a silent churn. A hand-edited projection is overwritten: it is
//! drift routed to the source, surfaced by `config.stale`/the guard, not a merge.
//! [`place`] is the whole-file placement merge for artifacts temper *places* rather
//! than emits (specs/architecture/50-distribution.md, `install`); it keeps its own
//! three-state conflict detection until `install` rides emit's projection.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;
use toml_edit::{DocumentMut, Item, value};

use crate::builtin_kind;
use crate::check::Workspace;
use crate::hash::sha256_hex;
use crate::import;
use crate::kind::{BUILTIN_KINDS, CustomKind};

/// Errors raised while computing a drift report. A hard failure (a source path
/// errors for a reason other than "not found", which is the `removed` state) —
/// distinct from a drift *state*, which is a finding the report carries. A failed
/// harness re-scan surfaces as the underlying `import` error, flowing through the
/// `miette::Result` the way `import::run` does.
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

/// One artifact's drift state on the real-on-disk vs import-baseline axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftState {
    /// The source still hashes to the imported `source_hash` — no drift.
    InSync,
    /// The source still exists but its bytes changed since import.
    Drifted,
    /// A source the harness scan found on disk that the surface does not carry.
    Added,
    /// The recorded source path is gone from disk.
    Removed,
}

impl DriftState {
    /// The lower-case label used in the rendered report and stable for tests.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            DriftState::InSync => "in-sync",
            DriftState::Drifted => "drifted",
            DriftState::Added => "added",
            DriftState::Removed => "removed",
        }
    }
}

/// One row of a [`DriftReport`]: which artifact, of which kind, located where, in
/// which state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftEntry {
    /// The artifact kind — a built-in `"skill"`/`"rule"` or a registered custom
    /// kind's own name. A `String` because custom kinds carry a dynamic name the
    /// assembly declares, not one of a fixed built-in set.
    pub kind: String,
    /// The artifact name (its surface name for a known artifact, or the name the
    /// path structurally implies for an `added` one).
    pub name: String,
    /// The on-disk source path the state was judged against.
    pub source_path: PathBuf,
    /// The artifact's drift state.
    pub state: DriftState,
}

/// The typed result of a [`diff`]: every artifact's drift state, in a stable
/// order (per kind: the surface artifacts as loaded, then the freshly-discovered
/// `added` ones). Renders nothing itself — [`render`] turns it into text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftReport {
    /// Every classified artifact, across all kinds.
    pub entries: Vec<DriftEntry>,
}

/// A surface artifact reduced to the three columns drift needs: its name and the
/// provenance lock (where it came from, and what it hashed to at import).
struct SurfaceArtifact {
    name: String,
    source_path: PathBuf,
    source_hash: String,
}

/// Compare the imported `workspace` surface against the live `harness` on disk,
/// classifying every artifact into one of the four [`DriftState`]s.
///
/// Read-only: re-reads each source and re-scans the harness, but writes nothing.
/// See the module header for the per-state definitions.
///
/// `workspace_dir` is the surface root — its lock (`lock.toml`) carries the
/// `[[<kind>]]` provenance rows a custom kind's surface members are judged against
/// (custom members are not materialized in [`Workspace`]). `custom_kinds` are the
/// registered custom kinds whose `governs` locus is scanned beside the built-ins;
/// pass an empty slice for the built-in-only report.
pub fn diff(
    workspace: &Workspace,
    workspace_dir: &Path,
    harness: &Path,
    custom_kinds: &[CustomKind],
) -> miette::Result<DriftReport> {
    let mut entries = Vec::new();

    // Thread the parsed built-in kinds through discovery rather than re-resolving each
    // bare name at the scan: keyed by qualified identity, the set never collides on an
    // unrelated scan when two providers co-embed one bare name
    // (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider axis").
    let builtins = builtin_kind::definitions()?;
    let skill_kind = builtins
        .values()
        .find(|k| k.name == "skill")
        .expect("the built-in `skill` kind is embedded product source");
    let rule_kind = builtins
        .values()
        .find(|k| k.name == "rule")
        .expect("the built-in `rule` kind is embedded product source");

    let skills = workspace
        .skills()
        .iter()
        .map(|skill| SurfaceArtifact {
            name: skill.id.clone(),
            source_path: skill.provenance.source_path.clone(),
            source_hash: skill.provenance.source_hash.clone(),
        })
        .collect::<Vec<_>>();
    // The unified `governs`-keyed scan yields a skill's source `SKILL.md` directly.
    let skills_on_disk = import::discover_builtin(harness, skill_kind)?;
    entries.extend(classify("skill", &skills, &skills_on_disk)?);

    let rules = workspace
        .rules()
        .iter()
        .map(|rule| SurfaceArtifact {
            name: rule.id.clone(),
            source_path: rule.provenance.source_path.clone(),
            source_hash: rule.provenance.source_hash.clone(),
        })
        .collect::<Vec<_>>();
    let rules_on_disk = import::discover_builtin(harness, rule_kind)?;
    entries.extend(classify("rule", &rules, &rules_on_disk)?);

    // Each registered custom kind classifies at its own `governs` locus. Its surface
    // provenance is the `[[<kind>]]` lock rows (custom members live only in the lock,
    // not in `Workspace`), and its on-disk corpus is the same `governs`-keyed scan
    // `import` runs — so a hand-edited `specs/*.md` reconciles instead of the gate
    // reading a stale surface body (`specs/architecture/20-surface.md`, the hard core).
    for kind in custom_kinds {
        let surface = lock_surface_artifacts(workspace_dir, &kind.name)?;
        let on_disk = import::discover_kind_units(harness, &kind.governs)?;
        entries.extend(classify(&kind.name, &surface, &on_disk)?);
    }

    Ok(DriftReport { entries })
}

/// Classify one kind's surface artifacts against the source paths the harness
/// scan turned up.
///
/// Each surface artifact is re-read at its `source_path`: gone ⇒ `removed`,
/// unchanged hash ⇒ `in-sync`, changed hash ⇒ `drifted`. Then every scanned path
/// the surface does not already account for is `added`.
fn classify(
    kind: &str,
    surface: &[SurfaceArtifact],
    on_disk: &[PathBuf],
) -> miette::Result<Vec<DriftEntry>> {
    let mut entries = Vec::new();
    let surface_paths: HashSet<&Path> = surface.iter().map(|a| a.source_path.as_path()).collect();

    for artifact in surface {
        let state = match fs::read(&artifact.source_path) {
            Ok(bytes) if sha256_hex(&bytes) == artifact.source_hash => DriftState::InSync,
            Ok(_) => DriftState::Drifted,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => DriftState::Removed,
            Err(source) => {
                return Err(DriftError::Read {
                    path: artifact.source_path.clone(),
                    source,
                }
                .into());
            }
        };
        entries.push(DriftEntry {
            kind: kind.to_string(),
            name: artifact.name.clone(),
            source_path: artifact.source_path.clone(),
            state,
        });
    }

    for path in on_disk {
        if !surface_paths.contains(path.as_path()) {
            entries.push(DriftEntry {
                kind: kind.to_string(),
                name: added_name(kind, path),
                source_path: path.clone(),
                state: DriftState::Added,
            });
        }
    }

    Ok(entries)
}

/// Derive a display name for an `added` source the surface has not parsed: a
/// skill is named by its directory (the `SKILL.md`'s parent), a rule by its file
/// stem. A scan, not a parse — the structural name, not the frontmatter one
/// (which only a full read would yield).
fn added_name(kind: &str, source_path: &Path) -> String {
    let component = if kind == "skill" {
        source_path.parent().and_then(Path::file_name)
    } else {
        source_path.file_stem()
    };
    component
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Read one custom kind's surface artifacts from the `[[<kind>]]` lock rows —
/// name + provenance (`source_path`, `source_hash`) — the three columns [`classify`]
/// judges against. Custom members are not materialized in [`Workspace`], so the lock
/// is their surface provenance of record (`specs/architecture/20-surface.md`, the lock as
/// state-of-record). A kind with no rows (or no lock array yet) yields an empty list.
fn lock_surface_artifacts(
    workspace_dir: &Path,
    kind: &str,
) -> Result<Vec<SurfaceArtifact>, DriftError> {
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

    let mut out = Vec::new();
    if let Some(rows) = doc.get(kind).and_then(Item::as_array_of_tables) {
        for row in rows.iter() {
            if let (Some(name), Some(source_path), Some(source_hash)) = (
                row.get("name").and_then(Item::as_str),
                row.get("source_path").and_then(Item::as_str),
                row.get("source_hash").and_then(Item::as_str),
            ) {
                out.push(SurfaceArtifact {
                    name: name.to_string(),
                    source_path: PathBuf::from(source_path),
                    source_hash: source_hash.to_string(),
                });
            }
        }
    }
    Ok(out)
}

/// Render a drift report for the terminal: one `<state>  <kind>  <name>` line per
/// entry, in the report's stable order.
#[must_use]
pub fn render(report: &DriftReport) -> String {
    let mut out = String::new();
    for entry in &report.entries {
        out.push_str(&format!(
            "{:<7}  {:<5}  {}\n",
            entry.state.label(),
            entry.kind,
            entry.name
        ));
    }
    out
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
    let emit_hashes = load_emit_hash(workspace_dir)?;

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
        update_lock(workspace_dir, &updates)?;
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

    let desired = project_bytes(&projection.fields, &projection.body);

    // Double-emit determinism (`specs/architecture/20-surface.md`, law 5): a second
    // projection over the same surface must be byte-identical. Nondeterministic
    // authoring (a timestamp, an unordered map surfacing into a field) is a loud
    // failure here, never a silent churn the next `emit` would rewrite.
    if project_bytes(&projection.fields, &projection.body) != desired {
        return Err(DriftError::Nondeterministic {
            path: projection.source_path.clone(),
        });
    }

    // Read the committed projection only to tell `Emitted` from the idempotent no-op
    // — never to merge. An absent source is not a conflict: emit writes it.
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
/// frontmatter block carrying every desired field in order, then the surface body
/// byte-for-byte.
///
/// The projection is *generated*, not patched (`specs/architecture/20-surface.md`, "Decision: the
/// projection is re-emitted; the surface is patched") — the on-disk source is never
/// read here, so a hand-edited frontmatter comment or reordered field is not
/// preserved (that is drift, routed to the authored source). An artifact with
/// no fields (a rule that carries no `paths`/unknown keys) projects to its body
/// alone — no empty frontmatter block.
fn project_bytes(fields: &[(String, JsonValue)], body: &str) -> String {
    if fields.is_empty() {
        return body.to_string();
    }
    let mut frontmatter = String::new();
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

/// Read the emit fingerprints from `<workspace_dir>/lock.toml`, keyed by
/// source path. Covers the built-in `[[skill]]`/`[[rule]]` rows — the kinds `emit`
/// projects. A row without an `emit_hash` column (a lock predating the field) is
/// simply absent, and the caller falls back to the source hash.
fn load_emit_hash(
    workspace_dir: &Path,
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
    for &kind in BUILTIN_KINDS {
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
/// matching each `[[skill]]`/`[[rule]]` row by its `source_path`. Format-preserving
/// via `toml_edit` — only the `emit_hash` values change.
fn update_lock(workspace_dir: &Path, updates: &[(PathBuf, String)]) -> Result<(), DriftError> {
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

    for &kind in BUILTIN_KINDS {
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

    const SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

    const RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

    /// Write a one-skill + one-rule harness and import it into a fresh surface,
    /// returning `(harness, workspace)`.
    fn imported(label: &str) -> (PathBuf, PathBuf) {
        let harness = tmpdir(&format!("{label}-src"));
        let skill = harness.join(".claude").join("skills").join("coordinate");
        fs::create_dir_all(&skill).unwrap();
        fs::write(skill.join("SKILL.md"), SKILL).unwrap();
        let rules = harness.join(".claude").join("rules");
        fs::create_dir_all(&rules).unwrap();
        fs::write(rules.join("rust.md"), RULE).unwrap();

        let into = tmpdir(&format!("{label}-into"));
        import::run(&harness, &into).unwrap();
        (harness, into)
    }

    /// Look up the single entry for `name`, asserting it exists exactly once.
    fn entry<'a>(report: &'a DriftReport, name: &str) -> &'a DriftEntry {
        let mut matches = report.entries.iter().filter(|e| e.name == name);
        let found = matches.next().expect("entry should exist");
        assert!(matches.next().is_none(), "entry {name} should be unique");
        found
    }

    #[test]
    fn unchanged_harness_is_all_in_sync() {
        let (harness, into) = imported("clean");
        let ws = Workspace::load(&into).unwrap();

        let report = diff(&ws, &into, &harness, &[]).unwrap();

        assert_eq!(report.entries.len(), 2);
        assert!(report.entries.iter().all(|e| e.state == DriftState::InSync));
    }

    #[test]
    fn edited_source_is_drifted_others_stay_in_sync() {
        let (harness, into) = imported("edit");
        let ws = Workspace::load(&into).unwrap();

        // Mutate one source after import; its hash no longer matches the baseline.
        let skill_md = harness
            .join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md");
        let edited = fs::read_to_string(&skill_md).unwrap() + "\nAn extra line.\n";
        fs::write(&skill_md, edited).unwrap();

        let report = diff(&ws, &into, &harness, &[]).unwrap();

        assert_eq!(entry(&report, "coordinate").state, DriftState::Drifted);
        assert_eq!(entry(&report, "rust").state, DriftState::InSync);
    }

    #[test]
    fn new_source_is_added() {
        let (harness, into) = imported("add");
        let ws = Workspace::load(&into).unwrap();

        // A rule that exists on disk but the surface never imported.
        fs::write(
            harness.join(".claude").join("rules").join("extra.md"),
            "# Extra\n\nA rule added after import.\n",
        )
        .unwrap();

        let report = diff(&ws, &into, &harness, &[]).unwrap();

        let added = entry(&report, "extra");
        assert_eq!(added.state, DriftState::Added);
        assert_eq!(added.kind, "rule");
    }

    #[test]
    fn deleted_source_is_removed() {
        let (harness, into) = imported("remove");
        let ws = Workspace::load(&into).unwrap();

        // Delete a source the surface imported: its path is gone from disk.
        fs::remove_dir_all(harness.join(".claude").join("skills").join("coordinate")).unwrap();

        let report = diff(&ws, &into, &harness, &[]).unwrap();

        assert_eq!(entry(&report, "coordinate").state, DriftState::Removed);
        assert_eq!(entry(&report, "rust").state, DriftState::InSync);
    }

    #[test]
    fn render_lists_each_state_label() {
        let report = DriftReport {
            entries: vec![
                DriftEntry {
                    kind: "skill".into(),
                    name: "coordinate".into(),
                    source_path: PathBuf::from("skills/coordinate/SKILL.md"),
                    state: DriftState::Drifted,
                },
                DriftEntry {
                    kind: "rule".into(),
                    name: "rust".into(),
                    source_path: PathBuf::from(".claude/rules/rust.md"),
                    state: DriftState::InSync,
                },
            ],
        };

        let rendered = render(&report);
        assert!(rendered.contains("drifted"));
        assert!(rendered.contains("coordinate"));
        assert!(rendered.contains("in-sync"));
        assert!(rendered.contains("rust"));
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
}
