//! Acceptance for the wedge's advisory coverage note (`specs/distribution.md`,
//! "Fail-loud delivery — the invariant"): the `check` gate states which built-in kinds
//! checked how many members and names the known Claude Code surfaces present on disk
//! that no kind governs, so the gate's silence about an unmodeled surface never reads
//! as "checked".
//!
//! Driven across the real process boundary through the one-shot `check --harness` verb
//! (the route session-start takes), over harness-dir fixtures mirroring the real Claude
//! Code layout — `.claude/skills/*` plus, for the gap arm, a `.claude/agents/` tree no
//! built-in kind governs. The GitHub reporter gives a machine-parseable finding set:
//! each finding is one `::warning title=<rule>::<artifact>: …` line, so the coverage
//! note's advisories are asserted exactly. Every coverage-note finding is `warning`
//! (advisory) — it never gates and never injects a session-start verdict.

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
        "author-coverage-note-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Write a clean one-skill surface at `<root>/.claude/skills/<name>/SKILL.md` — the
/// real Claude Code locus, never a layout invented for the test (`.claude/rules/rust.md`).
/// The `name` matches its directory and the chars are lowercase, so the skill trips no
/// `error`-severity clause and the coverage note is not masked by an unrelated failure.
fn write_skill(root: &Path, name: &str) {
    let dir = root.join(".claude").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    let skill_md = format!(
        "---\n\
name: {name}\n\
description: Use when exercising the {name} path across axes; not for single-axis work.\n\
---\n\
# {name}\n\
\n\
Drive the team through the playbook.\n"
    );
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();
}

/// Write a `.claude/agents/<name>.md` subagent — a real Claude Code surface
/// (code.claude.com/docs/en/settings) that **no built-in kind governs**, so the
/// coverage note must flag it.
fn write_agent(root: &Path, name: &str) {
    let dir = root.join(".claude").join("agents");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join(format!("{name}.md")),
        "---\nname: reviewer\n---\n# Reviewer\n\nA subagent temper models with no kind.\n",
    )
    .unwrap();
}

/// Run `temper check --harness <dir> --reporter github`, returning `(finding lines,
/// exit success)`. Each finding is one `::error`/`::warning …` line.
fn check_harness(harness: &Path) -> (Vec<String>, bool) {
    let output = Command::new(BIN)
        .arg("check")
        .arg("--harness")
        .arg(harness)
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
fn an_ungoverned_agents_dir_is_flagged_beside_the_checked_summary() {
    let harness = tmpdir("with-agents");
    // Two clean skills the gate checks, plus an ungoverned `.claude/agents/` tree.
    write_skill(&harness, "coordinate");
    write_skill(&harness, "review");
    write_agent(&harness, "reviewer");

    let (findings, success) = check_harness(&harness);

    // (1) The checked-summary names each kind's member count — silence never reads as
    // "checked". Exactly one summary, `warning`, reporting the two skills checked.
    let checked = findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    let summary = checked[0];
    assert!(
        summary.starts_with("::warning "),
        "the checked summary is advisory (warn), got: {summary}"
    );
    assert!(
        summary.contains("skill (2)"),
        "the summary reports the two checked skills, got: {summary}"
    );

    // (2) The ungoverned `.claude/agents/` surface is flagged — exactly once, `warning`,
    // naming the surface and carrying its Claude Code docs citation at the point of claim.
    let unmodeled = findings_for(&findings, "coverage.unmodeled-surface");
    let agents: Vec<&&String> = unmodeled
        .iter()
        .filter(|line| line.contains("::.claude/agents:"))
        .collect();
    assert_eq!(
        agents.len(),
        1,
        "expected exactly one flag on .claude/agents, got: {unmodeled:#?}"
    );
    let finding = agents[0];
    assert!(
        finding.starts_with("::warning "),
        "the unmodeled-surface flag is advisory (warn), got: {finding}"
    );
    assert!(
        finding.contains("no kind governs it"),
        "the flag says no kind governs the surface, got: {finding}"
    );
    assert!(
        finding.contains("code.claude.com/docs/en/settings"),
        "the flag cites the Claude Code docs at the point of claim, got: {finding}"
    );

    // The note never gates: no coverage finding is an `::error`, and the clean run
    // still exits success.
    assert!(
        findings_for(&findings, "coverage.checked")
            .iter()
            .chain(findings_for(&findings, "coverage.unmodeled-surface").iter())
            .all(|line| line.starts_with("::warning ")),
        "every coverage-note finding is advisory, got: {findings:#?}"
    );
    assert!(
        success,
        "the advisory coverage note must not fail the run, got: {findings:#?}"
    );
}

#[test]
fn a_harness_with_only_modeled_surfaces_flags_no_unmodeled_surface() {
    let harness = tmpdir("all-modeled");
    // Only a `.claude/skills/` surface — modeled by the `skill` kind. No agents dir,
    // no settings.json, no .mcp.json, so no known ungoverned surface is present.
    write_skill(&harness, "coordinate");

    let (findings, success) = check_harness(&harness);

    // The checked summary still fires — the gate states what it checked.
    assert_eq!(
        findings_for(&findings, "coverage.checked").len(),
        1,
        "the checked summary fires even with no gaps, got: {findings:#?}"
    );
    // But nothing is flagged unmodeled: every present surface is governed.
    assert!(
        findings_for(&findings, "coverage.unmodeled-surface").is_empty(),
        "a fully-modeled harness flags no unmodeled surface, got: {findings:#?}"
    );
    assert!(success, "the clean run exits success, got: {findings:#?}");
}
