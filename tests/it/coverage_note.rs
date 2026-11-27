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
use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload};
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
/// whole is a `kind.locus-collision` — a document's kind is its position alone.
fn widget_kind_facts(locus_root: &str, locus_glob: &str) -> KindFactRow {
    KindFactRow {
        unit_shape: Some("file".to_string()),
        ..common::kind_facts("widget", locus_root, locus_glob)
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
        members: vec![common::payload_member("widget", "panel", None, "{}\n")],
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
    // The governing kind set now comes from the caller's own read, so a corrupt lock fails
    // at read time, before check runs; a missing one leaves the caller holding the
    // built-in kinds alone, and the scan still runs over them.
    let empty_kinds: BTreeMap<String, CustomKind> = BTreeMap::new();

    // (1) A corrupt (unparseable) lock rejects loud when read — this happens before
    // check() is called, since the caller must now provide the governing kind set.
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

    // (2) A genuinely missing lock degrades to the kinds the caller holds — the note
    // scans over exactly those and still names the stray it finds.
    let missing = common::tmpdir("coverage-note-missing-lock");
    fs::create_dir_all(missing.join(".claude")).unwrap();
    fs::write(missing.join(".claude/stray.md"), "").unwrap();
    let diagnostics = coverage_note::check(
        &temper::import::Discovery::new(&missing),
        &empty_kinds,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &std::collections::BTreeSet::new(),
        0,
    );
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
        "name = \"workstation\"\n\n[[clause]]\nid = \"root.locus-declared\"\nseverity = \"required\"\n",
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

/// A provenance row carrying the pre-0024 `./` spelling declares the document discovered
/// bare at its governed locus. The two sides of the join reach it by different roads — a
/// row is read off the lock, a site is derived from disk — and each arrives through
/// [`temper::path::HarnessRelativePath`]'s own door, so an older engine's `./x` row meets
/// the bare `x` site as one key. Left to a call-site re-spell, the tolerance lived beside
/// the join instead of in the column's type, and the row read as naming nothing: the
/// declared, emitted member would be flagged a stranger at its own locus and counted
/// undeclared.
#[test]
fn a_dot_slash_spelled_provenance_row_declares_the_member_discovered_bare() {
    let harness = common::tmpdir("dot-slash-row-locus-member");
    lock_rules(&harness, &["declared"]);

    // Respell the emitted row the way an engine before decision 0024 wrote it. The
    // document on disk is untouched, so discovery still names it bare — the spelling fork
    // the join has to absorb.
    let lock_path = harness
        .join(temper::WORKSPACE_DIR)
        .join(temper::LOCK_FILENAME);
    let lock = fs::read_to_string(&lock_path).expect("the emitted lock must be readable");
    let respelled = lock.replace(
        r#"source_path = ".claude/rules/"#,
        r#"source_path = "./.claude/rules/"#,
    );
    // Non-vacuity: the case is only about the older spelling if the row really carries it.
    assert_ne!(
        respelled, lock,
        "the fixture must find a bare-spelled row to respell: {lock}"
    );
    fs::write(&lock_path, &respelled).expect("the respelled lock must be writable");

    let (findings, _success) = check_harness(&harness);

    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    // Asserted FIRST: without a non-zero `rule` read the silence below could be an empty
    // walk reading as clean.
    assert!(
        checked[0].contains("rule (1)"),
        "the member is discovered and reads as declared, got: {}",
        checked[0]
    );
    assert!(
        !checked[0].contains("undeclared"),
        "and nothing is counted undeclared, got: {}",
        checked[0]
    );
    assert!(
        common::findings_for(&findings, "root.locus-declared").is_empty(),
        "a `./`-spelled row names its document, so no stranger is reported, got: \
         {findings:#?}"
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

/// Commit a lock declaring both the `widget` custom kind and the `rule` built-in, each
/// with one projected member — the two-kind represented harness the loader-claim arms
/// need, since the claim's discriminator is whose kind governs the locus.
fn lock_widget_and_rule(root: &Path) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                widget_kind_facts(".claude/widgets", "*.json"),
                common::rule_kind_facts(None, &[]),
            ],
            ..Declarations::default()
        },
        members: vec![
            common::payload_member("widget", "panel", None, "{}\n"),
            common::rule_member("declared", None, "# declared\n\nBody.\n"),
        ],
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

/// The stray-at-a-governed-locus finding asserts only what temper knows about who reads
/// the document. Claude Code's own loci are the built-in kinds', so a stray under one is
/// context Claude Code loads and the finding says so; a custom kind's locus is the
/// author's, and nothing temper holds says what reads it — so the claim is dropped there,
/// exactly where the `local` remedy is offered instead.
///
/// Both arms in one harness, each pinned positively and negatively: an arm that simply
/// stopped producing a finding would otherwise read as clean.
#[test]
fn a_custom_kinds_locus_stray_names_no_loader_while_a_built_ins_does() {
    let harness = common::tmpdir("locus-stray-loader-claim");
    lock_widget_and_rule(&harness);
    common::write_sibling(&harness, ".claude/widgets/stray.json", "{}\n");
    common::write_rule(&harness, "stranger");

    let (findings, success) = check_harness(&harness);

    let undeclared = common::findings_for(&findings, "root.locus-declared");
    assert_eq!(
        undeclared.len(),
        2,
        "one stray at each kind's locus is named, got: {findings:#?}"
    );
    let custom = undeclared
        .iter()
        .find(|f| f.contains(".claude/widgets/stray.json"))
        .unwrap_or_else(|| panic!("the custom kind's stray is named, got: {undeclared:#?}"));
    let built_in = undeclared
        .iter()
        .find(|f| f.contains(".claude/rules/stranger.md"))
        .unwrap_or_else(|| panic!("the built-in kind's stray is named, got: {undeclared:#?}"));

    assert!(
        custom.contains("`widget`") && custom.contains("declare its kind `local`"),
        "the custom arm names its kind and offers the remedy only it can take, got: {custom}"
    );
    assert!(
        !custom.contains("Claude Code"),
        "and claims nothing about who loads a locus the author's own kind governs, got: \
         {custom}"
    );

    assert!(
        built_in.contains("`rule`") && built_in.contains("yet Claude Code loads it"),
        "the built-in arm still says Claude Code loads its locus, got: {built_in}"
    );
    assert!(
        !built_in.contains("declare its kind `local`"),
        "and still offers no remedy a built-in's commitment refuses, got: {built_in}"
    );

    assert!(success, "both findings stay advisory, got: {findings:#?}");
}

/// Commit a lock relocating the built-in `rule` kind to `.claude/policies/*.md` and
/// project one member through it — the one locus `rule` governs on this run, reachable
/// only through the overlay the lock's row carries, never the kind's embedded root.
fn lock_relocated_rule(root: &Path, name: &str) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![KindFactRow {
                locus_root: Some(".claude/policies".to_string()),
                ..common::rule_kind_facts(None, &[])
            }],
            ..Declarations::default()
        },
        members: vec![common::rule_member(
            name,
            None,
            &format!("# {name}\n\nBody.\n"),
        )],
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

#[test]
fn a_relocated_built_ins_new_locus_is_governed_and_its_old_one_is_not() {
    // The stray scan's governing set is the run's *effective* one, so a relocation row
    // moves which `.claude/` entry `rule` claims. Read off the raw embedded definitions
    // the scan would claim `.claude/rules` — a directory this harness's program does not
    // govern — and name the real locus a stray.
    let harness = common::tmpdir("relocated-builtin-locus");
    lock_relocated_rule(&harness, "house-style");
    common::write_sibling(&harness, ".claude/rules/orphan.md", "# orphan\n\nBody.\n");
    fs::write(harness.join(".claude/stray.md"), "").unwrap();

    let (findings, success) = check_harness(&harness);

    // Non-vacuity, both halves: the relocation reached the run (the disclosure counts the
    // member at its new locus) and the scan found entries to classify (the plain stray is
    // named), so neither assertion below can pass over an empty set.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(checked.len(), 1, "{findings:#?}");
    assert!(
        checked[0].contains("rule (1)"),
        "the relocated member is discovered and disclosed, got: {}",
        checked[0]
    );
    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/stray.md")),
        "the scan is populated and still names an entry nothing governs, got: {findings:#?}"
    );

    assert!(
        !unclaimed.iter().any(|f| f.contains(".claude/policies")),
        "the locus `rule` was relocated to is governed, never a stray, got: {unclaimed:#?}"
    );
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/rules")),
        "and the root it was relocated off is claimed by nothing, got: {unclaimed:#?}"
    );
    assert!(success, "both classes stay advisory, got: {findings:#?}");
}

