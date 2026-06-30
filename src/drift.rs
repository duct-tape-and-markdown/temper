//! `temper diff` — the read-only drift report.
//!
//! Implements the first, read-only slice of the three-state drift engine
//! (`specs/20-surface.md`, "Drift / apply — three states, never two"). The full
//! engine tracks three states — **desired** (the edited surface), the
//! **last-applied fingerprint**, and **real on-disk** — so `apply` can tell a
//! human surface edit from a world drift and merge rather than clobber. This
//! slice covers the **real-on-disk vs import-baseline** axis only: for each
//! artifact the surface imported, has its source on disk changed since import?
//! The surface-edit and post-apply-fingerprint axes arrive with `apply`/`re-add`.
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
//! `diff` returns a typed [`DriftReport`] and renders nothing; [`render`] turns
//! that report into terminal text, and `main` maps it to stdout. Drift is a
//! report, not a gate (`specs/20-surface.md`, the CLI surface) — the command
//! exits zero regardless of what it finds.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::check::Workspace;
use crate::import;

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
    /// The artifact kind — `"skill"`, `"rule"`, or `"spec"`.
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

    let specs = workspace
        .specs
        .iter()
        .map(|spec| SurfaceArtifact {
            name: spec.name.clone(),
            source_path: spec.provenance.source_path.clone(),
            import_hash: spec.provenance.import_hash.clone(),
        })
        .collect::<Vec<_>>();
    let specs_on_disk = import::discover_spec_files(harness)?;
    entries.extend(classify("spec", &specs, &specs_on_disk)?);

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
/// skill is named by its directory (the `SKILL.md`'s parent), a rule or spec by
/// its file stem. A scan, not a parse — the structural name, not the frontmatter
/// one (which only a full read would yield).
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

/// Lowercase hex SHA-256 of `bytes` — the same digest `import` anchors provenance
/// with, recomputed here over the live source to detect drift. Duplicated per the
/// one-helper-per-module convention (`.claude/rules/rust.md`).
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
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
        let skill = harness.join("skills").join("coordinate");
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
        let skill_md = harness.join("skills").join("coordinate").join("SKILL.md");
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
        fs::remove_dir_all(harness.join("skills").join("coordinate")).unwrap();

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
}
