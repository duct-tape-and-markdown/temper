//! A subkind — a kind row naming a `base` — keeps its base's facts and governs a locus
//! within the base's: a document there is the subkind's one member, never a governs
//! collision, and a locus outside the base's refuses when the lock loads.

use std::fs;
use std::path::Path;

use temper::drift::{ClauseRow, CountBoundRow, Declarations, KindFactRow, SatisfiesRow};

use crate::common;

fn write_rule(root: &Path, relative: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    // `globs` is a Cursor key the `rule` default contract forbids, so a `rule` member
    // carrying it is named by `rule.forbidden_keys` and a subkind member is not.
    fs::write(path, "---\nglobs: \"*.rs\"\n---\n# Doc\n\nBody.\n").unwrap();
}

fn decision(root: &str, glob: &str) -> KindFactRow {
    KindFactRow {
        base: Some("rule".to_string()),
        ..common::kind_facts("decision", root, glob)
    }
}

fn check_with(root: &Path, kinds: Vec<KindFactRow>) -> common::CheckRun {
    check_with_clauses(root, kinds, Vec::new())
}

fn check_with_clauses(
    root: &Path,
    kinds: Vec<KindFactRow>,
    clauses: Vec<ClauseRow>,
) -> common::CheckRun {
    common::write_lock(
        root,
        Declarations {
            kinds,
            clauses,
            ..Declarations::default()
        },
    );
    common::check_in(root, &[], Some("github"))
}

/// The finding lines at member `member` under clause address `label`.
fn findings_at(run: &common::CheckRun, label: &str, member: &str) -> usize {
    run.output
        .lines()
        .filter(|line| line.contains(&format!("title={label}::{member}:")))
        .count()
}

fn decision_requires_status() -> ClauseRow {
    ClauseRow {
        label: Some("decision.required.status".to_string()),
        kind: Some("decision".to_string()),
        field: Some("status".to_string()),
        ..common::clause("required", "required")
    }
}

#[test]
fn a_document_in_a_subkinds_locus_is_one_member_of_the_subkind() {
    let root = common::tmpdir("subkind-member");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    write_rule(&root, ".claude/rules/style.md");
    let run = check_with(&root, vec![decision(".claude/rules/decisions", "*.md")]);
    assert!(
        run.output.contains("rule.forbidden_keys") && run.output.contains("style"),
        "the base still owns its own documents, got:\n{}",
        run.output
    );
    let narration = common::explain_in(&root, "decision:0001-adopt");
    assert!(
        narration.contains("decision") && !narration.contains("rule:0001-adopt"),
        "the document is the subkind's member, got:\n{narration}"
    );
}

#[test]
fn a_document_in_a_subkinds_locus_raises_no_governs_collision_with_its_base() {
    let root = common::tmpdir("subkind-no-collision");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    for kinds in [
        vec![decision(".claude/rules/decisions", "*.md")],
        vec![decision(".claude/rules", "**/*.md")],
    ] {
        let run = check_with(&root, kinds);
        assert!(
            !run.output.contains("kind.governs-collision"),
            "a subkind and its base never collide, got:\n{}",
            run.output
        );
    }
}

#[test]
fn a_subkind_whose_locus_lies_outside_its_bases_refuses_at_load() {
    let root = common::tmpdir("subkind-outside");
    write_rule(&root, "docs/decisions/0001-adopt.md");
    let run = check_with(&root, vec![decision("docs/decisions", "*.md")]);
    assert!(
        !run.ok,
        "a subkind outside its base refuses, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("`decision`")
            && run.output.contains("`rule`")
            && run.output.contains("outside the base's"),
        "the refusal names the subkind, its base, and the locus fault, got:\n{}",
        run.output
    );
}

#[test]
fn a_subkind_naming_no_declared_base_refuses_at_load() {
    let root = common::tmpdir("subkind-unknown-base");
    let run = check_with(
        &root,
        vec![KindFactRow {
            base: Some("nonesuch".to_string()),
            ..common::kind_facts("decision", ".claude/rules/decisions", "*.md")
        }],
    );
    assert!(!run.ok, "an unknown base refuses, got:\n{}", run.output);
    assert!(
        run.output.contains("no kind named `nonesuch`"),
        "the refusal names the missing base, got:\n{}",
        run.output
    );
}

#[test]
fn explain_narrates_a_subkinds_base() {
    let root = common::tmpdir("subkind-explain");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    check_with(&root, vec![decision(".claude/rules/decisions", "*.md")]);
    let narration = common::explain_in(&root, "decision");
    assert!(
        narration.contains("`decision` is a subkind of `rule`"),
        "the kind narration names its base, got:\n{narration}"
    );
}

