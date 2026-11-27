//! A `when` guard over the member's load moment: its body binds on a member the runtime
//! loads on demand and is silent on one it loads at launch. The moment is read off the
//! member's registration — a subdirectory's `CLAUDE.md` registers on a read beneath its
//! directory, a rule declaring `paths` on a read of a match — never off a field.

use std::path::Path;

use temper::drift::{AbsenceGateRow, BoundRow, ClauseRow, Declarations, KindFactRow};

use crate::common::{self, check_harness, write_sibling};

/// A body three lines long — past the one-line budget the guarded body declares.
const THREE_LINES: &str = "# Notes\n\nKeep it short.\n";

/// A lock whose `kind` contract is one `when(loads(moment), [extent(lines, 1)])`.
fn guard_lock(root: &Path, kind: &str, moment: &str) {
    common::write_lock(
        root,
        Declarations {
            clauses: vec![ClauseRow {
                kind: Some(kind.to_string()),
                values: Some(vec![moment.to_string()]),
                guard_predicate: Some("loads".to_string()),
                body: Some(vec![ClauseRow {
                    bound: Some(BoundRow {
                        min: None,
                        max: Some(1),
                        entry: None,
                    }),
                    unit: Some("lines".to_string()),
                    ..common::clause("extent", "advisory")
                }]),
                ..common::clause("when", "advisory")
            }],
            ..Declarations::default()
        },
    );
}

/// A root and a subdirectory `CLAUDE.md`, both past the guarded body's budget.
fn memory_harness(label: &str, moment: &str) -> Vec<String> {
    let harness = common::tmpdir(label);
    write_sibling(&harness, "CLAUDE.md", THREE_LINES);
    write_sibling(&harness, "apps/web/CLAUDE.md", THREE_LINES);
    guard_lock(&harness, "memory", moment);
    let (findings, _) = check_harness(&harness);
    findings
}

fn guarded(findings: &[String], moment: &str) -> Vec<String> {
    common::findings_for(findings, &format!("memory.when.loads={moment}.extent"))
        .into_iter()
        .cloned()
        .collect()
}

#[test]
fn a_load_moment_guard_binds_its_body_on_a_member_that_loads_on_demand() {
    let findings = memory_harness("load-moment-on-demand", "on-demand");
    let fired = guarded(&findings, "on-demand");
    assert_eq!(
        fired.len(),
        1,
        "one finding, the subdirectory's: {findings:#?}"
    );
    assert!(
        fired[0].contains("::apps:web:CLAUDE:"),
        "the on-demand member is the subdirectory's `CLAUDE.md`: {findings:#?}"
    );
}

#[test]
fn a_load_moment_guard_is_silent_on_a_member_that_loads_at_launch() {
    let findings = memory_harness("load-moment-launch-silent", "on-demand");
    assert!(
        guarded(&findings, "on-demand")
            .iter()
            .all(|finding| finding.contains("::apps:web:CLAUDE:")),
        "the root `CLAUDE.md` loads at launch, so the guard is silent on it: {findings:#?}"
    );

    // The converse guard binds the launch-loaded member and nothing else.
    let findings = memory_harness("load-moment-launch-binds", "launch");
    let fired = guarded(&findings, "launch");
    assert_eq!(fired.len(), 1, "one finding, the root's: {findings:#?}");
    assert!(
        !fired[0].contains("::apps:web:CLAUDE:"),
        "the launch-loaded member is the root `CLAUDE.md`: {findings:#?}"
    );
}

#[test]
fn a_rule_declaring_paths_loads_on_demand_and_one_without_at_launch() {
    let harness = common::tmpdir("load-moment-rule-paths");
    write_sibling(
        &harness,
        ".claude/rules/scoped.md",
        &format!("---\npaths: \"src/**\"\n---\n{THREE_LINES}"),
    );
    write_sibling(&harness, ".claude/rules/always.md", THREE_LINES);
    write_sibling(&harness, "src/lib.rs", "");
    guard_lock(&harness, "rule", "on-demand");
    let (findings, _) = check_harness(&harness);
    let fired: Vec<&String> = common::findings_for(&findings, "rule.when.loads=on-demand.extent");
    assert_eq!(fired.len(), 1, "the scoped rule alone: {findings:#?}");
    assert!(fired[0].contains("scoped"), "{findings:#?}");
}

#[test]
fn a_load_moment_guard_as_a_clause_of_its_own_is_inadmissible() {
    let harness = common::tmpdir("load-moment-bare");
    write_sibling(&harness, "CLAUDE.md", THREE_LINES);
    common::write_lock(
        &harness,
        Declarations {
            clauses: vec![ClauseRow {
                kind: Some("memory".to_string()),
                values: Some(vec!["on-demand".to_string()]),
                ..common::clause("loads", "advisory")
            }],
            ..Declarations::default()
        },
    );
    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a bare `loads` clause decides nothing: {findings:#?}");
    assert!(
        findings
            .iter()
            .any(|finding| finding.contains("`when` guard form only")),
        "{findings:#?}"
    );
}

#[test]
fn a_loads_on_demand_guard_binds_over_a_custom_kind_registering_always_behind_an_on_demand_absence_gate()
 {
    let harness = common::tmpdir("load-moment-custom-always-gated");
    write_sibling(&harness, "NOTES.md", THREE_LINES);
    write_sibling(&harness, "apps/web/NOTES.md", THREE_LINES);
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![KindFactRow {
                unit_shape: Some("file".to_string()),
                registration: vec!["always".to_string()],
                absence_gate: vec![AbsenceGateRow {
                    template: "**/NOTES.md".to_string(),
                    absent: vec!["SKIP.md".to_string()],
                }],
                ..common::kind_facts("notes", ".", "**/NOTES.md")
            }],
            clauses: vec![ClauseRow {
                kind: Some("notes".to_string()),
                values: Some(vec!["on-demand".to_string()]),
                guard_predicate: Some("loads".to_string()),
                body: Some(vec![ClauseRow {
                    bound: Some(BoundRow {
                        min: None,
                        max: Some(1),
                        entry: None,
                    }),
                    unit: Some("lines".to_string()),
                    ..common::clause("extent", "advisory")
                }]),
                ..common::clause("when", "advisory")
            }],
            ..Declarations::default()
        },
    );
    let (findings, _) = check_harness(&harness);
    let fired = common::findings_for(&findings, "notes.when.loads=on-demand.extent");
    assert_eq!(
        fired.len(),
        1,
        "the subdirectory's member alone: {findings:#?}"
    );
    let subdirectory = fired[0];
    assert!(subdirectory.contains("apps:web"), "{findings:#?}");

    // `explain` states the same moment the guard judged, for both members.
    let subdirectory = common::explain_in(&harness, "apps:web:NOTES");
    assert!(
        subdirectory.contains("`always`: loaded on demand"),
        "{subdirectory}"
    );
    let root = common::explain_in(&harness, "NOTES");
    assert!(root.contains("`always`: loaded at launch"), "{root}");
}
