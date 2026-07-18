//! Acceptance for the wedge's advisory coverage note: the `check` gate states which
//! kinds checked how many members and names the known Claude Code surfaces present on
//! disk that no kind — built-in or locked custom — governs, so the gate's silence
//! about an unmodeled surface never reads as "checked".
//!
//! Driven across the real process boundary through the one-shot `check --harness` verb
//! (the route session-start takes), over harness-dir fixtures mirroring the real Claude
//! Code layout — `.claude/skills/*` plus, for the partial-governance arm, a bare
//! `.claude/settings.json` whose `hooks` segment the `hook` built-in governs while its
//! permissions/env residue stays unmodeled (the finding names only that residue, never
//! the whole file), and for the locked-kind arm, a `.claude/settings.json` a committed
//! `widget` kind row governs whole. `.mcp.json` is the wholly-ungoverned probe only when
//! the kinds handed in carry no `mcp-server` row; under the built-in set it is governed
//! whole and retires its finding.
//! The GitHub reporter gives a machine-parseable finding
//! set: each finding is one `::warning title=<rule>::<artifact>: …` line, so the
//! coverage note's advisories are asserted exactly. Every coverage-note finding is
//! `warning` (advisory) — it never gates and never injects a session-start verdict.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

mod common;

use common::check_harness;

use temper::coverage_note;
use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember};
use temper::kind::CustomKind;

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

/// Commit a lock at `<root>/.temper/lock.toml` declaring a `widget` kind rooted at
/// `.claude` selecting `settings.json`, and project its one member — a locked custom
/// kind the coverage note's built-in set carries no row for, so the gate discovers it
/// only by reading the lock (`COVERAGE-KIND-AWARE`). `widget` stands in for the
/// not-yet-shipped custom kind here: `agent` no longer fits (AGENT-KIND graduated it
/// to a real built-in), mirroring `command`'s own earlier graduation off this fixture.
///
/// The member's body is a valid-JSON `{}` so the projection it writes over the governed
/// `.claude/settings.json` stays a well-formed manifest — the `hook` built-in reads that
/// same file as JSON, so a markdown body would abort the gate on a parse error.
fn lock_widget_kind(root: &Path) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![KindFactRow {
                unit_shape: Some("file".to_string()),
                ..common::kind_facts("widget", ".claude", "settings.json")
            }],
            ..Declarations::default()
        },
        members: vec![PayloadMember {
            kind: "widget".to_string(),
            name: "settings".to_string(),
            host: None,
            fields: Vec::new(),
            body: "{}\n".to_string(),
            source_path: None,
        }],
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

