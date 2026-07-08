//! Acceptance for the memory-kind gate: the
//! `check` gate validates every embedded kind's members against
//! its default contract, so a discovered `CLAUDE.md` memory member fires its
//! `memory.anthropic` clauses instead of being silently skipped by a hardcoded skill/rule
//! pair.
//!
//! Driven across the real process boundary through the one-shot `check --harness` verb
//! (the route session-start takes: import into a scratch surface, gate, tear it down),
//! over harness-dir fixtures mirroring the real Claude Code layout — `.claude/skills/*`
//! plus a repo-root `CLAUDE.md`. The GitHub reporter is used for a machine-parseable
//! finding set: each finding is one `::error`/`::warning title=<rule>::<artifact>: …`
//! line, so a `max_lines` advisory on the `CLAUDE` member is counted exactly.

use std::fs;
use std::path::Path;
use std::process::Command;

mod common;

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

/// A skill that trips no `error`-severity clause: lowercase `name` matching its
/// directory, a present description, a short body — so the memory finding is not masked
/// by an unrelated skill failure, and the "clean skill still passes" arm is honest.
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill that violates a `required` clause: the uppercase `name` is outside the
/// `[a-z0-9-]` `allowed_chars` set — the existing skill finding the no-regression arm
/// asserts still fires beside the memory advisory.
const ERROR_SKILL: &str = "---\n\
name: Coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// Write a repo-root `CLAUDE.md` of `lines` total lines — the `memory` member `import`
/// discovers off its `governs` locus (`root = "."`, `glob = "CLAUDE.md"`).
fn write_claude_md(root: &Path, lines: usize) {
    let mut body = String::from("# Memory\n");
    // Already one line; pad to the requested total so `max_lines` (a body-line budget)
    // sees exactly `lines`.
    for i in 1..lines {
        body.push_str(&format!("Guidance line {i}.\n"));
    }
    fs::write(root.join("CLAUDE.md"), body).unwrap();
}

/// Run `temper check --harness <dir> --reporter github` and return the emitted finding
/// lines (`::error`/`::warning …`), one per finding.
fn check_harness(harness: &Path) -> Vec<String> {
    let run = common::check_in(
        harness,
        &["--harness", harness.to_str().unwrap()],
        Some("github"),
    );
    run.output
        .lines()
        .filter(|line| line.starts_with("::"))
        .map(str::to_string)
        .collect()
}

/// Write a repo-root `CLAUDE.md` whose body carries the given `@`-import directive on
/// its own line — the `memory` member `import` discovers off its `governs` locus, its
/// `at-import` target the directive classing resolves against provenance.
fn write_claude_md_importing(root: &Path, import_line: &str) {
    let body = format!("# Memory\n\nProject guidance.\n\n{import_line}\n");
    fs::write(root.join("CLAUDE.md"), body).unwrap();
}

