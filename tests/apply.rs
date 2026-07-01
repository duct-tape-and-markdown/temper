//! `temper apply` — the write direction over the three-state drift engine
//! (`specs/20-surface.md`, "Drift / apply — three states, never two").
//!
//! Drives the library `drift::apply` over a real imported surface and proves the
//! four properties the entry names:
//!
//! - **idempotence** — a clean surface→disk apply, re-run, changes nothing;
//! - **patch-not-re-emit fidelity** — a surface-only edit patches just the changed
//!   field and leaves the surrounding frontmatter bytes (comments, other fields)
//!   exactly as the human wrote them;
//! - **three-state conflict handling** — a world drift that differs from the
//!   last-applied fingerprint surfaces a conflict rather than clobbering the source;
//! - **dry-run writes nothing** — the outcome is reported but not a byte lands.
//!
//! A fifth case pins the fingerprint reconciliation the merge depends on: two
//! successive surface edits both apply cleanly (the second is *not* misread as a
//! world drift), which only holds if `apply` advances the last-applied fingerprint.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Workspace;
use temper::drift::{self, ApplyOptions, ApplyOutcome};
use temper::import;
use temper::skill::Skill;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-apply-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill whose frontmatter carries a human comment and a blank line — bytes a
/// surgical patch must preserve when an unrelated field changes. The body keeps a
/// missing final newline so the byte-faithful round trip is observable.
const SKILL: &str = "---\n\
name: coordinate\n\
# a human note that must survive a patch\n\
description: Use when coordinating agents across axes.\n\
version: \"0.3.0\"\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.";

/// A rule with `paths:` frontmatter and an unknown Cursor key preserved verbatim.
const RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// The on-disk source path of the imported skill in `harness`.
fn skill_source(harness: &Path) -> PathBuf {
    harness.join("skills").join("coordinate").join("SKILL.md")
}

/// Build a one-skill + one-rule harness and import it into a fresh surface,
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

/// Snapshot every file under `dir` as a sorted map of relative path -> bytes.
fn tree_bytes(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        for entry in fs::read_dir(&current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path.strip_prefix(dir).unwrap().to_path_buf();
                out.insert(rel, fs::read(&path).unwrap());
            }
        }
    }
    out
}

/// The outcome `apply` reported for `name` in `report`, asserting it is unique.
fn outcome(report: &drift::ApplyReport, name: &str) -> ApplyOutcome {
    let mut matches = report.entries.iter().filter(|e| e.name == name);
    let found = matches.next().expect("entry should exist");
    assert!(matches.next().is_none(), "entry {name} should be unique");
    found.outcome
}

/// Rewrite the surface skill's `description` to `new`, exactly as a human editing
/// the composition surface would, and return the reloaded workspace path.
fn edit_surface_description(workspace: &Path, new: &str) {
    let dir = workspace.join("skills").join("coordinate");
    let mut skill = Skill::from_dir(&dir).unwrap();
    skill.description = new.to_string();
    fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
}

#[test]
fn apply_is_idempotent_over_a_clean_surface() {
    let (harness, into) = imported("idem");
    let before = tree_bytes(&harness);

    // A freshly imported surface already sits on disk: every artifact is unchanged.
    let ws = Workspace::load(&into).unwrap();
    let report = drift::apply(&ws, &into, ApplyOptions::default()).unwrap();
    assert_eq!(outcome(&report, "coordinate"), ApplyOutcome::Unchanged);
    assert_eq!(outcome(&report, "rust"), ApplyOutcome::Unchanged);
    assert_eq!(before, tree_bytes(&harness), "a clean apply writes nothing");

    // Re-running changes nothing either — idempotent.
    let ws = Workspace::load(&into).unwrap();
    let report = drift::apply(&ws, &into, ApplyOptions::default()).unwrap();
    assert!(
        report
            .entries
            .iter()
            .all(|e| e.outcome == ApplyOutcome::Unchanged)
    );
    assert_eq!(before, tree_bytes(&harness), "the re-run writes nothing");
}

