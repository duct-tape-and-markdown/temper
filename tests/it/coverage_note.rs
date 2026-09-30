//! Acceptance for the wedge's coverage note's two finding classes: the disclosure of
//! which kinds checked how many members, and each entry directly under `.claude/` that no
//! kind — built-in or locked custom — governs and no known Claude Code surface names.
//!
//! The whole-harness arms drive the real process boundary through the one-shot
//! `check --harness` verb (the route session-start takes), over harness-dir fixtures
//! mirroring the real Claude Code layout. The GitHub reporter gives them a
//! machine-parseable finding set: each finding is one
//! `::<level> title=<rule>::<artifact>: …` line, so the coverage note's levels are
//! asserted exactly. Nothing here gates or injects a session-start verdict: the checked
//! summary is a `::notice` disclosure (no clause behind it, so `--deny-advisories` never
//! promotes it) and each stray-entry flag is a `::warning` advisory — a gap the corpus
//! can close.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::common;

use crate::common::check_harness;

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

/// The `widget` kind's fact row — a locked custom kind the coverage note's built-in set
/// carries no row for, so the gate discovers it only by reading the lock
/// (`COVERAGE-KIND-AWARE`). `widget` stands in for the not-yet-shipped custom kind here:
/// `agent` no longer fits (AGENT-KIND graduated it to a real built-in), mirroring
/// `command`'s own earlier graduation off this fixture.
///
/// The locus is the kind's own `.claude/widgets/*.json`, never `.claude/settings.json`:
/// that file is the built-in `settings` kind's, and a second document kind claiming it
/// whole is a `kind.governs-collision` — a document's kind is its position alone.
fn widget_kind_facts(governs_root: &str, governs_glob: &str) -> KindFactRow {
    KindFactRow {
        unit_shape: Some("file".to_string()),
        ..common::kind_facts("widget", governs_root, governs_glob)
    }
}

/// Commit a lock at `<root>/.temper/lock.toml` declaring the `widget` kind and project
/// its one member at `.claude/widgets/panel.json`. The member's body is a valid-JSON `{}`
/// so the projection stays a well-formed document of the format its locus implies.
fn lock_widget_kind(root: &Path) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![widget_kind_facts(".claude/widgets", "*.json")],
            ..Declarations::default()
        },
        members: vec![PayloadMember {
            kind: "widget".to_string(),
            name: "panel".to_string(),
            host: None,
            fields: Vec::new(),
            body: "{}\n".to_string(),
            source_path: None,
        }],
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

#[test]
fn the_note_reports_exactly_the_disclosure_and_the_unclaimed_entry() {
    // The note's whole surface, over a harness carrying a governed member, the one known
    // Claude Code surface the built-ins govern whole, and a stray nothing claims: the
    // disclosure of what was checked and that one stray, and no third `coverage.` class.
    let harness = common::tmpdir("note-two-classes");
    write_skill(&harness, "coordinate");
    common::write_settings(&harness, r#"{ "permissions": { "allow": [] } }"#);
    fs::write(harness.join(".claude/stray.md"), "").unwrap();

    let (findings, success) = check_harness(&harness);

    let coverage: Vec<&String> = findings
        .iter()
        .filter(|line| line.contains("title=coverage."))
        .collect();
    assert_eq!(
        coverage.len(),
        2,
        "the note reports exactly its two classes, got: {findings:#?}"
    );
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(checked.len(), 1, "{findings:#?}");
    assert!(
        checked[0].starts_with("::notice ") && checked[0].contains("skill (1)"),
        "the disclosure is a note naming what it checked, got: {}",
        checked[0]
    );
    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert_eq!(unclaimed.len(), 1, "{findings:#?}");
    assert!(
        unclaimed[0].starts_with("::warning ") && unclaimed[0].contains(".claude/stray.md"),
        "the one stray is an advisory naming its path, and it is not \
         `.claude/settings.json` — the built-ins govern that, got: {}",
        unclaimed[0]
    );
    assert!(success, "neither class gates the run, got: {findings:#?}");
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
    // with just the kinds it was handed and still names the stray it finds.
    let missing = common::tmpdir("coverage-note-missing-lock");
    fs::create_dir_all(missing.join(".claude")).unwrap();
    fs::write(missing.join(".claude/stray.md"), "").unwrap();
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
            .any(|d| d.rule == "coverage.unclaimed-entry" && d.artifact == ".claude/stray.md"),
        "a missing lock still names the unclaimed entry, got: {diagnostics:#?}"
    );
}