/// Write a plain sibling file at `<root>/<rel>` — not a harness member, just a real repo
/// file an `@import` can resolve to. The backing set is the whole repo, so a resolving
/// target need not itself be an imported artifact.
fn write_sibling(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

/// Gate the harness's live surface with cwd = harness root — the TWO-STEP path (a bare
/// `.temper` workspace argument, no `--harness`): `check` reads built-in kind members
/// live off harness disk, so
/// no scratch import is needed to populate `.temper` first — the exact route the
/// `--harness` cases never take. Returns the emitted finding lines.
fn check_two_step(harness: &Path) -> Vec<String> {
    let output = Command::new(BIN)
        .current_dir(harness)
        .arg("check")
        .arg(".temper")
        .arg("--reporter")
        .arg("github")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .lines()
        .filter(|line| line.starts_with("::"))
        .map(str::to_string)
        .collect()
}

#[test]
fn the_two_step_check_path_backs_a_real_repo_root_import() {
    let harness = common::tmpdir("two-step-backed");
    // The TWO-STEP path (import to `.temper/`, then gate the bare workspace relative
    // to the harness root): a CLAUDE.md `@import`ing a real repo-root sibling. Before a
    // past fix, the harness root resolved to the EMPTY path, so `repo_file_set` walked
    // nothing and EVERY real import read unbacked. The backing set is the whole repo,
    // so the resolving edge must fire no finding.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_sibling(&harness, "docs/ledger.md", "# Ledger\n\nShared state.\n");
    write_claude_md_importing(&harness, "@docs/ledger.md");

    assert!(
        common::findings_for(&check_two_step(&harness), "graph.directive-unbacked").is_empty(),
        "the two-step path must enumerate the whole repo and back a real sibling, got: {:#?}",
        check_two_step(&harness)
    );

    // The backing check stays live, not disabled: delete the sibling and the advisory
    // returns (the surface is unchanged — the target is a plain repo file, not a member —
    // so no re-import is needed; the backing set is re-walked from disk each gate).
    fs::remove_file(harness.join("docs/ledger.md")).unwrap();
    let findings = check_two_step(&harness);
    let unbacked = common::findings_for(&findings, "graph.directive-unbacked");
    assert_eq!(
        unbacked.len(),
        1,
        "removing the target brings the unbacked advisory back, got: {findings:#?}"
    );
    assert!(
        unbacked[0].contains("docs/ledger.md"),
        "the returned finding names the now-missing target, got: {}",
        unbacked[0]
    );
}

#[test]
fn an_unbacked_at_import_in_a_claude_md_fires_one_unbacked_pointer_finding() {
    let harness = common::tmpdir("unbacked-import");
    // A clean skill so the run is not empty, and a CLAUDE.md importing a path that backs
    // no member and no repo file — an unbacked pointer. Before the collection generalized
    // over every kind (DIRECTIVE-MEMBERS-ALL-KINDS), the hardcoded skill/rule pair never
    // reached the memory member's directives, so this drew no finding (exit 0).
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@docs/missing.md");

    let findings = check_harness(&harness);
    let unbacked = common::findings_for(&findings, "graph.directive-unbacked");

    // Exactly one unbacked-pointer finding, on the memory member — the wedge path now
    // collects the CLAUDE.md's `at-import` targets and classes them.
    assert_eq!(
        unbacked.len(),
        1,
        "expected exactly one unbacked-pointer finding on the memory member, got: {findings:#?}"
    );
    let finding = unbacked[0];
    assert!(
        finding.contains("::CLAUDE:"),
        "the finding names the CLAUDE memory member, got: {finding}"
    );
    assert!(
        finding.contains("docs/missing.md"),
        "the finding names the unbacked target, got: {finding}"
    );
}

#[test]
fn a_claude_md_import_resolving_to_a_member_fires_no_unbacked_finding() {
    let harness = common::tmpdir("backed-import");
    // The CLAUDE.md imports the coordinate skill member by its provenance locus — a
    // resolving member→member edge, not an unbacked pointer. The wedge collects the
    // directive and classes it as backed, so nothing fires.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@.claude/skills/coordinate/SKILL.md");

    let findings = check_harness(&harness);

    assert!(
        common::findings_for(&findings, "graph.directive-unbacked").is_empty(),
        "a directive resolving to a real member must fire no unbacked-pointer finding, got: {findings:#?}"
    );
}

#[test]
fn an_unbacked_at_import_fires_a_non_gating_advisory_with_zero_config() {
    let harness = common::tmpdir("floor-unbacked-import");
    // The FLOOR-tier wedge (WEDGE-FACT-FLOOR): a discovered CLAUDE.md carrying an unbacked
    // `@import`. Directive classing runs on the floor, so the unbacked pointer surfaces
    // with zero config authored anywhere. It is a non-gating advisory: the pure fact is
    // stated, never escalated.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@docs/missing.md");

    let findings = check_harness(&harness);
    let unbacked = common::findings_for(&findings, "graph.directive-unbacked");

    // Exactly one unbacked-pointer finding, on the memory member, drawn with zero config.
    assert_eq!(
        unbacked.len(),
        1,
        "the floor tier surfaces the unbacked `@import` with zero config, got: {findings:#?}"
    );
    let finding = unbacked[0];
    assert!(
        finding.starts_with("::warning "),
        "the unbacked pointer is a non-gating advisory (warn), got: {finding}"
    );
    assert!(
        finding.contains("::CLAUDE:"),
        "the finding names the CLAUDE memory member, got: {finding}"
    );
    assert!(
        finding.contains("docs/missing.md"),
        "the finding names the unbacked target, got: {finding}"
    );
}

#[test]
fn a_backed_at_import_fires_nothing_with_zero_config() {
    let harness = common::tmpdir("floor-backed-import");
    // The floor tier states only the fact: a CLAUDE.md whose `@path` resolves to a real repo
    // file (the coordinate skill's on-disk member) is a backed boundary edge, not an unbacked
    // pointer — so it draws no finding even with zero config. Pairs the fired case above.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@.claude/skills/coordinate/SKILL.md");

    let findings = check_harness(&harness);

    assert!(
        common::findings_for(&findings, "graph.directive-unbacked").is_empty(),
        "a backed `@import` fires no unbacked-pointer finding on the floor, got: {findings:#?}"
    );
}

#[test]
fn an_over_length_claude_md_fires_exactly_one_memory_max_lines_advisory() {
    let harness = common::tmpdir("over-length");
    // A clean skill so the run is not empty, and a 251-line CLAUDE.md over the
    // memory.anthropic 200-line budget.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md(&harness, 251);

    let findings = check_harness(&harness);
    let max_lines = common::findings_for(&findings, "max_lines");

    // Exactly one `max_lines` advisory — the memory member dispatched to
    // memory.anthropic, not silently skipped.
    assert_eq!(
        max_lines.len(),
        1,
        "expected exactly one max_lines advisory on the memory member, got: {findings:#?}"
    );
    // It is a `warning` (advisory) naming the `CLAUDE` member and the 251/200 budget.
    let finding = max_lines[0];
    assert!(
        finding.starts_with("::warning "),
        "max_lines is an advisory (warn), got: {finding}"
    );
    assert!(
        finding.contains("::CLAUDE:"),
        "the advisory names the CLAUDE memory member, got: {finding}"
    );
    assert!(
        finding.contains("251") && finding.contains("200"),
        "the advisory reports the 251-line body over the 200 budget, got: {finding}"
    );

    // No regression: the clean skill still trips no finding under its own kind.
    assert!(
        common::findings_for(&findings, "allowed_chars").is_empty(),
        "the clean skill must not trip allowed_chars, got: {findings:#?}"
    );
}

#[test]
fn an_under_length_claude_md_fires_no_memory_advisory() {
    let harness = common::tmpdir("under-length");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    // A short CLAUDE.md, well under the 200-line budget.
    write_claude_md(&harness, 10);

    let findings = check_harness(&harness);

    // The memory member is still dispatched to memory.anthropic — it simply conforms, so
    // the body-size budget fires nothing.
    assert!(
        common::findings_for(&findings, "max_lines").is_empty(),
        "an under-length CLAUDE.md must fire no max_lines advisory, got: {findings:#?}"
    );
}

#[test]
fn the_memory_dispatch_leaves_skill_findings_unchanged() {
    let harness = common::tmpdir("no-regression");
    // A failing skill (uppercase name) beside an over-length CLAUDE.md: both the skill's
    // existing finding and the new memory advisory must fire, unaffected by each other.
    common::write_skill(&harness, "coordinate", ERROR_SKILL);
    write_claude_md(&harness, 251);

    let findings = check_harness(&harness);

    // The skill finding still fires, exactly as before the gate generalized.
    let allowed_chars = common::findings_for(&findings, "allowed_chars");
    assert_eq!(
        allowed_chars.len(),
        1,
        "the uppercase-name skill must still trip allowed_chars, got: {findings:#?}"
    );
    assert!(
        allowed_chars[0].contains("::coordinate:"),
        "the skill finding names the coordinate skill, got: {}",
        allowed_chars[0]
    );

    // And the memory advisory fires beside it — the two kinds are judged in one run.
    assert_eq!(
        common::findings_for(&findings, "max_lines").len(),
        1,
        "the memory advisory fires beside the skill finding, got: {findings:#?}"
    );
}