#[test]
fn a_partially_governed_settings_json_names_only_the_present_ungoverned_residue() {
    let harness = common::tmpdir("with-settings-json");
    // Two clean skills the gate checks, plus a `.claude/settings.json` whose top-level keys
    // are exactly `{permissions, enabledPlugins, extraKnownMarketplaces, hooks}`: the `hook`,
    // `installed-plugin`, and `known-marketplace` built-ins govern the last three, and
    // `permissions` is a present-but-unmodeled key no kind governs. The advisory must classify
    // the file's ACTUAL keys — naming `permissions` as residue and never `env`, which is absent
    // from the file (the field defect this closes).
    write_skill(&harness, "coordinate");
    write_skill(&harness, "review");
    common::write_settings(
        &harness,
        r#"{
  "permissions": { "allow": ["Bash(git status)"] },
  "enabledPlugins": { "formatter@acme": true },
  "extraKnownMarketplaces": { "acme": { "source": { "source": "github", "repo": "acme/mk" } } },
  "hooks": { "SessionStart": [ { "hooks": [ { "type": "command", "command": "echo hi" } ] } ] }
}"#,
    );

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

    // (2) The partially-governed `.claude/settings.json` surface is flagged — exactly once,
    // `warning` — and the finding states only true things: it names the ungoverned
    // `permissions` residue, never claiming the whole file is ungoverned. Contradicting
    // its own `hooks` coverage is the invariant-6 violation this entry closes.
    let unmodeled = common::findings_for(&findings, "coverage.unmodeled-surface");
    let settings: Vec<&&String> = unmodeled
        .iter()
        .filter(|line| line.contains("::.claude/settings.json:"))
        .collect();
    assert_eq!(
        settings.len(),
        1,
        "expected exactly one flag on .claude/settings.json, got: {unmodeled:#?}"
    );
    let finding = settings[0];
    assert!(
        finding.starts_with("::warning "),
        "the unmodeled-surface flag is advisory (warn), got: {finding}"
    );
    assert!(
        finding.contains("partially governed") && finding.contains("permissions"),
        "the flag names the present ungoverned residue, got: {finding}"
    );
    // An absent segment is NEVER asserted — the advisory classifies the file's actual keys, so
    // `env` (which this settings.json does not carry) must not appear anywhere in the finding.
    // `extraKnownMarketplaces` legitimately appears among the *checked* segments now that
    // known-marketplace governs it, so it is no counter-example to that classification.
    assert!(
        !finding.contains("env"),
        "the flag must not name a key absent from the file, got: {finding}"
    );
    assert!(
        !finding.contains("no kind governs it")
            && !finding.contains("temper checks none of its members"),
        "a partially-governed manifest must not claim it is wholly ungoverned, got: {finding}"
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
fn a_fully_represented_settings_json_retires_its_unmodeled_surface_finding() {
    // The write side's terminal state: settings.json fully represented — its `hooks` a
    // modeled collection and its permissions/env residue carried as named opaque fields of
    // a container member. With every segment covered no residue remains to name, so the
    // partial-governance finding retires entirely — the manifest is no longer a gap the
    // note must flag. The `widget` container stands in for that fully-representing kind.
    let harness = common::tmpdir("fully-represented-settings");
    write_skill(&harness, "coordinate");
    common::write_settings(&harness, "{}");
    lock_widget_kind(&harness);

    let (findings, success) = check_harness(&harness);

    // Neither the partial-governance flag nor the full wholly-ungoverned finding survives:
    // a fully-represented manifest reports no coverage.unmodeled-surface at all.
    let settings: Vec<&String> = common::findings_for(&findings, "coverage.unmodeled-surface")
        .into_iter()
        .filter(|line| line.contains("::.claude/settings.json:"))
        .collect();
    assert!(
        settings.is_empty(),
        "a fully-represented settings.json flags no unmodeled surface, got: {settings:#?}"
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
fn a_corrupt_lock_rejects_loud_while_a_missing_one_degrades_to_the_built_in_kinds() {
    // The note reads `<root>/.temper/lock.toml` for any custom kind's `governs` beyond
    // the built-ins. That read must be loud: a corrupt lock silently reading as "no
    // kinds declared" would drop the locked-kind suppression (LOCK-READ-SWALLOW-LOUD).
    // Driven directly against the library, since the CLI gate reads the same lock a
    // step earlier — this pins the note's own read, the swept swallow.
    let empty_kinds: BTreeMap<String, CustomKind> = BTreeMap::new();

    // (1) A corrupt (unparseable) lock rejects loud rather than degrading to
    // built-ins-only suppression.
    let corrupt = common::tmpdir("coverage-note-corrupt-lock");
    fs::create_dir_all(corrupt.join(".temper")).unwrap();
    fs::write(
        corrupt.join(".temper/lock.toml"),
        "this is not = = valid toml",
    )
    .unwrap();
    assert!(
        coverage_note::check(&corrupt, &empty_kinds, &BTreeMap::new()).is_err(),
        "a corrupt lock must reject loud, not degrade to built-ins-only suppression"
    );

    // (2) A genuinely missing lock still degrades to the built-in kinds alone — the
    // note succeeds and still flags an ungoverned present surface.
    let missing = common::tmpdir("coverage-note-missing-lock");
    common::write_mcp_json(&missing, "{}");
    let diagnostics = coverage_note::check(&missing, &empty_kinds, &BTreeMap::new())
        .expect("a missing lock degrades to the built-in kinds, never an error");
    assert!(
        diagnostics
            .iter()
            .any(|d| d.rule == "coverage.unmodeled-surface" && d.artifact == ".mcp.json"),
        "a missing lock still flags the ungoverned surface, got: {diagnostics:#?}"
    );
}

#[test]
fn a_wholly_ungoverned_mcp_json_keeps_the_full_finding_a_governed_one_retires_it() {
    // The partial-governance narrowing must not soften the two ends it brackets: a
    // manifest no kind governs at all still reads the full wholly-ungoverned finding, and
    // one a whole-manifest kind governs still retires it entirely. `.mcp.json` is the
    // probe — wholly its `mcpServers` map, so the `mcp-server` built-in covers it outright.

    // (1) No `mcp-server` kind in scope: the full finding fires, naming the whole file.
    let ungoverned = common::tmpdir("mcp-wholly-ungoverned");
    common::write_mcp_json(&ungoverned, "{}");
    let empty_kinds: BTreeMap<String, CustomKind> = BTreeMap::new();
    let bare = coverage_note::check(&ungoverned, &empty_kinds, &BTreeMap::new()).unwrap();
    let mcp = bare
        .iter()
        .find(|d| d.rule == "coverage.unmodeled-surface" && d.artifact == ".mcp.json")
        .expect("a wholly-ungoverned .mcp.json is still flagged");
    assert!(
        mcp.message.contains("no kind governs it")
            && mcp.message.contains("temper checks none of its members"),
        "a wholly-ungoverned manifest keeps the full finding, got: {}",
        mcp.message
    );

    // (2) The `mcp-server` built-in governs `.mcp.json` whole (its collection spans the
    // manifest), so no finding survives — partial narrowing never reaches a governed file.
    let governed = common::tmpdir("mcp-wholly-governed");
    common::write_mcp_json(&governed, "{}");
    let builtins = temper::builtin_kind::definitions().unwrap();
    let full = coverage_note::check(&governed, &builtins, &BTreeMap::new()).unwrap();
    assert!(
        full.iter()
            .all(|d| !(d.rule == "coverage.unmodeled-surface" && d.artifact == ".mcp.json")),
        "the mcp-server built-in governs .mcp.json whole and retires its finding, got: {full:#?}"
    );
}

#[test]
fn a_locked_custom_kind_suppresses_the_surface_it_governs() {
    let harness = common::tmpdir("locked-widget-kind");
    write_skill(&harness, "coordinate");
    common::write_settings(&harness, "{}");
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
