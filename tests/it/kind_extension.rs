//! A subkind — a kind row naming a `base` — keeps its base's facts and governs a locus
//! within the base's: a document there is the subkind's one member, never a governs
//! collision, and a locus outside the base's refuses when the lock loads.

use std::fs;
use std::path::Path;

use temper::drift::{Declarations, KindFactRow};

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
    common::write_lock(
        root,
        Declarations {
            kinds,
            ..Declarations::default()
        },
    );
    common::check_in(root, &[], Some("github"))
}

#[test]
fn a_document_in_a_subkinds_locus_is_one_member_of_the_subkind() {
    let root = common::tmpdir("subkind-member");
    write_rule(&root, ".claude/rules/decisions/0001-adopt.md");
    write_rule(&root, ".claude/rules/style.md");
    let run = check_with(&root, vec![decision(".claude/rules/decisions", "*.md")]);
    assert!(
        run.output.contains("rule.forbidden_keys") && run.output.contains("style.md"),
        "the base still owns its own documents, got:\n{}",
        run.output
    );
    assert!(
        !run.output.lines().any(|line| line.contains("0001-adopt")
            && (line.contains("title=rule.") || line.contains("`rule` kind's"))),
        "the subkind's document is no member of the base, got:\n{}",
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