#[test]
fn the_claude_dir_memory_member_is_disclosed_as_checked_and_never_also_a_stray() {
    // `.claude/CLAUDE.md` is a documented home for a project memory file, and the
    // `memory` kind's any-depth `**/CLAUDE.md` locus covers it — so the one run that
    // checks it may not also report it unclaimed. Read as a glob *leaf* against the
    // entry's parent the two halves disagreed: `**/CLAUDE.md` matched, the leaf compare
    // (root `.` folded to the empty string, parent `.claude`) did not, and the same
    // output said both "checked" and "nothing covers it" about one file.
    let harness = common::tmpdir("claude-dir-memory-member");
    fs::create_dir_all(harness.join(".claude")).unwrap();
    fs::write(
        harness.join(".claude/CLAUDE.md"),
        "# Memory\n\nProject guidance.\n",
    )
    .unwrap();
    fs::write(harness.join(".claude/stray.md"), "").unwrap();

    let (findings, success) = check_harness(&harness);

    // Non-vacuity, both halves: the member really was checked (the disclosure counts it),
    // and the scan really classified entries (it names the stray) — so neither assertion
    // below can pass over an empty set.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(checked.len(), 1, "{findings:#?}");
    assert!(
        checked[0].contains("memory (1)"),
        "the `.claude/` memory file is discovered and disclosed as checked, got: {}",
        checked[0]
    );
    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/stray.md")),
        "the scan is populated and still names the entry nothing governs, got: {findings:#?}"
    );

    assert!(
        !unclaimed.iter().any(|f| f.contains(".claude/CLAUDE.md")),
        "a checked member is never also unclaimed — one file, one verdict, got: \
         {unclaimed:#?}"
    );
    assert!(
        success,
        "the coverage note stays advisory, got: {findings:#?}"
    );
}

