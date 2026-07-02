//! `temper diff` / `apply` — the three-state drift engine.
//!
//! Implements the drift engine of `specs/20-surface.md` ("Drift / apply — three
//! states, never two"). It tracks three states — **desired** (the edited
//! surface), the **last-applied fingerprint** (the source as `temper` last left
//! it, from the lock), and **real on-disk** — so the write direction can tell a
//! human surface edit from a world drift and merge rather than clobber.
//!
//! ## [`diff`] — the read-only report
//!
//! [`diff`] loads nothing and writes nothing of its own — it takes an already
//! loaded [`Workspace`] (the surface + its provenance lock) and a live harness
//! path, then classifies every artifact into one of four states:
//!
//! - **in-sync** — the source still hashes to the imported [`import_hash`].
//! - **drifted** — the source still exists but its bytes changed since import.
//! - **removed** — the recorded source path is gone from disk.
//! - **added** — a source the per-kind scan finds on disk that no surface
//!   artifact accounts for.
//!
//! The first three iterate the surface and re-read each `provenance.source_path`;
//! the last re-runs `import`'s own per-kind discovery
//! ([`discover_skill_dirs`](crate::import::discover_skill_dirs) and siblings) so
//! the "what's on disk" question is answered by the exact scan that imported it.
//! Drift is a report, not a gate — the command exits zero regardless.
//!
//! ## [`apply`] — the write direction
//!
//! [`apply`] projects the surface back onto the harness sources. It is
//! **patch-not-re-emit**: for each artifact it splits the on-disk source into its
//! frontmatter and body, replaces the body byte-faithfully with the surface body,
//! and patches *only the frontmatter fields whose value changed* — every untouched
//! byte (comments, key order, whitespace) survives exactly as the human wrote it
//! (`specs/20-surface.md`, "write-back patches changed fields, never re-emits").
//! No comment-preserving YAML editor exists in Rust, so a changed scalar/sequence
//! field's own formatting is re-rendered while its neighbours are left verbatim.
//!
//! The merge is the hard core. For each artifact `apply` compares the desired
//! projection against real-on-disk and the last-applied fingerprint:
//!
//! - projection **equals** on-disk ⇒ [`ApplyOutcome::Unchanged`] (idempotent
//!   no-op; the fingerprint is reconciled to the current bytes).
//! - projection **differs** and on-disk still hashes to the last-applied
//!   fingerprint (no world drift) ⇒ patch the source, [`ApplyOutcome::Applied`],
//!   and record the new fingerprint.
//! - projection **differs** and on-disk drifted from the last-applied fingerprint
//!   ⇒ [`ApplyOutcome::Conflicted`]: the world changed the source out from under
//!   the surface, so `apply` surfaces the choice rather than clobbering — it
//!   writes nothing and leaves the fingerprint untouched.
//!
//! A `--dry-run` computes every outcome but writes neither the sources nor the
//! updated lock. Like `diff`, `apply` covers the built-in kinds (skill, rule).
//!
//! ## [`re_add`] — the on-disk → surface direction
//!
//! [`re_add`] is the third drift direction and the one that keeps direct on-disk
//! editing first-class (`specs/20-surface.md`, "the surface is the source of
//! truth" — `re-add` reconciles the harness edits a human made outside the
//! surface). Where `apply` pushes the surface *out*, `re_add` pulls the harness
//! *in*: it runs [`diff`]'s four-state classification, then for every **drifted**
//! or **added** built-in artifact it re-parses the live source through the
//! skill/rule loaders and re-projects it into the surface tree via `import`'s own
//! per-kind writers ([`import::import_skill`]/[`import::import_rule`]) — the single
//! round-trip write path, never a second implementation. Each written artifact's
//! lock row is refreshed to the current source bytes (its `import_hash` and
//! `last_applied` fingerprint), an **added** source gaining a brand-new row. An
//! **in-sync** artifact is left untouched (a no-op), and a
//! **removed** one is skipped — `re_add` only pulls in what is actually on disk;
//! reconciling a deletion is a different direction. Like the other two, it covers
//! the built-in kinds; generic custom-kind re-add is follow-on work.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use gray_matter::Pod;
use gray_matter::engine::{Engine, YAML};
use serde_json::Value as JsonValue;
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::check::Workspace;
use crate::hash::sha256_hex;
use crate::import;
use crate::rule::Rule;
use crate::skill::Skill;

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

    /// A patched source could not be written back to the harness during `apply`.
    #[error("failed to write source {path}")]
    #[diagnostic(code(temper::drift::write))]
    Write {
        /// The destination source path that failed to write.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
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
    /// The source still hashes to the imported `import_hash` — no drift.
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
    /// The artifact kind — `"skill"` or `"rule"`. Only the built-in kinds have a
    /// drift axis today; generic custom-kind drift is future work alongside `apply`.
    pub kind: &'static str,
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
    import_hash: String,
}