#[test]
fn a_surface_edit_patches_only_the_changed_field() {
    let (harness, into) = imported("patch");

    edit_surface_description(&into, "Use when driving a team across many axes.");

    let ws = Workspace::load(&into).unwrap();
    let report = drift::apply(&ws, &into, ApplyOptions::default()).unwrap();
    assert_eq!(outcome(&report, "coordinate"), ApplyOutcome::Applied);
    // The untouched rule stays put.
    assert_eq!(outcome(&report, "rust"), ApplyOutcome::Unchanged);

    let patched = fs::read_to_string(skill_source(&harness)).unwrap();
    // The changed field carries the new value...
    assert!(
        patched.contains("Use when driving a team across many axes."),
        "the edited description must land on disk, got:\n{patched}"
    );
    assert!(
        !patched.contains("Use when coordinating agents across axes."),
        "the old description must be gone, got:\n{patched}"
    );
    // ...while every surrounding byte is exactly as the human wrote it: the comment,
    // the untouched `name` and `version` lines, and the byte-faithful body.
    assert!(
        patched.contains("# a human note that must survive a patch"),
        "the frontmatter comment must survive, got:\n{patched}"
    );
    assert!(patched.contains("name: coordinate\n"), "name untouched");
    assert!(
        patched.contains("version: \"0.3.0\"\n"),
        "version untouched"
    );
    assert!(
        patched.ends_with("Drive the team through the playbook."),
        "the body stays byte-faithful (no trailing newline added), got:\n{patched}"
    );

    // The patch re-parses to the same typed skill the surface holds — a valid write.
    let reloaded = Skill::from_source_dir(harness.join("skills").join("coordinate").as_path())
        .expect("the patched source must re-parse");
    assert_eq!(
        reloaded.description,
        "Use when driving a team across many axes."
    );
    assert_eq!(reloaded.version.as_deref(), Some("0.3.0"));
}

#[test]
fn a_world_drift_surfaces_a_conflict_instead_of_clobbering() {
    let (harness, into) = imported("conflict");

    // The human edits the source directly, on disk — a world drift the surface
    // knows nothing about. The surface itself is left as imported.
    let source = skill_source(&harness);
    let drifted = fs::read_to_string(&source).unwrap() + "\nA line added straight to disk.\n";
    fs::write(&source, &drifted).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::apply(&ws, &into, ApplyOptions::default()).unwrap();

    // The drifted source differs from the last-applied fingerprint, so `apply`
    // surfaces the choice rather than overwriting the human's on-disk edit.
    assert_eq!(outcome(&report, "coordinate"), ApplyOutcome::Conflicted);
    assert_eq!(
        fs::read_to_string(&source).unwrap(),
        drifted,
        "a conflict must not clobber the drifted source"
    );
    // The untouched rule still applies cleanly alongside the conflict.
    assert_eq!(outcome(&report, "rust"), ApplyOutcome::Unchanged);
}

#[test]
fn dry_run_reports_the_outcome_but_writes_nothing() {
    let (harness, into) = imported("dry");

    edit_surface_description(&into, "A description the dry run must not write.");
    let before_harness = tree_bytes(&harness);
    let before_lock = fs::read(into.join("lock.toml")).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::apply(&ws, &into, ApplyOptions { dry_run: true }).unwrap();
    // The report still says what *would* happen...
    assert_eq!(outcome(&report, "coordinate"), ApplyOutcome::Applied);
    // ...but not a byte of the harness or the lock changed.
    assert_eq!(
        before_harness,
        tree_bytes(&harness),
        "--dry-run must not touch the harness sources"
    );
    assert_eq!(
        before_lock,
        fs::read(into.join("lock.toml")).unwrap(),
        "--dry-run must not touch the lock fingerprints"
    );

    // A real apply afterwards does land the edit.
    let ws = Workspace::load(&into).unwrap();
    drift::apply(&ws, &into, ApplyOptions::default()).unwrap();
    assert!(
        fs::read_to_string(skill_source(&harness))
            .unwrap()
            .contains("A description the dry run must not write."),
        "the real apply must write what the dry run only reported"
    );
}

#[test]
fn successive_surface_edits_each_apply_cleanly() {
    // The fingerprint reconciliation the three-state merge depends on: after the
    // first apply advances the last-applied fingerprint, a *second* surface edit is
    // read as a clean surface edit — not misdiagnosed as a world drift.
    let (harness, into) = imported("successive");

    edit_surface_description(&into, "First edit.");
    let ws = Workspace::load(&into).unwrap();
    assert_eq!(
        outcome(
            &drift::apply(&ws, &into, ApplyOptions::default()).unwrap(),
            "coordinate"
        ),
        ApplyOutcome::Applied
    );

    edit_surface_description(&into, "Second edit, atop the first.");
    let ws = Workspace::load(&into).unwrap();
    assert_eq!(
        outcome(
            &drift::apply(&ws, &into, ApplyOptions::default()).unwrap(),
            "coordinate"
        ),
        ApplyOutcome::Applied,
        "a second surface edit must apply, not conflict — the fingerprint advanced"
    );

    assert!(
        fs::read_to_string(skill_source(&harness))
            .unwrap()
            .contains("Second edit, atop the first."),
        "the second edit must be the one on disk"
    );
}