#[test]
fn a_file_a_memory_import_reaches_draws_no_unclaimed_entry_finding() {
    // The root memory file imports a `.claude/` file no kind governs: the import edge is
    // what loads it, so the importer claims it. A sibling nothing imports stays a stray,
    // and a reached file that has its own kind is still judged — counted — by that kind.
    let harness = common::tmpdir("import-reached-claimed");
    fs::create_dir_all(harness.join(".claude/rules")).unwrap();
    fs::write(
        harness.join("CLAUDE.md"),
        "# Memory\n\nSee @.claude/notes.md and @.claude/rules/style.md.\n",
    )
    .unwrap();
    fs::write(harness.join(".claude/notes.md"), "# Notes\n").unwrap();
    fs::write(
        harness.join(".claude/rules/style.md"),
        "# Style\n\nBe terse.\n",
    )
    .unwrap();
    fs::write(harness.join(".claude/stray.md"), "").unwrap();

    let (findings, success) = check_harness(&harness);

    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/stray.md")),
        "the scan is populated and still names the file nothing imports, got: {findings:#?}"
    );
    assert!(
        !unclaimed.iter().any(|f| f.contains(".claude/notes.md")),
        "an import-reached file is claimed by its importer, got: {unclaimed:#?}"
    );
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(checked.len(), 1, "{findings:#?}");
    assert!(
        checked[0].contains("rule (1)") && checked[0].contains("memory (1)"),
        "a reached file with its own kind is still judged by that kind, got: {}",
        checked[0]
    );
    assert!(
        success,
        "the coverage note stays advisory, got: {findings:#?}"
    );
}

/// A root memory file importing `.claude/notes/a.md`, with `.claude/notes/` holding a.md
/// plus each of `extra` — the directory-claim fixture.
fn write_notes_dir_importer(name: &str, extra: &[&str]) -> std::path::PathBuf {
    let harness = common::tmpdir(name);
    fs::create_dir_all(harness.join(".claude/notes")).unwrap();
    fs::write(
        harness.join("CLAUDE.md"),
        "# Memory\n\nSee @.claude/notes/a.md for notes.\n",
    )
    .unwrap();
    fs::write(harness.join(".claude/notes/a.md"), "# A\n").unwrap();
    for file in extra {
        fs::write(harness.join(".claude/notes").join(file), "# Other\n").unwrap();
    }
    fs::write(harness.join(".claude/stray.md"), "").unwrap();
    harness
}

#[test]
fn a_directory_whose_every_file_an_import_reaches_draws_no_unclaimed_entry_finding() {
    let harness = write_notes_dir_importer("import-reached-dir-claimed", &[]);

    let (findings, success) = check_harness(&harness);

    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/stray.md")),
        "the scan is populated, got: {findings:#?}"
    );
    assert!(
        !unclaimed.iter().any(|f| f.contains(".claude/notes")),
        "a fully import-reached directory is claimed, got: {unclaimed:#?}"
    );
    assert!(success, "{findings:#?}");
}

