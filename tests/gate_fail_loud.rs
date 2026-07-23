//! Fail-loud on a mis-rooted or malformed harness. Every path argument naming one
//! harness — its root or its `.temper` workspace — resolves whole: the lock is read
//! from the same harness the corpus is walked from, so neither spelling can half-gate
//! into a silent green or a false unfilled requirement. What remains fail-loud here:
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
    (run.findings(), run.ok)
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
fn a_workspace_dir_argument_resolves_the_enclosing_roots_corpus() {
    // The mirror half: the workspace directory itself as the path argument. `.temper`
    // carries the lock at its top, so it resolves whole — gated against the harness root
    // enclosing it, the corpus its declarations were written about. Rooting it at itself
    // would read the lock from `.temper` while walking `.temper` for members that live
    // beside it, so the filled requirement below would false-fire `requirement.unfilled`
    // and the member count would collapse to whatever prose sits inside the workspace.
    let root = common::tmpdir("workspace-dir-arg");
    common::write_skill(&root, "coordinate", CLEAN_SKILL);
    common::write_requirements(&root, vec![common::requirement("docs", true, None)]);
    common::author_satisfies(&root, "skills", "coordinate", &["docs"]);

    let (from_root, root_ok) = check_in(&root, &["."]);
    let (from_workspace, workspace_ok) = check_in(&root, &[".temper"]);

    assert!(
        common::findings_for(&from_workspace, "requirement.unfilled").is_empty(),
        "the skill fills `docs`, so no unfilled requirement may fire, got: {from_workspace:#?}"
    );
    // Every spelling of one harness earns one verdict: same members discovered, same
    // requirements judged.
    assert_eq!(
        from_workspace, from_root,
        "`check .temper` must report exactly what `check .` reports"
    );
    assert_eq!(
        workspace_ok, root_ok,
        "`check .temper` must reach the same exit verdict as `check .`"
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
    // parse used to abort with no findings; now it surfaces as an error Diagnostic naming
    // the file. Invariant 6 wants the malformation surfaced loud — an error naming the
    // file and the fault, continuing to check other members in the harness.
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
    // The fault is now collected as an error Diagnostic naming the file.
    let findings = run.findings();
    let fault_findings: Vec<_> = common::findings_for(&findings, "member.load-fault");
    assert_eq!(
        fault_findings.len(),
        1,
        "a malformed block must emit a load-fault finding, got:\n{findings:#?}"
    );
    assert!(
        fault_findings[0].contains("SKILL.md"),
        "the finding must name the file, got: {}",
        fault_findings[0]
    );
}

#[test]
fn a_harness_with_one_malformed_member_still_checks_clean_members() {
    // When a harness carries a malformed member plus N clean members, the malformed
    // member's load fault is collected as a diagnostic, and the check continues over
    // the remaining members instead of aborting. This validates the 4/9-harness scenario
    // where a single malformed member should not prevent the others from being checked.
    let root = common::tmpdir("malformed-with-clean-members");

    // Add a clean skill.
    common::write_skill(&root, "coordinate", CLEAN_SKILL);

    // Add a malformed skill.
    let malformed = "---\n\
        this is a bare scalar, not a mapping\n\
        ---\n\
        # Broken\n\
        \n\
        Body.\n";
    common::write_skill(&root, "broken", malformed);

    let (findings, success) = check_in(&root, &["."]);

    // The run fails due to the malformed member.
    assert!(
        !success,
        "a harness with a malformed member must exit non-zero, got: {findings:#?}"
    );

    // The malformed member produces a load-fault finding.
    let fault_findings = common::findings_for(&findings, "member.load-fault");
    assert_eq!(
        fault_findings.len(),
        1,
        "exactly one load-fault finding for the malformed member, got: {findings:#?}"
    );

    // The clean member is still discovered and contributes to the member count.
    let coverage_findings = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        coverage_findings.len(),
        1,
        "coverage finding should report member counts, got: {findings:#?}"
    );
    // Should report "checked 1 members ... skill (1)" indicating the clean skill was found.
    assert!(
        coverage_findings[0].contains("skill (1)"),
        "the clean skill must still be discovered, got: {}",
        coverage_findings[0]
    );
}