/// Compare the imported `workspace` surface against the live `harness` on disk,
/// classifying every artifact into one of the four [`DriftState`]s.
///
/// Read-only: re-reads each source and re-scans the harness, but writes nothing.
/// See the module header for the per-state definitions.
pub fn diff(workspace: &Workspace, harness: &Path) -> miette::Result<DriftReport> {
    let mut entries = Vec::new();

    let skills = workspace
        .skills
        .iter()
        .map(|skill| SurfaceArtifact {
            name: skill.name.clone(),
            source_path: skill.provenance.source_path.clone(),
            import_hash: skill.provenance.import_hash.clone(),
        })
        .collect::<Vec<_>>();
    // A skill's source is the `SKILL.md` inside its discovered directory.
    let skills_on_disk = import::discover_skill_dirs(harness)?
        .iter()
        .map(|dir| dir.join("SKILL.md"))
        .collect::<Vec<_>>();
    entries.extend(classify("skill", &skills, &skills_on_disk)?);

    let rules = workspace
        .rules
        .iter()
        .map(|rule| SurfaceArtifact {
            name: rule.name.clone(),
            source_path: rule.provenance.source_path.clone(),
            import_hash: rule.provenance.import_hash.clone(),
        })
        .collect::<Vec<_>>();
    let rules_on_disk = import::discover_rule_files(harness)?;
    entries.extend(classify("rule", &rules, &rules_on_disk)?);

    Ok(DriftReport { entries })
}