#[test]
fn a_directory_holding_an_unreached_file_still_draws_an_unclaimed_entry_finding() {
    let harness = write_notes_dir_importer("import-reached-dir-partial", &["b.md"]);

    let (findings, _) = check_harness(&harness);

    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/notes")),
        "a directory holding an unimported file stays a stray, got: {unclaimed:#?}"
    );
}

/// Write a skill whose body links its `reference.md` supporting doc in prose alone — a
/// hand-typed link, which declares no edge.
fn write_skill_with_prose_linked_doc(root: &Path, name: &str) {
    let skill_md = format!(
        "---\n\
name: {name}\n\
description: Use when exercising the {name} path across axes; not for single-axis work.\n\
---\n\
# {name}\n\
\n\
See [the reference](reference.md) for the playbook.\n"
    );
    common::write_skill(root, name, &skill_md);
    common::write_sibling(
        root,
        &format!(".claude/skills/{name}/reference.md"),
        "# Reference\n",
    );
}

#[test]
fn an_unrepresented_harness_reports_unreached_supporting_docs_as_one_coverage_line() {
    let harness = common::tmpdir("unrepresented-unreached-docs");
    write_skill_with_prose_linked_doc(&harness, "alpha");
    write_skill_with_prose_linked_doc(&harness, "beta");

    let (findings, _success) = check_harness(&harness);

    assert!(
        common::findings_for(&findings, "supporting-doc.degree").is_empty(),
        "an unrepresented harness files no per-doc degree finding, got: {findings:#?}"
    );
    let line = common::findings_for(&findings, "coverage.unreached-supporting-docs");
    assert_eq!(
        line.len(),
        1,
        "expected one coverage line, got: {findings:#?}"
    );
    assert!(
        line[0].starts_with("::notice") && line[0].contains("2 supporting docs"),
        "the one line is a disclosure counting both docs, got: {}",
        line[0]
    );
}

#[test]
fn a_represented_harness_reports_an_unreached_supporting_doc_per_doc() {
    let harness = common::tmpdir("represented-unreached-docs");
    lock_rules(&harness, &["declared"]);
    write_skill_with_prose_linked_doc(&harness, "alpha");
    write_skill_with_prose_linked_doc(&harness, "beta");

    let (findings, _success) = check_harness(&harness);

    assert_eq!(
        common::findings_for(&findings, "supporting-doc.degree").len(),
        2,
        "a represented harness judges each unreached doc, got: {findings:#?}"
    );
    assert!(
        common::findings_for(&findings, "coverage.unreached-supporting-docs").is_empty(),
        "a represented harness folds nothing into a coverage line, got: {findings:#?}"
    );
}

/// A harness holding a `claude --worktree` checkout at `.claude/worktrees/<name>/`.
fn write_claude_worktree(name: &str) -> std::path::PathBuf {
    let harness = common::tmpdir(name);
    common::write_sibling(
        &harness,
        ".claude/worktrees/feature/README.md",
        "# Feature\n",
    );
    harness
}

#[test]
fn the_claude_worktrees_unclaimed_entry_quotes_the_docs_gitignore_advice() {
    let harness = write_claude_worktree("worktrees-unignored");

    let run = common::check_harness_in(&harness, None);

    assert_eq!(
        run.output.matches("coverage.unclaimed-entry").count(),
        1,
        "the unignored worktrees folder draws exactly one unclaimed entry, got: {}",
        run.output
    );
    assert!(
        run.output.contains(".claude/worktrees"),
        "the finding names the entry, got: {}",
        run.output
    );
    assert!(
        run.output
            .contains("\"Add `.claude/worktrees/` to your `.gitignore`\"")
            && run.output.contains("code.claude.com/docs/en/worktrees"),
        "the guidance quotes and cites the worktrees docs' advice, got: {}",
        run.output
    );
    assert!(run.ok, "the finding stays advisory, got: {}", run.output);
}

#[test]
fn a_gitignored_claude_worktrees_folder_fires_no_unclaimed_entry() {
    // The remedy the finding quotes, followed as the docs spell it: the harness root's
    // `.gitignore` names the folder, and the scan reads that file.
    let harness = write_claude_worktree("worktrees-gitignored");
    common::write_sibling(&harness, ".gitignore", ".claude/worktrees/\n");
    fs::write(harness.join(".claude/stray.md"), "").unwrap();

    let (findings, success) = check_harness(&harness);

    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/stray.md")),
        "the scan is populated, got: {findings:#?}"
    );
    assert!(
        !unclaimed.iter().any(|f| f.contains(".claude/worktrees")),
        "a gitignored worktrees folder is not authored here, got: {unclaimed:#?}"
    );
    assert!(success, "{findings:#?}");
}