#[test]
fn a_kind_contract_naming_a_judge_less_predicate_fails_loud() {
    // `dependency-exists` names no decidable reference syntax or extractor, so no
    // projection carries the fact it would range over: no selection makes it decidable,
    // and it is the one predicate left that no judge reaches. Declared on a kind, the run
    // must refuse loud naming the predicate rather than let the member pass a check that
    // never ran. The refusal lands at *load* — the lock carries no decodable argument
    // shape for it — one tier above the admissibility fence that would catch it next.
    let root = common::tmpdir("judgeless-kind-clause");
    common::write_skill(&root, "coordinate", CLEAN_SKILL);
    common::write_lock(
        &root,
        temper::drift::Declarations {
            clauses: vec![temper::drift::ClauseRow {
                unit: None,
                label: None,
                kind: Some("skill".to_string()),
                ..common::clause("dependency-exists", "required")
            }],
            ..temper::drift::Declarations::default()
        },
    );

    let run = common::check_in(&root, &["."], None);

    assert!(
        !run.ok,
        "a contract naming a predicate no judge decides must exit non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("dependency-exists"),
        "the refusal names the predicate it could not admit, got:\n{}",
        run.output
    );
}

#[test]
fn a_count_floor_bound_to_a_kinds_own_selection_is_judged_not_fenced() {
    // The other half of the inversion: `count` binds to the kind's *whole population* —
    // the universal selection — which is as decidable as a requirement's opt-in one. The
    // floor is judged rather than fenced, so a single skill against a `[2, 3]` band is a
    // real out-of-band finding naming the kind, not an admissibility refusal.
    let root = common::tmpdir("kind-count-floor");
    common::write_skill(&root, "coordinate", CLEAN_SKILL);
    common::write_lock(
        &root,
        temper::drift::Declarations {
            clauses: vec![temper::drift::ClauseRow {
                unit: None,
                label: None,
                kind: Some("skill".to_string()),
                count: Some(temper::drift::CountBoundRow { min: 2, max: 3 }),
                ..common::clause("count", "required")
            }],
            ..temper::drift::Declarations::default()
        },
    );

    let (findings, success) = check_in(&root, &["."]);

    let counted = common::findings_for(&findings, "skill.count");
    assert_eq!(
        counted.len(),
        1,
        "the by-kind selection holds one member, outside [2, 3], got: {findings:#?}"
    );
    assert!(
        counted[0].contains("kind `skill`") && counted[0].contains("[2, 3]"),
        "the finding names the selection it judged and the bound it missed, got: {}",
        counted[0]
    );
    assert!(!success, "a required clause's violation fails the run");
}

/// A harness whose `standard` kind templates an embedded `citation` child, with one
/// citation value whose format placed `placed_edges`. The `citation` kind declares a
/// `source` edge and carries a `format-places-edges` clause, so the value's placement is
/// the only thing that varies between the two cases below.
fn write_embedded_citation_harness(
    root: &Path,
    placed_edges: Vec<String>,
    extra_clauses: Vec<temper::drift::ClauseRow>,
) {
    fs::create_dir_all(root.join("docs").join("standards")).unwrap();
    fs::write(
        root.join("docs").join("standards").join("the-charter.md"),
        "# The charter\n\nThe standard the citation points back at.\n",
    )
    .unwrap();
    common::write_lock(
        root,
        temper::drift::Declarations {
            kinds: vec![temper::drift::KindFactRow {
                templates: vec![temper::drift::TemplateRow {
                    kind: "citation".to_string(),
                    path: None,
                }],
                ..common::kind_facts("standard", "docs/standards", "*.md")
            }],
            assembly: vec![temper::drift::AssemblyFactRow {
                fact: "edge".to_string(),
                value: None,
                from: Some("citation".to_string()),
                field: Some("source".to_string()),
                to: Some(vec!["standard".to_string()]),
            }],
            nested_members: vec![temper::drift::NestedMemberRow {
                host: "standard:the-charter".to_string(),
                kind: "citation".to_string(),
                key: "the-standard".to_string(),
                leaves: std::collections::BTreeMap::from([(
                    "source".to_string(),
                    "the-charter".to_string(),
                )]),
                collections: Vec::new(),
                placed_edges: Some(placed_edges),
                rendered_lines: None,
                rendered_chars: None,
            }],
            clauses: std::iter::once(temper::drift::ClauseRow {
                unit: None,
                label: None,
                kind: Some("citation".to_string()),
                ..common::clause("format-places-edges", "required")
            })
            .chain(extra_clauses)
            .collect(),
            ..temper::drift::Declarations::default()
        },
    );
}

