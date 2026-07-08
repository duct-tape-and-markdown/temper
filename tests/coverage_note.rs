//! Acceptance for the wedge's advisory coverage note: the `check` gate states which
//! kinds checked how many members and names the known Claude Code surfaces present on
//! disk that no kind — built-in or locked custom — governs, so the gate's silence
//! about an unmodeled surface never reads as "checked".
//!
//! Driven across the real process boundary through the one-shot `check --harness` verb
//! (the route session-start takes), over harness-dir fixtures mirroring the real Claude
//! Code layout — `.claude/skills/*` plus, for the gap arm, a bare `.mcp.json` no kind
//! governs, and for the locked-kind arm, a `.claude/settings.json` a committed
//! `widget` kind row governs (`.claude/agents` no longer fits either arm: the `agent`
//! built-in now governs it unconditionally, so it is never an available gap —
//! mirroring `.claude/commands`'s own graduation off this fixture).
//! The GitHub reporter gives a machine-parseable finding
//! set: each finding is one `::warning title=<rule>::<artifact>: …` line, so the
//! coverage note's advisories are asserted exactly. Every coverage-note finding is
//! `warning` (advisory) — it never gates and never injects a session-start verdict.

use std::fs;
use std::path::Path;

mod common;

use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember};

/// Write a clean one-skill surface at `<root>/.claude/skills/<name>/SKILL.md` — the
/// real Claude Code locus, never a layout invented for the test (`.claude/rules/rust.md`).
/// The `name` matches its directory and the chars are lowercase, so the skill trips no
/// `error`-severity clause and the coverage note is not masked by an unrelated failure.
fn write_skill(root: &Path, name: &str) {
    let skill_md = format!(
        "---\n\
name: {name}\n\
description: Use when exercising the {name} path across axes; not for single-axis work.\n\
---\n\
# {name}\n\
\n\
Drive the team through the playbook.\n"
    );
    common::write_skill(root, name, &skill_md);
}

/// Write a bare `.mcp.json` — a real Claude Code surface (code.claude.com/docs/en/settings)
/// that **no built-in kind governs**, so the coverage note must flag it.
fn write_mcp_json(root: &Path) {
    fs::write(root.join(".mcp.json"), "{}").unwrap();
}

/// Commit a lock at `<root>/.temper/lock.toml` declaring a `widget` kind rooted at
/// `.claude` selecting `settings.json`, and project its one member — a locked custom
/// kind the coverage note's built-in set carries no row for, so the gate discovers it
/// only by reading the lock (`COVERAGE-KIND-AWARE`). `widget` stands in for the
/// not-yet-shipped custom kind here: `agent` no longer fits (AGENT-KIND graduated it
/// to a real built-in), mirroring `command`'s own earlier graduation off this fixture.
fn lock_widget_kind(root: &Path) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![KindFactRow {
                name: "widget".to_string(),
                provider: None,
                governs_root: ".claude".to_string(),
                governs_glob: "settings.json".to_string(),
                format: None,
                unit_shape: Some("file".to_string()),
                registration: Vec::new(),
                templates: Vec::new(),
            }],
            ..Declarations::default()
        },
        members: vec![PayloadMember {
            kind: "widget".to_string(),
            name: "settings".to_string(),
            fields: Vec::new(),
            body: "# Settings\n\nProject settings.\n".to_string(),
            source_path: None,
        }],
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

/// Run `temper check --harness <dir> --reporter github`, returning `(finding lines,
/// exit success)`. Each finding is one `::error`/`::warning …` line.
fn check_harness(harness: &Path) -> (Vec<String>, bool) {
    let run = common::check_in(
        harness,
        &["--harness", harness.to_str().unwrap()],
        Some("github"),
    );
    let findings = run
        .output
        .lines()
        .filter(|line| line.starts_with("::"))
        .map(str::to_string)
        .collect();
    (findings, run.ok)
}

#[test]
fn an_ungoverned_mcp_json_is_flagged_beside_the_checked_summary() {
    let harness = common::tmpdir("with-mcp-json");
    // Two clean skills the gate checks, plus an ungoverned `.mcp.json`.
    write_skill(&harness, "coordinate");
    write_skill(&harness, "review");
    write_mcp_json(&harness);

    let (findings, success) = check_harness(&harness);

    // (1) The checked-summary names each kind's member count — silence never reads as
    // "checked". Exactly one summary, `warning`, reporting the two skills checked.
    let checked = common::findings_for(&findings, "coverage.checked");
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

    // (2) The ungoverned `.mcp.json` surface is flagged — exactly once, `warning`,
    // naming the surface and carrying its Claude Code docs citation at the point of claim.
    let unmodeled = common::findings_for(&findings, "coverage.unmodeled-surface");
    let mcp: Vec<&&String> = unmodeled
        .iter()
        .filter(|line| line.contains("::.mcp.json:"))
        .collect();
    assert_eq!(
        mcp.len(),
        1,
        "expected exactly one flag on .mcp.json, got: {unmodeled:#?}"
    );
    let finding = mcp[0];
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
        common::findings_for(&findings, "coverage.checked")
            .iter()
            .chain(common::findings_for(&findings, "coverage.unmodeled-surface").iter())
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
    let harness = common::tmpdir("all-modeled");
    // Only a `.claude/skills/` surface — modeled by the `skill` kind. No
    // settings.json, no .mcp.json, so no known ungoverned surface is present.
    write_skill(&harness, "coordinate");

    let (findings, success) = check_harness(&harness);

    // The checked summary still fires — the gate states what it checked.
    assert_eq!(
        common::findings_for(&findings, "coverage.checked").len(),
        1,
        "the checked summary fires even with no gaps, got: {findings:#?}"
    );
    // But nothing is flagged unmodeled: every present surface is governed.
    assert!(
        common::findings_for(&findings, "coverage.unmodeled-surface").is_empty(),
        "a fully-modeled harness flags no unmodeled surface, got: {findings:#?}"
    );
    assert!(success, "the clean run exits success, got: {findings:#?}");
}

#[test]
fn a_locked_custom_kind_suppresses_the_surface_it_governs() {
    let harness = common::tmpdir("locked-widget-kind");
    write_skill(&harness, "coordinate");
    fs::create_dir_all(harness.join(".claude")).unwrap();
    fs::write(harness.join(".claude/settings.json"), "{}").unwrap();
    lock_widget_kind(&harness);

    let (findings, success) = check_harness(&harness);

    // `.claude/settings.json` is present and governed by the locked `widget` kind,
    // so it is never flagged unmodeled.
    let unmodeled = common::findings_for(&findings, "coverage.unmodeled-surface");
    assert!(
        unmodeled
            .iter()
            .all(|line| !line.contains("::.claude/settings.json:")),
        "a locked custom kind governing .claude/settings.json must suppress the finding, got: {unmodeled:#?}"
    );

    // The checked-count message folds the custom kind's member in beside the
    // built-ins and carries no "built-in" qualifier that would misdescribe it.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    let summary = checked[0];
    assert!(
        summary.contains("widget (1)"),
        "the summary counts the locked custom kind's member, got: {summary}"
    );
    assert!(
        !summary.contains("built-in"),
        "the checked-count message must not say 'built-in' when a custom kind is counted, got: {summary}"
    );

    assert!(
        success,
        "the advisory coverage note must not fail the run, got: {findings:#?}"
    );
}