/// A program declaring each script in `.claude/hooks/` as a `{ file }` tool, one hook
/// handler running each.
const SCRIPT_TOOLS_PROGRAM: &str = r#"
import { harness, tool } from "@dtmd/temper";
import { hook } from "@dtmd/temper/claude-code";

const lint = tool({ file: ".claude/hooks/lint.sh" });
const audit = tool({ file: ".claude/hooks/audit.sh" });
const after = hook({
  event: "PostToolUse",
  matcher: "Edit",
  hooks: [
    { type: "command", command: ".claude/hooks/lint.sh", runs: lint },
    { type: "command", command: ".claude/hooks/audit.sh", runs: audit },
  ],
});

export default harness({ members: [lint, audit, after] });
"#;

#[test]
fn a_folder_of_declared_script_tools_fires_no_unclaimed_entry() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("script-tools-claimed", SCRIPT_TOOLS_PROGRAM);
    common::write_sibling(&harness, ".claude/hooks/lint.sh", "#!/bin/sh\n");
    common::write_sibling(&harness, ".claude/hooks/audit.sh", "#!/bin/sh\n");
    common::write_sibling(&harness, ".claude/scratch/notes.sh", "#!/bin/sh\n");
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let (findings, _) = check_harness(&harness);

    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/scratch")),
        "the scan is populated: an undeclared script folder still fires, got: {findings:#?}"
    );
    assert!(
        !unclaimed.iter().any(|f| f.contains(".claude/hooks")),
        "every script in the folder is a declared tool's input, so the folder is claimed, \
         got: {unclaimed:#?}"
    );
}

#[test]
fn an_unrepresented_harness_whose_settings_hold_a_hook_command_draws_the_script_note() {
    let harness = common::tmpdir("unrepresented-hook-command");
    common::write_settings(
        &harness,
        r#"{ "hooks": { "PreToolUse": [ { "matcher": "Bash", "hooks": [ { "type": "command", "command": "./scripts/guard.sh" } ] } ] } }"#,
    );

    let (findings, _success) = check_harness(&harness);

    let note = common::findings_for(&findings, "coverage.unrepresented-scripts");
    assert_eq!(
        note.len(),
        1,
        "expected one script note, got: {findings:#?}"
    );
    assert!(
        note[0].starts_with("::notice")
            && note[0].contains("`{ file }` tool")
            && !note[0].contains("guard.sh"),
        "the note is a disclosure naming no script, got: {}",
        note[0]
    );
}

#[test]
fn an_unrepresented_harness_whose_status_line_runs_a_command_draws_the_script_note() {
    let harness = common::tmpdir("unrepresented-status-line");
    common::write_sibling(
        &harness,
        ".claude/settings.local.json",
        r#"{ "statusLine": { "type": "command", "command": "~/.claude/statusline.sh" } }"#,
    );

    let (findings, _success) = check_harness(&harness);

    assert_eq!(
        common::findings_for(&findings, "coverage.unrepresented-scripts").len(),
        1,
        "{findings:#?}"
    );
}

#[test]
fn an_unrepresented_harness_with_no_hook_or_status_line_command_draws_no_script_note() {
    let harness = common::tmpdir("unrepresented-no-command");
    common::write_settings(
        &harness,
        r#"{ "permissions": { "allow": [] }, "hooks": { "Stop": [ { "hooks": [ { "type": "http", "url": "https://example.test/hook" } ] } ] } }"#,
    );

    let (findings, _success) = check_harness(&harness);

    assert!(
        common::findings_for(&findings, "coverage.unrepresented-scripts").is_empty(),
        "no command runs, so no script note, got: {findings:#?}"
    );
}

#[test]
fn a_represented_harness_whose_settings_hold_a_hook_command_draws_no_script_note() {
    let harness = common::tmpdir("represented-hook-command");
    lock_rules(&harness, &["declared"]);
    common::write_settings(
        &harness,
        r#"{ "hooks": { "Stop": [ { "hooks": [ { "type": "command", "command": "echo done" } ] } ] } }"#,
    );

    let (findings, _success) = check_harness(&harness);

    assert!(
        common::findings_for(&findings, "coverage.unrepresented-scripts").is_empty(),
        "a represented harness declares its tools, got: {findings:#?}"
    );
}