/// Classify one kind's surface artifacts against the source paths the harness
/// scan turned up.
///
/// Each surface artifact is re-read at its `source_path`: gone ⇒ `removed`,
/// unchanged hash ⇒ `in-sync`, changed hash ⇒ `drifted`. Then every scanned path
/// the surface does not already account for is `added`.
fn classify(
    kind: &'static str,
    surface: &[SurfaceArtifact],
    on_disk: &[PathBuf],
) -> miette::Result<Vec<DriftEntry>> {
    let mut entries = Vec::new();
    let surface_paths: HashSet<&Path> = surface.iter().map(|a| a.source_path.as_path()).collect();

    for artifact in surface {
        let state = match fs::read(&artifact.source_path) {
            Ok(bytes) if sha256_hex(&bytes) == artifact.import_hash => DriftState::InSync,
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
            kind,
            name: artifact.name.clone(),
            source_path: artifact.source_path.clone(),
            state,
        });
    }

    for path in on_disk {
        if !surface_paths.contains(path.as_path()) {
            entries.push(DriftEntry {
                kind,
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
// apply — the write direction (`specs/20-surface.md`, the hard core)
// ---------------------------------------------------------------------------

/// Options controlling an [`apply`] run.
#[derive(Debug, Clone, Copy, Default)]
pub struct ApplyOptions {
    /// When set, compute every outcome and report it but write nothing — neither
    /// the patched harness sources nor the updated lock fingerprints.
    pub dry_run: bool,
}

/// One artifact's outcome from an [`apply`]: what the three-state merge decided.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyOutcome {
    /// The source was patched to match the surface (or, under `--dry-run`, would
    /// have been). Only reachable when the source had *not* drifted from the
    /// last-applied fingerprint — a clean surface edit.
    Applied,
    /// The source already matched the surface projection; nothing to write. The
    /// idempotent no-op — a re-run of a clean apply lands here for every artifact.
    Unchanged,
    /// The source drifted from the last-applied fingerprint *and* the surface wants
    /// something different — a genuine conflict. `apply` surfaces the choice rather
    /// than clobbering: it writes nothing. (A source removed from disk since the
    /// last apply is reported here too — the world changed it out from under us.)
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

/// One row of an [`ApplyReport`]: which artifact, of which kind, located where, and
/// the outcome the merge produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyEntry {
    /// The artifact kind — `"skill"` or `"rule"`.
    pub kind: &'static str,
    /// The artifact name (its surface name).
    pub name: String,
    /// The on-disk source path the projection targeted.
    pub source_path: PathBuf,
    /// What `apply` did (or would do, under `--dry-run`) for this artifact.
    pub outcome: ApplyOutcome,
}

/// The typed result of an [`apply`]: every artifact's outcome, in the workspace's
/// stable load order (skills then rules, each name-sorted). Renders nothing itself
/// — [`render_apply`] turns it into text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyReport {
    /// Every projected artifact, across the built-in kinds.
    pub entries: Vec<ApplyEntry>,
}

/// The desired projection of one surface artifact: its identity, the last-applied
/// fingerprint the merge compares against, the ordered header fields the surface
/// wants the frontmatter to express, and the byte-faithful body.
struct Projection {
    kind: &'static str,
    name: String,
    source_path: PathBuf,
    /// The fingerprint of the source as `temper` last left it (from the lock, or
    /// the import hash as a baseline when the lock predates this field).
    last_applied: String,
    /// The desired header fields in canonical order (known fields first, then the
    /// preserved unknown keys). Each value is compared as JSON against the on-disk
    /// frontmatter to decide whether that one field changed.
    fields: Vec<(String, JsonValue)>,
    /// The desired body — the surface body, projected byte-faithfully.
    body: String,
}

/// Project the loaded `workspace` surface back onto the harness sources, patching
/// only changed frontmatter fields over the three-state merge.
///
/// `workspace_dir` is the surface root — where the lock (`lock.toml`) carrying the
/// last-applied fingerprints lives. Each artifact is written to its recorded
/// `provenance.source_path` (where `import` read it from). See the module header
/// for the per-outcome merge rules; nothing is written under `options.dry_run`.
pub fn apply(
    workspace: &Workspace,
    workspace_dir: &Path,
    options: ApplyOptions,
) -> miette::Result<ApplyReport> {
    let last_applied = load_last_applied(workspace_dir)?;

    let mut projections = Vec::new();
    for skill in &workspace.skills {
        projections.push(Projection {
            kind: "skill",
            name: skill.name.clone(),
            source_path: skill.provenance.source_path.clone(),
            last_applied: fingerprint(&last_applied, skill.provenance.source_path.as_path())
                .unwrap_or_else(|| skill.provenance.import_hash.clone()),
            fields: skill_fields(skill),
            body: skill.body.clone(),
        });
    }
    for rule in &workspace.rules {
        projections.push(Projection {
            kind: "rule",
            name: rule.name.clone(),
            source_path: rule.provenance.source_path.clone(),
            last_applied: fingerprint(&last_applied, rule.provenance.source_path.as_path())
                .unwrap_or_else(|| rule.provenance.import_hash.clone()),
            fields: rule_fields(rule),
            body: rule.body.clone(),
        });
    }

    let mut entries = Vec::new();
    // source_path -> new fingerprint to record for Applied/Unchanged artifacts.
    let mut updates: Vec<(PathBuf, String)> = Vec::new();
    for projection in &projections {
        let (entry, update) = project_one(projection, options.dry_run)?;
        if let Some(fingerprint) = update {
            updates.push((projection.source_path.clone(), fingerprint));
        }
        entries.push(entry);
    }

    if !options.dry_run && !updates.is_empty() {
        update_lock(workspace_dir, &updates)?;
    }

    Ok(ApplyReport { entries })
}

/// Merge one artifact against real-on-disk, returning its [`ApplyEntry`] and, when
/// the source is in a reconciled state (applied or unchanged), the fingerprint to
/// record for it. A conflict records nothing — the lock is left untouched so the
/// next run still sees the drift.
fn project_one(
    projection: &Projection,
    dry_run: bool,
) -> Result<(ApplyEntry, Option<String>), DriftError> {
    let row = |outcome| ApplyEntry {
        kind: projection.kind,
        name: projection.name.clone(),
        source_path: projection.source_path.clone(),
        outcome,
    };

    // Read real-on-disk. A source removed since the last apply is a world drift we
    // must not silently re-create — surface it as a conflict.
    let real_bytes = match fs::read(&projection.source_path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok((row(ApplyOutcome::Conflicted), None));
        }
        Err(source) => {
            return Err(DriftError::Read {
                path: projection.source_path.clone(),
                source,
            });
        }
    };
    // The source drifted into non-UTF-8 bytes; we cannot faithfully patch it.
    let Ok(real) = String::from_utf8(real_bytes) else {
        return Ok((row(ApplyOutcome::Conflicted), None));
    };

    let desired = project_bytes(&projection.fields, &projection.body, &real);

    if desired == real {
        // The projection already sits on disk. If the fingerprint is stale (a benign
        // world edit that happens to match the surface), reconcile it to the current
        // bytes so it stops reading as drift; otherwise there is nothing to record and
        // the lock is left alone.
        let real_hash = sha256_hex(real.as_bytes());
        let update = (real_hash != projection.last_applied).then_some(real_hash);
        return Ok((row(ApplyOutcome::Unchanged), update));
    }

    if sha256_hex(real.as_bytes()) == projection.last_applied {
        // No world drift since the last apply: the on-disk source is exactly what
        // `temper` last wrote, so patching the surface edit onto it is clean.
        if !dry_run {
            fs::write(&projection.source_path, desired.as_bytes()).map_err(|source| {
                DriftError::Write {
                    path: projection.source_path.clone(),
                    source,
                }
            })?;
        }
        Ok((
            row(ApplyOutcome::Applied),
            Some(sha256_hex(desired.as_bytes())),
        ))
    } else {
        // The world drifted *and* the surface wants something else: a genuine
        // conflict. Do not clobber — surface the choice, write nothing.
        Ok((row(ApplyOutcome::Conflicted), None))
    }
}

/// The desired header fields of a skill, in canonical order: the known scalars
/// (only those present), then the preserved unknown keys (already key-sorted in
/// [`Skill::extra`]). Mirrors the order [`Skill::to_document`] projects.
fn skill_fields(skill: &Skill) -> Vec<(String, JsonValue)> {
    let mut fields = vec![
        ("name".to_string(), JsonValue::from(skill.name.clone())),
        (
            "description".to_string(),
            JsonValue::from(skill.description.clone()),
        ),
    ];
    if let Some(version) = &skill.version {
        fields.push(("version".to_string(), JsonValue::from(version.clone())));
    }
    if let Some(license) = &skill.license {
        fields.push(("license".to_string(), JsonValue::from(license.clone())));
    }
    for (key, value) in &skill.extra {
        fields.push((key.clone(), value.clone()));
    }
    fields
}

/// The desired header fields of a rule: the optional `paths` sequence, then the
/// preserved unknown keys. A no-frontmatter rule yields an empty field set, so its
/// projection is the body alone. Mirrors [`Rule::to_document`].
fn rule_fields(rule: &Rule) -> Vec<(String, JsonValue)> {
    let mut fields = Vec::new();
    if let Some(paths) = &rule.paths {
        fields.push(("paths".to_string(), JsonValue::from(paths.clone())));
    }
    for (key, value) in &rule.extra {
        fields.push((key.clone(), value.clone()));
    }
    fields
}

/// Project the desired header + body onto the real on-disk source, byte-faithfully
/// except for the frontmatter fields that changed.
///
/// The on-disk source is split into its frontmatter and body; the body is replaced
/// with the surface body and the frontmatter is patched field-by-field
/// ([`patch_frontmatter`]). A source with no frontmatter takes the body directly
/// (an empty desired header) or a freshly synthesised block (a rule the surface
/// gave `paths`/unknown keys but disk has none).
fn project_bytes(fields: &[(String, JsonValue)], body: &str, real: &str) -> String {
    match split_source(real) {
        Some(split) => {
            let patched = patch_frontmatter(&split.inner, fields);
            format!("{}{}{}{}", split.open, patched, split.close, body)
        }
        None if fields.is_empty() => body.to_string(),
        None => {
            let mut frontmatter = String::new();
            for (key, value) in fields {
                frontmatter.push_str(&render_field(key, value));
            }
            format!("---\n{frontmatter}---\n{body}")
        }
    }
}

/// A source `.md` split around its frontmatter so a patched frontmatter can be
/// reassembled without re-emitting the delimiters. `apply` replaces the body
/// wholesale with the surface body, so only the header region is retained here.
struct SourceSplit {
    /// The opening delimiter line, including its trailing newline (`"---\n"`).
    open: String,
    /// The frontmatter text between the delimiters (no delimiter lines).
    inner: String,
    /// The closing delimiter line, exactly as written (including its newline, if any).
    close: String,
}

/// Split a source into [`SourceSplit`], or `None` when it has no leading
/// `---`-delimited frontmatter block. Mirrors the delimiter detection the skill/rule
/// loaders use, but keeps the delimiter lines so `apply` can reassemble the file
/// without re-emitting them.
fn split_source(raw: &str) -> Option<SourceSplit> {
    let (first, rest) = raw.split_once('\n')?;
    if first.trim_end() != "---" {
        return None;
    }

    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.trim_end() == "---" {
            return Some(SourceSplit {
                open: format!("{first}\n"),
                inner: rest[..offset].to_string(),
                close: line.to_string(),
            });
        }
        offset += line.len();
    }

    // Opening delimiter but no close — not a frontmatter block.
    None
}

