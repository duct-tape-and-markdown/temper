//! The fail-loud coherence guard (`specs/architecture/50-distribution.md`, "Fail-loud
//! delivery — the invariant"): a placement that cannot run the engine must error, never
//! silently skip. `temper check .` at a harness root reads no `./lock.toml`/`./skills`
//! (they live under `.temper/`), so a committed `temper.toml` that declares
//! members/requirements resolves nothing and used to exit 0 — the wave-end confirmation
//! caught exactly this ("checked 0 members … exit 0"). This drives the real binary so
//! the mis-rooting is reproduced exactly as the confirmation hit it, not just the pure
//! predicate.
//!
//! Cases mirror the entry's acceptance:
//! (a) declared-but-nothing-resolved ⇒ an `error` `coverage.empty-assembly` and a
//!     non-zero exit;
//! (b) a correctly-rooted check that resolves ≥1 member never fires (law 3: no false
//!     block on a clean gate), even though the same `temper.toml` declares a requirement;
//! (c) a genuinely empty harness (no `temper.toml`) never fires — zero members is
//!     legitimate there.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-gate-fail-loud-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill clean against the floor (lowercase `name` matching its directory, a present
/// short description) — the real Claude Code locus (`.claude/skills/<name>/SKILL.md`),
/// never a layout invented for the test.
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A `temper.toml` declaring one requirement — enough to make the committed assembly
/// `declared` (`specs/architecture/50-distribution.md`) without needing a `[[member]]` carriage
/// round-trip.
const DECLARES_A_REQUIREMENT: &str = "[requirement.docs]\n\
means = \"Every shipped skill is documented.\"\n";

/// Write `<root>/temper.toml`.
fn write_temper_toml(root: &Path, contents: &str) {
    fs::write(root.join("temper.toml"), contents).unwrap();
}

/// Run `temper check <args...>` from `root`, returning `(github-format finding lines,
/// exit success)` — the machine format used elsewhere in this suite
/// (`tests/coverage_note.rs`) so a rule id is asserted exactly rather than scraped out of
/// miette's graphical rendering.
fn check_in(root: &Path, args: &[&str]) -> (Vec<String>, bool) {
    let output = Command::new(BIN)
        .current_dir(root)
        .arg("check")
        .args(args)
        .arg("--reporter")
        .arg("github")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let findings = stdout
        .lines()
        .filter(|line| line.starts_with("::"))
        .map(str::to_string)
        .collect();
    (findings, output.status.success())
}

/// The findings whose rule (the `title=<rule>` property) equals `rule`.
fn findings_for<'a>(findings: &'a [String], rule: &str) -> Vec<&'a String> {
    let needle = format!("title={rule}::");
    findings
        .iter()
        .filter(|line| line.contains(&needle))
        .collect()
}

#[test]
fn declared_but_nothing_resolved_fails_loud_with_the_coherence_error() {
    // The harness-root `temper check .` case the wave-end confirmation caught: a
    // committed `temper.toml` declares a requirement, but nothing was ever imported —
    // no `.temper/lock.toml`, no surface tree at the workspace `check` reads.
    let root = tmpdir("declared-empty");
    write_temper_toml(&root, DECLARES_A_REQUIREMENT);

    let (findings, success) = check_in(&root, &["."]);

    let fired = findings_for(&findings, "coverage.empty-assembly");
    assert_eq!(
        fired.len(),
        1,
        "expected exactly one empty-assembly error, got: {findings:#?}"
    );
    let finding = fired[0];
    assert!(
        finding.starts_with("::error "),
        "the empty-assembly guard is error-severity (fails the run), got: {finding}"
    );
    assert!(
        !success,
        "a declared-but-unresolved assembly must exit non-zero, got: {findings:#?}"
    );
}

#[test]
fn a_correctly_rooted_check_that_resolves_members_stays_silent() {
    // The same requirement-declaring `temper.toml`, but this time the harness is
    // properly imported: a real skill lands under `.temper/skills/`, and `check` reads
    // the default `./.temper` workspace — the correctly-rooted path resolves ≥1 member,
    // so the guard must not fire even though the assembly still declares a requirement.
    let root = tmpdir("declared-resolved");
    let harness = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&harness).unwrap();
    fs::write(harness.join("SKILL.md"), CLEAN_SKILL).unwrap();
    write_temper_toml(&root, DECLARES_A_REQUIREMENT);
    temper::import::run(&root, &root.join(".temper")).unwrap();

    let (findings, success) = check_in(&root, &[]);

    assert!(
        findings_for(&findings, "coverage.empty-assembly").is_empty(),
        "a resolving workspace must not trip the empty-assembly guard, got: {findings:#?}"
    );
    assert!(
        success,
        "the correctly-rooted, resolving check must exit zero, got: {findings:#?}"
    );
}

#[test]
fn a_genuinely_empty_harness_stays_silent() {
    // No `temper.toml` at all: the assembly declares nothing, so zero resolved members
    // is legitimate and the guard must never fire.
    let root = tmpdir("genuinely-empty");

    let (findings, success) = check_in(&root, &[]);

    assert!(
        findings_for(&findings, "coverage.empty-assembly").is_empty(),
        "a genuinely empty harness must not trip the empty-assembly guard, got: {findings:#?}"
    );
    assert!(
        success,
        "a genuinely empty harness's check must exit zero, got: {findings:#?}"
    );
}
