//! `temper install` — projecting the gate's wiring under the three-state drift
//! engine (`specs/50-distribution.md`, "Decision: `install` projects the gate's
//! wiring; drift keeps it synced").
//!
//! Drives the library `install::run` / `install::gate_installed` over a real
//! harness and proves the four properties the entry names:
//!
//! - **projection** — one run writes the `SessionStart` hook into
//!   `.claude/settings.json` (merged, preserving what was there), the CI job into
//!   `.github/workflows/`, and the schema modeline into each artifact's frontmatter;
//! - **idempotence** — a second run lands every placement `Unchanged` and touches
//!   not a byte;
//! - **dry-run** — `--dry-run` reports every outcome but writes nothing;
//! - **self-verify** — a hand-drifted placement is flagged by `gate_installed`.
//!
//! A final CLI case drives the real `temper install` binary across the process
//! boundary, since `main`'s dispatch (and the default `.` root) is observable only
//! there.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::ApplyOutcome;
use temper::install::{self, InstallReport};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-install-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill with frontmatter — the modeline placement inserts a modeline into it.
const SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A rule with `paths:` frontmatter — carries a modeline too.
const RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// A rule with no frontmatter — the modeline placement skips it (nothing to
/// validate), so a human's frontmatter-free file is never rewritten.
const COLLAB_RULE: &str = "# Collaboration\n\nPushback is the point.\n";

/// A pre-existing `.claude/settings.json` carrying an unrelated hook, so the merge
/// can be proven additive — the human's content survives the `SessionStart` graft.
const EXISTING_SETTINGS: &str =
    "{\n  \"permissions\": {\n    \"allow\": [\"Bash(cargo test:*)\"]\n  }\n}\n";

/// Build a harness with a skill, two rules (one frontmatter-free), and a
/// pre-existing settings file, and return its root.
fn write_harness(label: &str, with_settings: bool) -> PathBuf {
    let root = tmpdir(label);
    let skill = root.join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();

    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();
    fs::write(rules.join("collaboration.md"), COLLAB_RULE).unwrap();

    if with_settings {
        fs::write(
            root.join(".claude").join("settings.json"),
            EXISTING_SETTINGS,
        )
        .unwrap();
    }
    root
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

/// The outcome `install` reported for the placement at the file named `file`
/// (a path suffix match), asserting it is unique.
fn outcome_for(report: &InstallReport, file: &str) -> ApplyOutcome {
    let mut matches = report
        .entries
        .iter()
        .filter(|e| e.path.to_string_lossy().ends_with(file));
    let found = matches
        .next()
        .unwrap_or_else(|| panic!("no entry for {file}"));
    assert!(
        matches.next().is_none(),
        "entry for {file} should be unique"
    );
    found.outcome
}

#[test]
fn install_projects_the_three_placements() {
    let root = write_harness("projects", true);

    let report = install::run(&root, false).unwrap();

    // 1. The SessionStart hook is merged into settings.json — additively, so the
    //    pre-existing permissions block survives beside the grafted hook.
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "temper session-start ."
    );
    assert_eq!(
        json["permissions"]["allow"][0], "Bash(cargo test:*)",
        "the merge must preserve the human's existing settings"
    );
    assert_eq!(outcome_for(&report, "settings.json"), ApplyOutcome::Applied);

    // 2. The CI job lands as a whole file under .github/workflows/.
    let ci = root.join(".github").join("workflows").join("temper.yml");
    assert!(ci.is_file(), "the CI job must be written");
    assert!(
        fs::read_to_string(&ci).unwrap().contains("temper check"),
        "the CI job must run the gate"
    );
    assert_eq!(outcome_for(&report, "temper.yml"), ApplyOutcome::Applied);

    // 3. The schema modeline is inserted as the first frontmatter line of each
    //    artifact that HAS frontmatter — the skill and the `paths:` rule.
    let skill_md =
        fs::read_to_string(root.join("skills").join("coordinate").join("SKILL.md")).unwrap();
    assert!(
        skill_md
            .starts_with("---\n# yaml-language-server: $schema=../../.temper/schema/skill.json\n"),
        "the skill modeline must lead the frontmatter, got:\n{skill_md}"
    );
    // The body and the other frontmatter fields are preserved byte-for-byte.
    assert!(skill_md.contains("name: coordinate\n"));
    assert!(skill_md.ends_with("Drive the team through the playbook.\n"));

    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(
        rust_md.contains("# yaml-language-server: $schema=../../.temper/schema/rule.json"),
        "the rule modeline must reference the rule schema, got:\n{rust_md}"
    );

    // A frontmatter-free rule is left untouched — nothing to validate, so no header
    // is synthesised and no placement entry is emitted for it.
    let collab =
        fs::read_to_string(root.join(".claude").join("rules").join("collaboration.md")).unwrap();
    assert_eq!(
        collab, COLLAB_RULE,
        "a frontmatter-free rule must be untouched"
    );
    assert!(
        report
            .entries
            .iter()
            .all(|e| !e.path.to_string_lossy().ends_with("collaboration.md")),
        "no modeline placement is emitted for a frontmatter-free artifact"
    );
}

