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
//! set: each finding is one `::<level> title=<rule>::<artifact>: …` line, so the
//! coverage note's levels are asserted exactly. Nothing here gates or injects a
//! session-start verdict: the checked summary is a `::notice` disclosure (no clause
//! behind it, so `--deny-advisories` never promotes it) and each ungoverned-surface
//! flag is a `::warning` advisory — a gap the corpus can close.

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
    // "checked". Exactly one summary, a disclosure `notice`, reporting the two skills
    // checked.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    let summary = checked[0];
    assert!(
        summary.starts_with("::notice "),
        "the checked summary is disclosure (note), never a violation, got: {summary}"
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

    // Neither level gates: no coverage finding is an `::error`, and the clean run still
    // exits success. The two are apart — what was checked is disclosure, what is
    // ungoverned is an advisory the corpus can close.
    assert!(
        common::findings_for(&findings, "coverage.checked")
            .iter()
            .all(|line| line.starts_with("::notice ")),
        "the checked summary is disclosure, got: {findings:#?}"
    );
    assert!(
        common::findings_for(&findings, "coverage.unmodeled-surface")
            .iter()
            .all(|line| line.starts_with("::warning ")),
        "every ungoverned-surface flag is advisory, got: {findings:#?}"
    );
    assert!(
        success,
        "the coverage note must not fail the run, got: {findings:#?}"
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
    // A corrupt lock must reject loud when read — a corrupt lock silently reading as "no
    // kinds declared" would drop the locked-kind suppression (LOCK-READ-SWALLOW-LOUD).
    // The locked kind rows now come from the caller's own read, so a corrupt lock fails
    // at read time, before check runs. A missing lock degrades to an empty slice, and check
    // succeeds with just the built-in kinds.
    let empty_kinds: BTreeMap<String, CustomKind> = BTreeMap::new();

    // (1) A corrupt (unparseable) lock rejects loud when read — this happens before
    // check() is called, since the caller must now provide the parsed kind rows.
    let corrupt = common::tmpdir("coverage-note-corrupt-lock");
    fs::create_dir_all(corrupt.join(".temper")).unwrap();
    fs::write(
        corrupt.join(".temper/lock.toml"),
        "this is not = = valid toml",
    )
    .unwrap();
    let corrupt_read = drift::read_declarations(&corrupt.join(".temper"));
    assert!(
        corrupt_read.is_err(),
        "a corrupt lock must reject loud when read"
    );

    // (2) A genuinely missing lock still degrades to an empty slice — the note succeeds
    // with just the built-in kinds and still flags an ungoverned present surface.
    let missing = common::tmpdir("coverage-note-missing-lock");
    common::write_mcp_json(&missing, "{}");
    let diagnostics = coverage_note::check(
        &missing,
        &empty_kinds,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &[],
    )
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
    let bare = coverage_note::check(
        &ungoverned,
        &empty_kinds,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &[],
    )
    .unwrap();
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
    let builtins = temper::builtin_kind::definitions();
    let full = coverage_note::check(
        &governed,
        &builtins,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &[],
    )
    .unwrap();
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

    // The fixture's own skill is on disk but not in the emitted payload, so this
    // represented harness carries exactly one undeclared member — pinned here rather
    // than left implicit, since it is what the arms above are read against.
    let undeclared = common::findings_for(&findings, "locus.undeclared-member");
    assert_eq!(
        undeclared.len(),
        1,
        "the one skill the lock declares no member for is named, got: {findings:#?}"
    );
    assert!(
        undeclared[0].contains(".claude/skills/coordinate/SKILL.md"),
        "the finding names the stray document's path, got: {}",
        undeclared[0]
    );
    assert!(
        summary.contains("skill (1: 0 declared, 1 undeclared)"),
        "and the disclosure counts it apart from the declared members, got: {summary}"
    );

    assert!(
        success,
        "the advisory coverage note must not fail the run, got: {findings:#?}"
    );
}

#[test]
fn an_embedded_kind_with_nested_members_renders_a_nonzero_embedded_marked_count() {
    // Regression: a lock with nested_member rows against an embedded kind (one with no
    // discoverable locus) should render a nonzero, embedded-marked count in the coverage
    // note's summary line — never a bare (0) that reads as dead. Create a minimal lock
    // with nested_member rows for an embedded kind.
    let harness = common::tmpdir("embedded-nested-members");
    write_skill(&harness, "test-skill");

    let lock_dir = harness.join(".temper");
    std::fs::create_dir_all(&lock_dir).unwrap();
    std::fs::write(
        lock_dir.join("lock.toml"),
        r#"[declaration]

[[declaration.nested_member]]
host = "skill:test-skill"
kind = "supporting-doc"
key = "overview"
"#,
    )
    .unwrap();

    let (findings, _success) = check_harness(&harness);

    // The checked-summary must include the embedded `supporting-doc` kind with a nonzero,
    // embedded-marked count — never (0).
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    let summary = checked[0];
    assert!(
        summary.contains("supporting-doc (1 embedded)"),
        "an embedded kind with nested members must show a nonzero embedded-marked count, got: {summary}"
    );
    // The file-based skill kind should also appear with its discovered count — and,
    // since this fixture's lock carries nested_member rows alone and no provenance row
    // for the skill on disk, that one member is undeclared and is disclosed apart.
    assert!(
        summary.contains("skill (1: 0 declared, 1 undeclared)"),
        "the summary should also include the discovered skill kind, marked undeclared \
         where no lock row declares it, got: {summary}"
    );
}

/// Commit a lock at `<root>/.temper/lock.toml` declaring the `rule` built-in and
/// projecting one member per name through it — a *represented* harness whose
/// `.claude/rules/` locus carries exactly the members the lock's provenance rows name.
fn lock_rules(root: &Path, names: &[&str]) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::rule_kind_facts(None, &[])],
            ..Declarations::default()
        },
        members: names
            .iter()
            .map(|name| common::rule_member(name, None, &format!("# {name}\n\nBody.\n")))
            .collect(),
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

