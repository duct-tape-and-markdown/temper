//! The fail-loud coherence guard: a placement that cannot run the engine must error, never
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
//! (b) a correctly-rooted check that resolves ≥1 member never fires, even though the same lock declares a requirement;
//! (c) a genuinely empty harness (no declared requirements) never fires — zero members
//!     is legitimate there.

use std::fs;
use std::path::Path;

mod common;

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

/// Run `temper check <args...>` from `root`, returning `(github-format finding lines,
/// exit success)` — the machine format used elsewhere in this suite
/// (`tests/coverage_note.rs`) so a rule id is asserted exactly rather than scraped out of
/// miette's graphical rendering.
fn check_in(root: &Path, args: &[&str]) -> (Vec<String>, bool) {
    let run = common::check_in(root, args, Some("github"));
    let findings = run
        .output
        .lines()
        .filter(|line| line.starts_with("::"))
        .map(str::to_string)
        .collect();
    (findings, run.ok)
}

#[test]
fn declared_but_nothing_resolved_fails_loud_with_the_coherence_error() {
    // The harness-root `temper check .` case the wave-end confirmation caught: a
    // committed lock declares a requirement, but nothing was ever imported — no
    // surface tree at the workspace `check` reads.
    let root = common::tmpdir("declared-empty");
    common::write_requirements(&root, vec![common::requirement("docs", false, None)]);

    let (findings, success) = check_in(&root, &["."]);

    let fired = common::findings_for(&findings, "coverage.empty-assembly");
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
    // reads built-in kind members live off harness disk, no scratch import required, and the correctly-rooted path
    // resolves ≥1 member, so the guard must not fire even though the assembly still
    // declares a requirement.
    let root = common::tmpdir("declared-resolved");
    let harness = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&harness).unwrap();
    fs::write(harness.join("SKILL.md"), CLEAN_SKILL).unwrap();
    common::write_requirements(&root, vec![common::requirement("docs", false, None)]);

    let (findings, success) = check_in(&root, &[]);

    assert!(
        common::findings_for(&findings, "coverage.empty-assembly").is_empty(),
        "a resolving workspace must not trip the empty-assembly guard, got: {findings:#?}"
    );
    assert!(
        success,
        "the correctly-rooted, resolving check must exit zero, got: {findings:#?}"
    );
}

#[test]
fn a_malformed_frontmatter_block_fails_loud_naming_the_file() {
    // A skill whose SKILL.md carries a present-but-non-mapping frontmatter block. The
    // parse used to degrade to an empty field map, so the floor judged fabricated
    // absence (a missing `name`/`description`). Invariant 6 wants the malformation
    // surfaced loud — an error naming the file, never a missing-field finding over
    // silently-emptied fields.
    let root = common::tmpdir("malformed-frontmatter");
    let malformed = "---\n\
        this is a bare scalar, not a mapping\n\
        ---\n\
        # Broken\n\
        \n\
        Body.\n";
    common::write_skill(&root, "broken", malformed);

    let run = common::check_in(&root, &["."], Some("github"));

    assert!(
        !run.ok,
        "a malformed frontmatter block must fail check, got success:\n{}",
        run.output
    );
    assert!(
        run.output.contains("SKILL.md"),
        "the error must name the offending file, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("mapping"),
        "the error must name the malformation, got:\n{}",
        run.output
    );
    // The block aborts loud; no field-level finding is emitted over the emptied fields.
    let findings: Vec<&str> = run
        .output
        .lines()
        .filter(|line| line.starts_with("::"))
        .collect();
    assert!(
        findings.is_empty(),
        "a malformed block aborts loud; it must not emit field findings, got:\n{findings:#?}"
    );
}

#[test]
fn a_genuinely_empty_harness_stays_silent() {
    // No declared requirements at all: the assembly declares nothing, so zero resolved
    // members is legitimate and the guard must never fire.
    let root = common::tmpdir("genuinely-empty");

    let (findings, success) = check_in(&root, &[]);

    assert!(
        common::findings_for(&findings, "coverage.empty-assembly").is_empty(),
        "a genuinely empty harness must not trip the empty-assembly guard, got: {findings:#?}"
    );
    assert!(
        success,
        "a genuinely empty harness's check must exit zero, got: {findings:#?}"
    );
}