/// Patch the desired fields into a frontmatter's `inner` text, changing only the
/// fields whose value differs and leaving every other byte — comments, blank lines,
/// key order, the untouched fields' exact formatting — verbatim.
///
/// The inner text is parsed into top-level segments: a *field* segment (a `key:`
/// line at column 0 plus its indented continuation lines) or a *verbatim* segment
/// (a comment, blank line, or any other column-0 line). Each field the surface
/// still carries is compared as JSON against its on-disk value: equal ⇒ the segment
/// is kept verbatim; changed ⇒ it is re-rendered. A field the surface dropped is
/// removed; a field the surface added that disk lacks is appended in desired order.
fn patch_frontmatter(inner: &str, desired: &[(String, JsonValue)]) -> String {
    let on_disk = parse_frontmatter_json(inner);
    let mut out = String::new();
    let mut emitted: HashSet<String> = HashSet::new();

    for segment in parse_segments(inner) {
        match segment {
            Segment::Verbatim(text) => out.push_str(&text),
            Segment::Field { key, text } => {
                if let Some((_, wanted)) = desired.iter().find(|(k, _)| *k == key) {
                    let unchanged = on_disk.get(&key).is_some_and(|current| current == wanted);
                    if unchanged {
                        out.push_str(&text);
                    } else {
                        out.push_str(&render_field(&key, wanted));
                    }
                    emitted.insert(key);
                }
                // A key the surface dropped is simply not re-emitted.
            }
        }
    }

    // Fields the surface carries that disk did not — append in desired order.
    for (key, value) in desired {
        if !emitted.contains(key.as_str()) {
            out.push_str(&render_field(key, value));
        }
    }

    out
}

