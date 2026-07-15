//! Fail-loud on a mis-rooted or malformed harness. `temper check <root>` resolves
//! the harness root's own `<root>/.temper` lock and walks members off `<root>`, so an
//! adopted assembly can no longer resolve to a lockless workspace and exit a silent
//! green — the half-gate the arg-resolution fix closed. What remains fail-loud here:
//! a required requirement with no filler still fails via the coverage tier, and a
//! malformed member aborts loud naming the file. This drives the real binary so the
//! resolution is exercised exactly as a session hits it, not just the pure predicate.

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
fn an_adopted_root_resolves_its_own_lock_rather_than_half_gating() {
    // The mis-rooting the arg fix closed: an adopted `<root>/.temper` lock declaring a
    // `required` requirement, with no member filling it. `check .` at the harness root
    // must resolve that lock and fail loud on the unfilled requirement — never read the
    // lock from `<root>` itself (finding none) and exit a silent green.
    let root = common::tmpdir("declared-empty");
    common::write_requirements(&root, vec![common::requirement("docs", true, None)]);

    let (findings, success) = check_in(&root, &["."]);

    let unfilled = common::findings_for(&findings, "requirement.unfilled");
    assert_eq!(
        unfilled.len(),
        1,
        "the adopted lock's unfilled required requirement must fire, got: {findings:#?}"
    );
    assert!(
        unfilled[0].starts_with("::error "),
        "an unfilled required requirement is error-severity (fails the run), got: {}",
        unfilled[0]
    );
    assert!(
        !success,
        "a declared-but-unfilled required requirement must exit non-zero, got: {findings:#?}"
    );
}

#[test]
fn a_non_required_requirement_with_no_members_is_legitimately_clean() {
    // Not every zero-member harness is a mis-rooting: an adopted lock whose only
    // requirement is *not* `required` resolves cleanly. The lock was read (declarations
    // are non-empty), so this is a fully-resolved workspace that happens to carry no
    // members — a legitimate green, not a half-gate.
    let root = common::tmpdir("declared-non-required");
    common::write_requirements(&root, vec![common::requirement("docs", false, None)]);

    let (findings, success) = check_in(&root, &["."]);

    assert!(
        common::findings_for(&findings, "requirement.unfilled").is_empty(),
        "a non-required requirement must not fire an unfilled error, got: {findings:#?}"
    );
    assert!(
        success,
        "an adopted lock with only a non-required requirement must exit zero, got: {findings:#?}"
    );
}

#[test]
fn a_correctly_rooted_check_that_resolves_members_stays_silent() {
    // The same requirement-declaring lock, but this time the harness carries a real
    // skill at its committed locus (`.claude/skills/coordinate/SKILL.md`) — `check`
    // reads built-in kind members live off harness disk, no scratch import required, and
    // the correctly-rooted path resolves ≥1 member, so a bare `check` from the root
    // stays clean even though the assembly still declares a (non-required) requirement.
    let root = common::tmpdir("declared-resolved");
    let harness = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&harness).unwrap();
    fs::write(harness.join("SKILL.md"), CLEAN_SKILL).unwrap();
    common::write_requirements(&root, vec![common::requirement("docs", false, None)]);

    let (findings, success) = check_in(&root, &[]);

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
    // members is legitimate and the check exits clean.
    let root = common::tmpdir("genuinely-empty");

    let (findings, success) = check_in(&root, &[]);

    assert!(
        success,
        "a genuinely empty harness's check must exit zero, got: {findings:#?}"
    );
}