#[test]
fn a_clause_bound_to_the_base_kind_judges_a_subkinds_member() {
    let root = common::tmpdir("subkind-base-clause");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    write_rule(&root, ".claude/rules/style.md");
    let run = check_with_clauses(
        &root,
        vec![decision(".claude/rules/decisions", "*.md")],
        vec![decision_requires_status()],
    );
    assert_eq!(
        findings_at(&run, "rule.forbidden_keys", "0001-adopt"),
        1,
        "the base's clause fires once at the subkind member's address, got:\n{}",
        run.output
    );
    assert_eq!(
        findings_at(&run, "decision.required.status", "0001-adopt"),
        1,
        "the subkind's own clause fires beside it, got:\n{}",
        run.output
    );
    assert_eq!(
        findings_at(&run, "rule.forbidden_keys", "style"),
        1,
        "the base's own member is judged as before, got:\n{}",
        run.output
    );
}

#[test]
fn a_subkinds_own_clause_binds_only_the_subkinds_members() {
    let root = common::tmpdir("subkind-own-clause");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    write_rule(&root, ".claude/rules/style.md");
    let run = check_with_clauses(
        &root,
        vec![decision(".claude/rules/decisions", "*.md")],
        vec![decision_requires_status()],
    );
    assert_eq!(
        findings_at(&run, "decision.required.status", "style"),
        0,
        "a base member is never held to its subkind's clause, got:\n{}",
        run.output
    );
    assert_eq!(
        findings_at(&run, "decision.required.status", "0001-adopt"),
        1,
        "the subkind member is, got:\n{}",
        run.output
    );
}

#[test]
fn a_by_kind_selection_of_the_base_counts_its_subkinds_members() {
    let root = common::tmpdir("subkind-base-count");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    write_rule(&root, ".claude/rules/style.md");
    let run = check_with_clauses(
        &root,
        vec![decision(".claude/rules/decisions", "*.md")],
        vec![ClauseRow {
            label: Some("rule.count".to_string()),
            kind: Some("rule".to_string()),
            count: Some(CountBoundRow { min: 0, max: 1 }),
            ..common::clause("count", "required")
        }],
    );
    assert!(
        run.output.contains("title=rule.count"),
        "the base's selection holds both members, so its count of one is exceeded, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_narrowing_to_a_base_admits_its_subkinds_members() {
    let root = common::tmpdir("subkind-narrowing");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![decision(".claude/rules/decisions", "*.md")],
            requirements: vec![common::requirement("review-coverage", true, Some("rule"))],
            satisfies: vec![SatisfiesRow {
                member: "decision:0001-adopt".to_string(),
                requirement: "review-coverage".to_string(),
            }],
            ..Declarations::default()
        },
    );
    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        !run.output.contains("title=requirement.review-coverage"),
        "a decision is a rule, so it satisfies the rule-narrowed requirement, got:\n{}",
        run.output
    );
}

#[test]
fn a_bases_unique_name_judges_its_subkinds_members_with_its_own() {
    let root = common::tmpdir("subkind-unique-name");
    write_rule(&root, ".claude/rules/decisions/style.md");
    write_rule(&root, ".claude/rules/style.md");
    let run = check_with_clauses(
        &root,
        vec![decision(".claude/rules/decisions", "*.md")],
        vec![ClauseRow {
            label: Some("rule.unique-name".to_string()),
            kind: Some("rule".to_string()),
            ..common::clause("unique-name", "required")
        }],
    );
    assert_eq!(
        run.output.matches("title=rule.unique-name::style:").count(),
        2,
        "the rule and the decision share a name in the base's one population, got:\n{}",
        run.output
    );
}

fn rule_requires_owner() -> ClauseRow {
    ClauseRow {
        label: Some("rule.required.owner".to_string()),
        kind: Some("rule".to_string()),
        field: Some("owner".to_string()),
        ..common::clause("required", "required")
    }
}

#[test]
fn schema_for_a_subkind_includes_its_bases_clauses() {
    let root = common::tmpdir("subkind-schema");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![decision(".claude/rules/decisions", "*.md")],
            clauses: vec![rule_requires_owner(), decision_requires_status()],
            ..Declarations::default()
        },
    );
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_temper"))
        .current_dir(&root)
        .args(["schema", "--kind", "decision"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "schema --kind decision must exit zero, got:\n{stdout}{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let required: Vec<&str> = json["required"]
        .as_array()
        .unwrap_or_else(|| panic!("a `required` list, got:\n{stdout}"))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect();
    assert!(
        required.contains(&"owner") && required.contains(&"status"),
        "the subkind's schema carries its base's required field beside its own, got:\n{stdout}"
    );
}

#[test]
fn explain_narrates_a_subkinds_inherited_clauses() {
    let root = common::tmpdir("subkind-explain-clauses");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    check_with_clauses(
        &root,
        vec![decision(".claude/rules/decisions", "*.md")],
        vec![rule_requires_owner(), decision_requires_status()],
    );
    let narration = common::explain_in(&root, "decision:0001-adopt");
    assert!(
        narration.contains("rule.required.owner") && narration.contains("decision.required.status"),
        "the subkind member's narration names its base's clause beside its own, got:\n{narration}"
    );
}