#[test]
fn a_second_run_is_unchanged_and_writes_nothing() {
    let root = write_harness("idempotent", true);

    install::run(&root, false).unwrap();
    let after_first = tree_bytes(&root);

    // The second run finds every placement already in its desired state.
    let report = install::run(&root, false).unwrap();
    assert!(
        report
            .entries
            .iter()
            .all(|e| e.outcome == ApplyOutcome::Unchanged),
        "a re-install must land every placement Unchanged, got: {:?}",
        report.entries
    );
    assert_eq!(
        after_first,
        tree_bytes(&root),
        "the idempotent re-run must not change a single byte"
    );
}

#[test]
fn dry_run_reports_outcomes_but_writes_nothing() {
    let root = write_harness("dry", true);
    let before = tree_bytes(&root);

    let report = install::run(&root, true).unwrap();
    // The report still says every placement *would* be written...
    assert!(
        report
            .entries
            .iter()
            .all(|e| e.outcome == ApplyOutcome::Applied),
        "a first dry run reports Applied for every placement, got: {:?}",
        report.entries
    );
    // ...but nothing landed: no .github, no modeline, no hook in settings.
    assert_eq!(before, tree_bytes(&root), "--dry-run must write nothing");
    assert!(!root.join(".github").exists());
}

#[test]
fn gate_installed_flags_a_missing_then_drifted_placement() {
    let root = write_harness("self-verify", true);

    // Before install: every placement is missing, so the self-verify warns for each
    // (the hook, the CI job, and both frontmatter artifacts' modelines).
    let before = install::gate_installed(&root);
    assert!(
        before
            .iter()
            .all(|d| d.severity == temper::check::Severity::Warn),
        "the self-verify is advisory — never error"
    );
    assert!(
        before.iter().any(|d| d.artifact.ends_with("settings.json")),
        "a missing hook is flagged"
    );
    assert!(
        before.iter().any(|d| d.artifact.ends_with("temper.yml")),
        "a missing CI job is flagged"
    );

    // After a full install the gate is clean — nothing to report.
    install::run(&root, false).unwrap();
    assert!(
        install::gate_installed(&root).is_empty(),
        "a fully installed gate reports no drift, got: {:?}",
        install::gate_installed(&root)
    );

    // Hand-drift one placement: a human edits the CI job out from under temper.
    let ci = root.join(".github").join("workflows").join("temper.yml");
    fs::write(&ci, "name: not-temper\n").unwrap();

    let after = install::gate_installed(&root);
    assert_eq!(after.len(), 1, "exactly the drifted placement is flagged");
    assert!(after[0].artifact.ends_with("temper.yml"));
    assert!(
        after[0].message.contains("temper install"),
        "the diagnostic points at the fix, got: {}",
        after[0].message
    );
}

#[test]
fn a_fresh_install_settings_document_is_stable() {
    // A fresh install (no pre-existing settings) yields a deterministic single-hook
    // document — a reviewable golden for the merge's output shape.
    let root = write_harness("golden", false);
    install::run(&root, false).unwrap();
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    insta::assert_snapshot!(settings);
}

#[test]
fn the_cli_install_verb_projects_and_dry_runs() {
    let root = write_harness("cli", true);

    // A dry run over the real binary reports the pending placements but writes none.
    let before = tree_bytes(&root);
    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--dry-run")
        .output()
        .unwrap();
    assert!(output.status.success(), "install --dry-run must exit zero");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("dry run") && stdout.contains("applied"),
        "the dry run must report the pending install, got:\n{stdout}"
    );
    assert_eq!(before, tree_bytes(&root), "the CLI dry run writes nothing");

    // The real run lands the placements and exits zero.
    let status = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .status()
        .unwrap();
    assert!(status.success(), "install must exit zero");
    assert!(
        root.join(".github")
            .join("workflows")
            .join("temper.yml")
            .is_file(),
        "the CLI install must write the CI job"
    );
}
