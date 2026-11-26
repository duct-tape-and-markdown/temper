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

#[test]
fn a_when_nested_unique_name_finds_a_base_member_and_a_subkind_member_sharing_a_name() {
    let root = common::tmpdir("subkind-when-unique-name");
    write_rule(&root, ".claude/rules/decisions/style.md");
    write_rule(&root, ".claude/rules/style.md");
    // Both members carry `globs: "*.rs"`, so the guard holds at each of them.
    let run = check_with_clauses(
        &root,
        vec![decision(".claude/rules/decisions", "*.md")],
        vec![ClauseRow {
            label: Some("rule.when.globs".to_string()),
            kind: Some("rule".to_string()),
            field: Some("globs".to_string()),
            values: Some(vec!["*.rs".to_string()]),
            guard_predicate: Some("enum".to_string()),
            body: Some(vec![ClauseRow {
                label: Some("rule.when.globs.unique-name".to_string()),
                ..common::clause("unique-name", "required")
            }]),
            ..common::clause("when", "required")
        }],
    );
    assert_eq!(
        run.output.matches(".unique-name::style:").count(),
        2,
        "a guarded unique-name decides over the base's one population, got:\n{}",
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

/// The guard binds the one effective kind set `check` reads: a pending write inside a
/// subkind's locus is the subkind's undeclared member alone, the base ceding the path,
/// while a write in the rest of the base's locus is still the base's.
#[test]
fn the_guard_judges_a_write_in_a_subkinds_locus_as_the_subkind_alone() {
    let root = common::tmpdir("guard-subkind-locus");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![decision(".claude/rules/decisions", "*.md")],
            clauses: vec![common::clause("locus-declared", "required")],
            ..Declarations::default()
        },
    );

    let (code, output) = common::run_guard(
        &root,
        &common::guard_write_payload(".claude/rules/decisions/0001.md"),
    );
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");
    assert!(
        in_band.contains("`decision`") && !in_band.contains("`rule`"),
        "a write in the subkind's locus is the subkind's member alone, got: {in_band}"
    );

    let (code, output) = common::run_guard(
        &root,
        &common::guard_write_payload(".claude/rules/style.md"),
    );
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");
    assert!(
        in_band.contains("`rule`") && !in_band.contains("`decision`"),
        "a write elsewhere in the base's locus stays the base's, got: {in_band}"
    );
}

/// A base member placed at a path inside a subkind's locus would read back as the
/// subkind's document, so emit refuses it by name; a base member elsewhere in the base's
/// locus still places.
#[test]
fn emit_refuses_a_base_member_whose_projection_path_lies_in_a_subkinds_locus() {
    use temper::drift::{self, EmitOptions, Payload};
    let root = common::tmpdir("emit-subkind-locus");
    let rule = KindFactRow {
        governs_glob: Some("**/*.md".to_string()),
        ..common::rule_kind_facts(None, &[])
    };
    let standard = KindFactRow {
        base: Some("rule".to_string()),
        ..common::kind_facts("standard", ".claude/rules/standards", "*.md")
    };
    let payload = |name: &str| Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![rule.clone(), standard.clone()],
            ..Declarations::default()
        },
        members: vec![common::rule_member(name, None, "# Doc\n\nBody.\n")],
    };

    let err = drift::emit(
        &payload("standards:x"),
        &root.join(".temper"),
        EmitOptions::default(),
    )
    .expect_err("a base member in a subkind's locus must refuse");
    let message = err.to_string();
    assert!(
        message.contains("`standard`")
            && message.contains("rule:standards:x")
            && message.contains(".claude/rules/standards/x.md"),
        "the refusal names the subkind, the member and the path, got: {message}"
    );
    assert!(
        !root.join(".claude/rules/standards/x.md").exists(),
        "refused before a byte is written"
    );

    drift::emit(
        &payload("guides:x"),
        &root.join(".temper"),
        EmitOptions::default(),
    )
    .expect("placement outside any subkind locus is unchanged");
    assert!(root.join(".claude/rules/guides/x.md").is_file());
}

fn write_skill(root: &Path, dir: &str, docs: &[&str]) {
    let unit = root.join(".claude/skills").join(dir);
    fs::create_dir_all(&unit).unwrap();
    fs::write(
        unit.join("SKILL.md"),
        format!(
            "---\nname: {dir}\ndescription: Use when {dir} is asked for.\n---\n# {dir}\n\nBody.\n"
        ),
    )
    .unwrap();
    for doc in docs {
        fs::write(unit.join(format!("{doc}.md")), "# Doc\n\nBody.\n").unwrap();
    }
}

/// `procedure` extends `skill` over the `proc-*` skills and templates `topic` — a subkind
/// of the nested `supporting-doc`, governing no locus — as its file child.
fn procedure_and_topic() -> Vec<KindFactRow> {
    vec![
        KindFactRow {
            base: Some("skill".to_string()),
            templates: vec![temper::drift::TemplateRow {
                kind: "topic".to_string(),
                path: Some("*.md".to_string()),
            }],
            ..common::kind_facts("procedure", ".", "**/.claude/skills/proc-*/SKILL.md")
        },
        KindFactRow {
            base: Some("supporting-doc".to_string()),
            governs_root: None,
            governs_glob: None,
            ..common::kind_facts("topic", "", "")
        },
    ]
}