/// A top-level segment of a frontmatter's inner text — either a field (a `key:`
/// line plus its indented continuation) or a run of verbatim bytes (comments,
/// blank lines, anything else at column 0) preserved untouched.
enum Segment {
    Verbatim(String),
    Field { key: String, text: String },
}

/// Parse a frontmatter's inner text into ordered [`Segment`]s. Concatenating every
/// segment's text reproduces `inner` exactly, so an all-verbatim / no-change patch
/// is byte-identical.
fn parse_segments(inner: &str) -> Vec<Segment> {
    let lines: Vec<&str> = inner.split_inclusive('\n').collect();
    let mut segments = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let Some(key) = top_level_key(lines[i]) {
            let mut text = lines[i].to_string();
            i += 1;
            // Continuation lines of a block value are indented; a column-0 line
            // (next key, comment, or blank) ends the segment.
            while i < lines.len() && is_indented(lines[i]) {
                text.push_str(lines[i]);
                i += 1;
            }
            segments.push(Segment::Field { key, text });
        } else {
            segments.push(Segment::Verbatim(lines[i].to_string()));
            i += 1;
        }
    }
    segments
}

/// The top-level YAML key a line declares (`name: demo` -> `name`), or `None` when
/// the line is indented, blank, a comment, or carries no `key:`.
fn top_level_key(line: &str) -> Option<String> {
    let content = line.strip_suffix('\n').unwrap_or(line);
    let first = content.chars().next()?;
    if first.is_whitespace() || first == '#' {
        return None;
    }
    let colon = content.find(':')?;
    let key = &content[..colon];
    if key.is_empty() || key.contains('#') {
        return None;
    }
    Some(key.to_string())
}

/// Whether a line begins with whitespace — a continuation of the preceding block
/// value. A blank line (`"\n"`) is not indented, so it ends the current field.
fn is_indented(line: &str) -> bool {
    line.starts_with(' ') || line.starts_with('\t')
}