#[test]
fn a_clause_bound_to_an_embedded_kind_judges_its_members() {
    // An embedded kind reaches the lock solely through its host's `templates` column, so
    // it has no kind-fact row and neither at-locus dispatcher keys off it. Its members
    // must still reach conformance: a `format-places-edges` clause over a citation whose
    // format dropped the `source` edge it carries fails the run loud, naming the field.
    // Bound to a kind nobody judged, the clause would silently decide nothing.
    let root = common::tmpdir("embedded-kind-clause-fires");
    write_embedded_citation_harness(&root, Vec::new(), Vec::new());

    let (findings, success) = check_in(&root, &["."]);

    let omissions = common::findings_for(&findings, "citation.format-places-edges");
    assert_eq!(
        omissions.len(),
        1,
        "the embedded kind's clause must fire over its member, got: {findings:#?}"
    );
    assert!(
        omissions[0].starts_with("::error "),
        "a `required` clause's violation is error-severity (fails the run), got: {}",
        omissions[0]
    );
    assert!(
        omissions[0].contains("source"),
        "the finding must name the omitted edge field, got: {}",
        omissions[0]
    );
    assert!(
        !success,
        "an embedded kind's error-severity finding must exit non-zero, got: {findings:#?}"
    );
}

#[test]
fn an_embedded_kind_clause_that_holds_leaves_the_run_silent() {
    // The same harness with the `source` edge placed: the dispatcher adds a judge, never
    // a finding. A clause reaching a kind it never reached before must not invent one.
    let root = common::tmpdir("embedded-kind-clause-holds");
    write_embedded_citation_harness(&root, vec!["source".to_string()], Vec::new());

    let (findings, success) = check_in(&root, &["."]);

    assert!(
        common::findings_for(&findings, "citation.format-places-edges").is_empty(),
        "the format placed every edge the value carries, got: {findings:#?}"
    );
    assert!(
        success,
        "a holding embedded-kind clause must exit zero, got: {findings:#?}"
    );
}

#[test]
fn a_body_shaped_clause_bound_to_an_embedded_kind_fails_admissibility() {
    // An embedded member owns no document: the lift reads it off its host's declared
    // surface and its headings/sections/source-directory arrive empty. A `name-matches-dir`
    // bound to the kind therefore has no directory to compare and passes whatever the value
    // holds — a clause that decides nothing while reading as a check that ran. Admissibility
    // is where that dies, naming the predicate and the kind, rather than conformance
    // inventing a body to judge. (`extent` is no longer among these: a composed member's
    // rendered span is captured at emit, so its `extent` reads real data.)
    let root = common::tmpdir("embedded-kind-body-clause");
    write_embedded_citation_harness(
        &root,
        vec!["source".to_string()],
        vec![temper::drift::ClauseRow {
            kind: Some("citation".to_string()),
            ..common::clause("name-matches-dir", "required")
        }],
    );

    let (findings, success) = check_in(&root, &["."]);

    let fenced = common::findings_for(&findings, "citation.name-matches-dir");
    assert_eq!(
        fenced.len(),
        1,
        "a body-shaped clause an embedded kind can never decide must fail admissibility, got: {findings:#?}"
    );
    assert!(
        fenced[0].starts_with("::error "),
        "admissibility carries no advisory severity, got: {}",
        fenced[0]
    );
    assert!(
        fenced[0].contains("citation"),
        "the refusal names the kind the clause is bound to, got: {}",
        fenced[0]
    );
    assert!(
        !success,
        "an inadmissible contract must exit non-zero, got: {findings:#?}"
    );
}

#[test]
fn the_body_fence_leaves_an_embedded_kinds_decidable_clauses_judging() {
    // The fence's line is the feature read, not the kind. With an inadmissible
    // `name-matches-dir` declared, the citation's decidable clauses must still reach their
    // members: `format-places-edges` fires over the dropped `source` edge exactly as it
    // does without it. A fence that swallowed the dispatch would trade one silence for
    // another.
    let root = common::tmpdir("embedded-kind-fence-keeps-dispatch");
    write_embedded_citation_harness(
        &root,
        Vec::new(),
        vec![temper::drift::ClauseRow {
            kind: Some("citation".to_string()),
            ..common::clause("name-matches-dir", "required")
        }],
    );

    let (findings, success) = check_in(&root, &["."]);

    let omissions = common::findings_for(&findings, "citation.format-places-edges");
    assert_eq!(
        omissions.len(),
        1,
        "the embedded kind's decidable clause must still judge its member, got: {findings:#?}"
    );
    assert!(
        omissions[0].contains("source"),
        "the finding must name the omitted edge field, got: {}",
        omissions[0]
    );
    assert!(
        !success,
        "both the fence and the finding fail the run, got: {findings:#?}"
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