#[test]
fn an_undeclared_document_at_a_governed_locus_is_named_and_counted_apart() {
    // A represented harness carrying one declared rule and one stranger dropped beside
    // it: `emit` will never maintain the stranger and `guard` never bound it, yet Claude
    // Code loads it — so `check` names it rather than counting it as checked.
    let harness = common::tmpdir("undeclared-locus-member");
    lock_rules(&harness, &["declared"]);
    common::write_rule(&harness, "stranger");

    let (findings, success) = check_harness(&harness);

    let undeclared = common::findings_for(&findings, "locus.undeclared-member");
    assert_eq!(
        undeclared.len(),
        1,
        "exactly the one undeclared document is named, got: {findings:#?}"
    );
    let finding = undeclared[0];
    assert!(
        finding.starts_with("::warning "),
        "the finding is advisory, not blocking, got: {finding}"
    );
    assert!(
        finding.contains(".claude/rules/stranger.md"),
        "the finding names the document's path, got: {finding}"
    );
    assert!(
        finding.contains("`rule`"),
        "and the kind whose locus it sits at, got: {finding}"
    );

    // The disclosure counts the stranger apart, so the one line stating what was
    // checked cannot absorb a member the program does not declare.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("rule (2: 1 declared, 1 undeclared)"),
        "the disclosure states declared and undeclared apart, got: {}",
        checked[0]
    );

    assert!(
        success,
        "the finding is advisory — it never fails the run on its own, got: {findings:#?}"
    );
}

#[test]
fn a_declared_and_emitted_member_reports_neither_the_finding_nor_an_undeclared_count() {
    // The same harness with both documents declared and emitted.
    let harness = common::tmpdir("declared-locus-member");
    lock_rules(&harness, &["declared", "stranger"]);

    let (findings, _success) = check_harness(&harness);

    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    // The non-zero declared read is asserted FIRST: without it the silence below could
    // be an empty walk reading as clean.
    assert!(
        checked[0].contains("rule (2)"),
        "both members are discovered and declared, got: {}",
        checked[0]
    );
    assert!(
        !checked[0].contains("undeclared"),
        "and nothing is marked undeclared, got: {}",
        checked[0]
    );
    assert!(
        common::findings_for(&findings, "locus.undeclared-member").is_empty(),
        "a declared, emitted member trips no undeclared finding, got: {findings:#?}"
    );
}

#[test]
fn an_unrepresented_harness_reports_no_undeclared_member_however_many_it_finds() {
    // No lock at all: every discovered member is undeclared, and naming them all would
    // be noise rather than a finding — the built-in default contract still checks them.
    let harness = common::tmpdir("unrepresented-locus-members");
    write_skill(&harness, "coordinate");
    common::write_rule(&harness, "rust");
    common::write_rule(&harness, "collaboration");

    let (findings, _success) = check_harness(&harness);

    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("rule (2)") && checked[0].contains("skill (1)"),
        "the three discovered members are checked and counted as always, got: {}",
        checked[0]
    );
    assert!(
        common::findings_for(&findings, "locus.undeclared-member").is_empty(),
        "an unrepresented harness names no undeclared member, got: {findings:#?}"
    );
}