/// Render one frontmatter field as `key: <value>\n`. The value is emitted as
/// compact JSON, which is valid YAML flow — a double-quoted string, a bare number
/// or bool, a `[..]` sequence — so it round-trips back to the same JSON on the next
/// parse (keeping `apply` idempotent). Only *changed* or *added* fields are rendered
/// this way; unchanged fields keep their original block formatting verbatim.
fn render_field(key: &str, value: &JsonValue) -> String {
    // Serializing a `serde_json::Value` is infallible in practice; fall back to a
    // null literal rather than panic on the unreachable error path.
    let rendered = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
    format!("{key}: {rendered}\n")
}

/// Parse a frontmatter's inner text into a JSON map for value comparison, reusing
/// the same YAML engine the loaders parse with. A non-mapping frontmatter yields an
/// empty map (every field then reads as "added", never "unchanged").
fn parse_frontmatter_json(inner: &str) -> std::collections::HashMap<String, JsonValue> {
    let mut out = std::collections::HashMap::new();
    if let Pod::Hash(hash) = YAML::parse(inner.trim()) {
        for (key, pod) in hash {
            out.insert(key, pod.into());
        }
    }
    out
}

/// Read the last-applied fingerprints from `<workspace_dir>/lock.toml`, keyed by
/// source path. Covers the built-in `[[skill]]`/`[[rule]]` rows — the kinds `apply`
/// projects. A row without a `last_applied` column (a lock predating the field) is
/// simply absent, and the caller falls back to the import hash.
fn load_last_applied(
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
    for kind in ["skill", "rule"] {
        let Some(rows) = doc.get(kind).and_then(Item::as_array_of_tables) else {
            continue;
        };
        for row in rows.iter() {
            if let (Some(source_path), Some(fingerprint)) = (
                row.get("source_path").and_then(Item::as_str),
                row.get("last_applied").and_then(Item::as_str),
            ) {
                map.insert(PathBuf::from(source_path), fingerprint.to_string());
            }
        }
    }
    Ok(map)
}

/// Look up one source path's last-applied fingerprint in the loaded map.
fn fingerprint(map: &std::collections::HashMap<PathBuf, String>, source: &Path) -> Option<String> {
    map.get(source).cloned()
}

/// Write the reconciled fingerprints back into `<workspace_dir>/lock.toml` in place,
/// matching each `[[skill]]`/`[[rule]]` row by its `source_path`. Format-preserving
/// via `toml_edit` — only the `last_applied` values change.
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

    for kind in ["skill", "rule"] {
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
                row["last_applied"] = value(fingerprint.clone());
            }
        }
    }

    fs::write(&path, doc.to_string()).map_err(|source| DriftError::Write { path, source })
}

/// Render an apply report for the terminal: one `<outcome>  <kind>  <name>` line per
/// entry in the report's stable order, then a one-line tally.
#[must_use]
pub fn render_apply(report: &ApplyReport) -> String {
    let mut out = String::new();
    let (mut applied, mut unchanged, mut conflicted) = (0u32, 0u32, 0u32);
    for entry in &report.entries {
        match entry.outcome {
            ApplyOutcome::Applied => applied += 1,
            ApplyOutcome::Unchanged => unchanged += 1,
            ApplyOutcome::Conflicted => conflicted += 1,
        }
        out.push_str(&format!(
            "{:<10}  {:<5}  {}\n",
            entry.outcome.label(),
            entry.kind,
            entry.name
        ));
    }
    out.push_str(&format!(
        "\n{applied} applied, {unchanged} unchanged, {conflicted} conflicted\n"
    ));
    out
}

// ---------------------------------------------------------------------------
// re-add — the on-disk -> surface direction (`specs/20-surface.md`, the hard core)
// ---------------------------------------------------------------------------

/// One artifact's outcome from a [`re_add`]: how its live on-disk source was
/// reconciled back into the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReAddOutcome {
    /// A **drifted** source was pulled into an existing surface artifact — its
    /// member document (header and body) rewritten, and its lock row's fingerprints
    /// refreshed to the current source bytes.
    Reconciled,
    /// An **added** on-disk source the surface did not carry became a new surface
    /// artifact and gained a new lock row.
    Added,
    /// The source still hashes to the import baseline — nothing to pull in.
    Unchanged,
}

impl ReAddOutcome {
    /// The lower-case label used in the rendered report and stable for tests.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            ReAddOutcome::Reconciled => "reconciled",
            ReAddOutcome::Added => "added",
            ReAddOutcome::Unchanged => "unchanged",
        }
    }
}