#[test]
fn a_locked_custom_kinds_members_are_counted_beside_the_built_ins() {
    let harness = common::tmpdir("locked-widget-kind");
    write_skill(&harness, "coordinate");
    lock_widget_kind(&harness);

    let (findings, success) = check_harness(&harness);

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
    let undeclared = common::findings_for(&findings, "root.locus-declared");
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

    // Written by the real lock writer (`drift::emit`), which renders this family
    // (`NestedMemberRow::to_table`) — and writes no provenance row for a declarations-only
    // payload, so the skill on disk stays undeclared, exactly as the case requires.
    common::write_lock(
        &harness,
        Declarations {
            nested_members: vec![drift::NestedMemberRow {
                host: "skill:test-skill".to_string(),
                kind: "supporting-doc".to_string(),
                key: "overview".to_string(),
                leaves: BTreeMap::new(),
                collections: Vec::new(),
                placed_edges: None,
                rendered_lines: None,
                rendered_chars: None,
            }],
            ..Declarations::default()
        },
    );

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
    // And the headline counts that embedded member too: two members were checked here —
    // the skill on disk and the `supporting-doc` row the embedded dispatcher judges — so
    // the one line stating what was checked may not say `checked 1 member` over them.
    assert!(
        summary.contains("checked 2 members across"),
        "the total must count the embedded member alongside the discovered skill, got: \
         {summary}"
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

    let undeclared = common::findings_for(&findings, "root.locus-declared");
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

/// The disclosure is not the clause's finding. `coverage.checked` states the undeclared
/// count off the same single pass the findings come from, and `src/gate.rs` feeds it
/// whatever the root `locus-declared` clause declares — so dialing that clause moves the
/// finding's weight and leaves the count printed here exactly where it was.
#[test]
fn the_disclosures_undeclared_count_is_unchanged_by_the_clauses_severity() {
    let harness = common::tmpdir("undeclared-locus-dialed");
    lock_rules(&harness, &["declared"]);
    common::write_rule(&harness, "stranger");

    // Undialed: the shipped default's `advisory`.
    let (findings, success) = check_harness(&harness);
    let advisory = common::findings_for(&findings, "root.locus-declared");
    assert_eq!(advisory.len(), 1, "got: {findings:#?}");
    assert!(
        advisory[0].starts_with("::warning "),
        "got: {}",
        advisory[0]
    );
    let checked = common::findings_for(&findings, "coverage.checked");
    assert!(
        checked[0].contains("rule (2: 1 declared, 1 undeclared)"),
        "got: {}",
        checked[0]
    );
    assert!(success, "got: {findings:#?}");

    common::write_sibling(
        &harness,
        ".temper/dial.toml",
        "name = \"workstation\"\n\n[[clause]]\nlabel = \"root.locus-declared\"\nseverity = \"required\"\n",
    );

    let (findings, success) = check_harness(&harness);
    let dialed = common::findings_for(&findings, "root.locus-declared");
    assert_eq!(dialed.len(), 1, "got: {findings:#?}");
    assert!(
        dialed[0].starts_with("::error "),
        "the dial reaches the finding's weight, got: {}",
        dialed[0]
    );
    assert!(!success, "and the run now blocks, got: {findings:#?}");

    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(checked.len(), 1, "got: {findings:#?}");
    assert!(
        checked[0].contains("rule (2: 1 declared, 1 undeclared)"),
        "and the disclosure says the same thing at either severity, got: {}",
        checked[0]
    );
    assert!(
        checked[0].starts_with("::notice "),
        "the disclosure is still a note, never promoted with the clause, got: {}",
        checked[0]
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
        common::findings_for(&findings, "root.locus-declared").is_empty(),
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
        common::findings_for(&findings, "root.locus-declared").is_empty(),
        "an unrepresented harness names no undeclared member, got: {findings:#?}"
    );
}