#[test]
fn a_nested_subkind_named_by_an_extended_hosts_template_loads() {
    let root = common::tmpdir("nested-subkind-loads");
    write_skill(&root, "proc-deploy", &["steps"]);
    let run = check_with(&root, procedure_and_topic());
    assert!(
        !run.output.contains("subkind") && !run.output.contains("governs no file locus"),
        "a locus-less subkind of a nested base binds, got:\n{}",
        run.output
    );
    let narration = common::explain_in(&root, "topic");
    assert!(
        narration.contains("`topic` is a subkind of `supporting-doc`"),
        "the nested subkind loads with its base, got:\n{narration}"
    );
}

#[test]
fn a_file_child_under_an_extended_hosts_unit_is_a_member_of_the_subkind_its_template_names() {
    let root = common::tmpdir("nested-subkind-member");
    write_skill(&root, "proc-deploy", &["steps"]);
    write_skill(&root, "review", &["checklist"]);
    check_with(&root, procedure_and_topic());
    let topic = common::explain_in(&root, "procedure:proc-deploy/topic/steps");
    assert!(
        topic.contains("Member `procedure:proc-deploy/topic/steps` (topic)"),
        "the procedure's child is a `topic` member, got:\n{topic}"
    );
    let doc = common::explain_in(&root, "skill:review/supporting-doc/checklist");
    assert!(
        doc.contains("Member `skill:review/supporting-doc/checklist` (supporting-doc)"),
        "an ordinary skill's child is still a supporting doc, got:\n{doc}"
    );
}

#[test]
fn a_clause_bound_to_the_nested_base_judges_the_nested_subkinds_member() {
    let root = common::tmpdir("nested-subkind-base-clause");
    write_skill(&root, "proc-deploy", &["steps"]);
    let run = check_with(&root, procedure_and_topic());
    assert!(
        run.output
            .lines()
            .any(|line| line.contains("supporting-doc.") && line.contains("steps")),
        "supporting-doc's own clause judges the unreferenced topic member, got:\n{}",
        run.output
    );
}

/// `strict-charter` extends the layout kind `charter` over `specs/strict-*.md` and names
/// `strict-invariant` — a subkind of the embedded `invariant`, which no row declares — as
/// its collection's member kind in place of its base's. Both hosts are `local`, so `check`
/// reads their documents in place.
fn charter_and_strict_invariant() -> Vec<KindFactRow> {
    let layout = |member_kind: &str| temper::drift::LayoutRow {
        regions: vec![temper::drift::LayoutRegionRow {
            region: "collection".to_string(),
            import: None,
            slot: None,
            member_kind: Some(member_kind.to_string()),
            key: None,
        }],
    };
    vec![
        KindFactRow {
            content: Some(layout("invariant")),
            commitment: Some("local".to_string()),
            ..common::kind_facts("charter", "specs", "*.md")
        },
        KindFactRow {
            base: Some("charter".to_string()),
            content: Some(layout("strict-invariant")),
            commitment: Some("local".to_string()),
            ..common::kind_facts("strict-charter", "specs", "strict-*.md")
        },
        KindFactRow {
            base: Some("invariant".to_string()),
            governs_root: None,
            governs_glob: None,
            ..common::kind_facts("strict-invariant", "", "")
        },
    ]
}

fn write_charter(root: &Path, name: &str) {
    let path = root.join("specs").join(format!("{name}.md"));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        "# Charter\n\n## Invariants\n\n### Loud\nNever silent.\n",
    )
    .unwrap();
}

fn strict_invariant_requires_test() -> ClauseRow {
    ClauseRow {
        label: Some("strict-invariant.required.test".to_string()),
        kind: Some("strict-invariant".to_string()),
        field: Some("test".to_string()),
        ..common::clause("required", "required")
    }
}

#[test]
fn an_embedded_member_under_an_extended_host_is_checked_against_the_subkinds_fields() {
    let root = common::tmpdir("embedded-subkind-member");
    write_charter(&root, "strict-core");
    write_charter(&root, "plain");
    let run = check_with_clauses(
        &root,
        charter_and_strict_invariant(),
        vec![strict_invariant_requires_test()],
    );
    let flagged: Vec<&str> = run
        .output
        .lines()
        .filter(|line| line.contains("strict-invariant.required.test"))
        .collect();
    assert!(
        flagged.len() == 1 && flagged[0].contains("strict-core"),
        "the subkind's clause judges the extended host's member alone, got:\n{}",
        run.output
    );
}

#[test]
fn a_clause_bound_to_the_embedded_base_judges_the_embedded_subkinds_member() {
    let root = common::tmpdir("embedded-subkind-base-clause");
    write_charter(&root, "strict-core");
    write_charter(&root, "plain");
    let run = check_with_clauses(
        &root,
        charter_and_strict_invariant(),
        vec![ClauseRow {
            label: Some("invariant.required.test".to_string()),
            kind: Some("invariant".to_string()),
            ..strict_invariant_requires_test()
        }],
    );
    let flagged = run
        .output
        .lines()
        .filter(|line| line.contains("invariant.required.test"))
        .count();
    assert_eq!(
        flagged, 2,
        "the base's clause judges its own member and the subkind's, got:\n{}",
        run.output
    );
}