/// One row of a [`ReAddReport`]: which artifact, of which kind, located where, and
/// what `re_add` did with its on-disk source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReAddEntry {
    /// The artifact kind — `"skill"` or `"rule"`.
    pub kind: &'static str,
    /// The artifact name — the name the freshly parsed source carries for a
    /// reconciled/added artifact, or its surface name for an unchanged one.
    pub name: String,
    /// The on-disk source path that was pulled into the surface.
    pub source_path: PathBuf,
    /// What `re_add` did for this artifact.
    pub outcome: ReAddOutcome,
}

/// The typed result of a [`re_add`]: every classified artifact's outcome, in the
/// [`diff`] report's stable order. Renders nothing itself — [`render_readd`] turns
/// it into text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReAddReport {
    /// Every reconciled, added, or unchanged artifact, across the built-in kinds.
    pub entries: Vec<ReAddEntry>,
}

/// Reconcile the live `harness` on disk back into the loaded `workspace` surface,
/// pulling every drifted or added built-in source into the surface tree and
/// refreshing the lock.
///
/// `workspace_dir` is the surface root — where the artifact trees and the lock
/// (`lock.toml`) live. The classification is [`diff`]'s own (so "what's on disk"
/// is answered by the exact scan `import` used); the write is `import`'s own
/// per-kind projection. A drifted or added source is re-parsed and re-projected;
/// an in-sync source is a no-op; a removed source is skipped (there is nothing on
/// disk to pull in). Writes only when something changed — an all-in-sync harness
/// leaves every surface byte identical.
pub fn re_add(
    workspace: &Workspace,
    workspace_dir: &Path,
    harness: &Path,
) -> miette::Result<ReAddReport> {
    let classification = diff(workspace, harness)?;

    let mut entries = Vec::new();
    // The rows to fold into the lock, tagged with their kind's array-of-tables key.
    let mut updates: Vec<(&'static str, import::RollupEntry)> = Vec::new();

    for entry in &classification.entries {
        // A removed source has nothing on disk to reconcile in — skip it entirely
        // (pruning the orphaned surface artifact is a separate direction).
        if entry.state == DriftState::Removed {
            continue;
        }
        if entry.state == DriftState::InSync {
            entries.push(ReAddEntry {
                kind: entry.kind,
                name: entry.name.clone(),
                source_path: entry.source_path.clone(),
                outcome: ReAddOutcome::Unchanged,
            });
            continue;
        }

        // Drifted or added: re-project the live source through `import`'s writer for
        // its kind. A skill's source is the `SKILL.md`, so its surface directory is
        // that file's parent (which the discovery scan always yields).
        let rollup = match entry.kind {
            "skill" => {
                let dir = entry.source_path.parent().unwrap_or_else(|| Path::new("."));
                import::import_skill(dir, workspace_dir)?
            }
            "rule" => import::import_rule(&entry.source_path, workspace_dir)?,
            // No other built-in kind has a drift axis today (mirrors `apply`).
            _ => continue,
        };

        let outcome = if entry.state == DriftState::Drifted {
            ReAddOutcome::Reconciled
        } else {
            ReAddOutcome::Added
        };
        entries.push(ReAddEntry {
            kind: entry.kind,
            // The name the parsed source carries — for a skill this is its
            // frontmatter `name`, which is what the surface directory was written
            // under, not necessarily the structural name `diff` inferred.
            name: rollup.name.clone(),
            source_path: entry.source_path.clone(),
            outcome,
        });
        updates.push((entry.kind, rollup));
    }

    if !updates.is_empty() {
        rewrite_lock_rows(workspace_dir, &updates)?;
    }

    Ok(ReAddReport { entries })
}

/// Fold the reconciled/added `updates` into `<workspace_dir>/lock.toml`,
/// format-preserving via `toml_edit`: an existing row (matched by `source_path`
/// within its kind's `[[<kind>]]` array) has its `import_hash` and `last_applied`
/// refreshed in place; an added source with no row yet is appended as a fresh
/// table carrying all four columns.
fn rewrite_lock_rows(
    workspace_dir: &Path,
    updates: &[(&'static str, import::RollupEntry)],
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

    for (kind, row) in updates {
        let kind = *kind;
        // An added kind may have no array yet (an empty `ArrayOfTables` emits no
        // bytes, so a skill-only lock carries no `[[rule]]` key) — create it.
        if doc.get(kind).and_then(Item::as_array_of_tables).is_none() {
            doc[kind] = Item::ArrayOfTables(ArrayOfTables::new());
        }
        let Some(rows) = doc.get_mut(kind).and_then(Item::as_array_of_tables_mut) else {
            // Only reachable if the lock holds a non-array under this key; leaving
            // the row unwritten is safer than clobbering an unexpected shape.
            continue;
        };

        // Match by source_path in an immutable pass first, so the mutable borrow
        // for the in-place patch does not linger into the append arm.
        let existing = rows.iter().position(|table| {
            table.get("source_path").and_then(Item::as_str) == Some(&row.source_path)
        });
        match existing.and_then(|index| rows.get_mut(index)) {
            Some(table) => {
                table["import_hash"] = value(row.import_hash.clone());
                table["last_applied"] = value(row.last_applied.clone());
            }
            None => rows.push(lock_table(row)),
        }
    }

    fs::write(&path, doc.to_string()).map_err(|source| DriftError::Write { path, source })
}

/// Build a fresh `lock.toml` table for an added artifact — the four shared columns
/// in the same fixed order `import` writes them, so an appended row reads
/// identically to an imported one.
fn lock_table(row: &import::RollupEntry) -> Table {
    let mut table = Table::new();
    table["name"] = value(row.name.clone());
    table["source_path"] = value(row.source_path.clone());
    table["import_hash"] = value(row.import_hash.clone());
    table["last_applied"] = value(row.last_applied.clone());
    table
}

/// Render a re-add report for the terminal: one `<outcome>  <kind>  <name>` line
/// per entry in the report's stable order, then a one-line tally.
#[must_use]
pub fn render_readd(report: &ReAddReport) -> String {
    let mut out = String::new();
    let (mut reconciled, mut added, mut unchanged) = (0u32, 0u32, 0u32);
    for entry in &report.entries {
        match entry.outcome {
            ReAddOutcome::Reconciled => reconciled += 1,
            ReAddOutcome::Added => added += 1,
            ReAddOutcome::Unchanged => unchanged += 1,
        }
        out.push_str(&format!(
            "{:<10}  {:<5}  {}\n",
            entry.outcome.label(),
            entry.kind,
            entry.name
        ));
    }
    out.push_str(&format!(
        "\n{reconciled} reconciled, {added} added, {unchanged} unchanged\n"
    ));
    out
}

// ---------------------------------------------------------------------------
// place — the whole-file direction (`specs/50-distribution.md`, `install`)
// ---------------------------------------------------------------------------

/// Project `desired` onto `path` under the three-state merge — the whole-file
/// sibling of [`apply`]'s per-field patch, for artifacts temper *places* rather
/// than round-trips (`specs/50-distribution.md`, the `install` gate wiring). It
/// reuses the engine's own [`ApplyOutcome`] and [`DriftError`] so `install` builds
/// on this write-back direction rather than re-emitting one.
///
/// The three states are the engine's own: **desired** (the caller's bytes),
/// **last-applied** (the fingerprint of the file as temper last wrote it, from
/// `last_applied`), and **real on-disk**. The merge:
///
/// - target **absent** ⇒ [`ApplyOutcome::Applied`] — the placement is *created*
///   (an `install` onto a harness that does not carry it yet, or a re-add of one a
///   human deleted). This is the one divergence from [`apply`], where an absent
///   source is a world-deletion conflict: a placement has no prior on-disk source
///   to have been deleted, so writing it is the whole point.
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
            // Absent: create it (fresh install / re-add). There is nothing on disk
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
/// (`specs/50-distribution.md`, "Fail-loud delivery").
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

        let report = diff(&ws, &harness).unwrap();

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

        let report = diff(&ws, &harness).unwrap();

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

        let report = diff(&ws, &harness).unwrap();

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

        let report = diff(&ws, &harness).unwrap();

        assert_eq!(entry(&report, "coordinate").state, DriftState::Removed);
        assert_eq!(entry(&report, "rust").state, DriftState::InSync);
    }

    #[test]
    fn render_lists_each_state_label() {
        let report = DriftReport {
            entries: vec![
                DriftEntry {
                    kind: "skill",
                    name: "coordinate".into(),
                    source_path: PathBuf::from("skills/coordinate/SKILL.md"),
                    state: DriftState::Drifted,
                },
                DriftEntry {
                    kind: "rule",
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
