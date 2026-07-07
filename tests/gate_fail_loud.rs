//! The fail-loud coherence guard (`specs/distribution.md`, "Fail-loud
//! delivery — the invariant"): a placement that cannot run the engine must error, never
//! silently skip. `temper check .` at a harness root reads no `./lock.toml`/`./skills`
//! (they live under `.temper/`), so a committed assembly that declares
//! members/requirements resolves nothing and used to exit 0 — the wave-end confirmation
//! caught exactly this ("checked 0 members … exit 0"). This drives the real binary so
//! the mis-rooting is reproduced exactly as the confirmation hit it, not just the pure
//! predicate.
//!
//! Cases mirror the entry's acceptance:
//! (a) declared-but-nothing-resolved ⇒ an `error` `coverage.empty-assembly` and a
//!     non-zero exit;
//! (b) a correctly-rooted check that resolves ≥1 member never fires (`specs/intent.md`:
//!     no false block on a clean gate), even though the same lock declares a requirement;
//! (c) a genuinely empty harness (no declared requirements) never fires — zero members
//!     is legitimate there.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::{self, Declarations, EmitOptions, Payload, RequirementRow};

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

/// A bare `RequirementRow` naming `name` — enough to make the committed assembly
/// `declared` (`specs/distribution.md`).
fn requirement(name: &str) -> RequirementRow {
    RequirementRow {
        name: name.to_string(),
        kind: None,
        required: false,
        clauses: Vec::new(),
        verified_by: None,
    }
}

/// Compile a golden lock at `<root>/.temper/lock.toml` carrying just the declared
/// `requirements` — the SDK-emitted fixture standing in for `import::run`'s scratch
/// projection: the gate sources requirements from the lock, never a re-imported
/// assembly (`specs/model/pipeline.md`, "The lock").
fn write_requirements(root: &Path, requirements: Vec<RequirementRow>) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            requirements,
            ..Declarations::default()
        },
        members: Vec::new(),
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
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
    // committed lock declares a requirement, but nothing was ever imported — no
    // surface tree at the workspace `check` reads.
    let root = tmpdir("declared-empty");
    write_requirements(&root, vec![requirement("docs")]);

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
    // The same requirement-declaring lock, but this time the harness carries a real
    // skill at its committed locus (`.claude/skills/coordinate/SKILL.md`) — `check`
    // reads built-in kind members live off harness disk (`specs/model/pipeline.md`,
    // "The lock"), no scratch import required, and the correctly-rooted path
    // resolves ≥1 member, so the guard must not fire even though the assembly still
    // declares a requirement.
    let root = tmpdir("declared-resolved");
    let harness = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&harness).unwrap();
    fs::write(harness.join("SKILL.md"), CLEAN_SKILL).unwrap();
    write_requirements(&root, vec![requirement("docs")]);

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
    // No declared requirements at all: the assembly declares nothing, so zero resolved
    // members is legitimate and the guard must never fire.
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
