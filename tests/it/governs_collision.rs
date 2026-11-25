//! `kind.governs-collision` judged per discovered document: a document two file kinds'
//! loci both reach is the collision, answered by a literal exclude; an overlap no
//! document occupies, or one the exclude already answers, collides with nothing.

use std::fs;
use std::path::Path;

use temper::drift::{Declarations, KindFactRow};

use crate::common;

fn write_doc(root: &Path, relative: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, "# Doc\n\nBody.\n").unwrap();
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
fn a_document_two_kinds_distinct_loci_both_reach_draws_a_governs_collision_naming_both_kinds() {
    let root = common::tmpdir("governs-collision-document");
    write_doc(&root, "docs/charter.md");
    let run = check_with(
        &root,
        vec![
            common::kind_facts("policy", "docs", "*.md"),
            common::kind_facts("standard", ".", "docs/*.md"),
        ],
    );
    assert!(!run.ok, "a document collision blocks, got:\n{}", run.output);
    assert!(
        run.output.contains("kind.governs-collision")
            && run.output.contains("`policy`")
            && run.output.contains("`standard`")
            && run.output.contains("docs/charter.md"),
        "the finding names the document and both kinds, got:\n{}",
        run.output
    );
}

#[test]
fn the_per_document_governs_collision_names_a_literal_exclude_as_its_remedy() {
    let root = common::tmpdir("governs-collision-remedy");
    write_doc(&root, "docs/CLAUDE.md");
    let run = check_with(&root, vec![common::kind_facts("guide", "docs", "*.md")]);
    assert!(
        run.output.contains("kind.governs-collision")
            && run.output.contains("`guide`")
            && run.output.contains("`memory`"),
        "a real docs/CLAUDE.md is reached by memory and the custom kind, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("`docs/CLAUDE.md` to the `exclude`"),
        "the remedy names the literal exclude, got:\n{}",
        run.output
    );
}

#[test]
fn two_kinds_whose_templates_overlap_only_on_unoccupied_paths_draw_no_governs_collision() {
    // `docs/*.md` overlaps memory's `**/CLAUDE.md` at `docs/CLAUDE.md`, which no file occupies.
    let root = common::tmpdir("governs-collision-unoccupied");
    write_doc(&root, "docs/charter.md");
    let run = check_with(&root, vec![common::kind_facts("guide", "docs", "*.md")]);
    assert!(
        !run.output.contains("kind.governs-collision"),
        "an unoccupied overlap collides with nothing, got:\n{}",
        run.output
    );
}

#[test]
fn a_document_excluded_from_one_of_two_overlapping_kinds_draws_no_governs_collision() {
    let root = common::tmpdir("governs-collision-excluded");
    write_doc(&root, "docs/CLAUDE.md");
    write_doc(&root, "docs/charter.md");
    let run = check_with(
        &root,
        vec![KindFactRow {
            exclude: vec!["docs/CLAUDE.md".to_string()],
            ..common::kind_facts("guide", "docs", "*.md")
        }],
    );
    assert!(
        !run.output.contains("kind.governs-collision"),
        "the exclude answers the collision, got:\n{}",
        run.output
    );
}

#[test]
fn the_built_in_kinds_draw_no_governs_collision() {
    let root = common::tmpdir("governs-collision-builtins");
    for doc in [
        "CLAUDE.md",
        "CLAUDE.local.md",
        ".claude/CLAUDE.md",
        "sub/CLAUDE.md",
        ".claude/rules/style.md",
        ".claude/agents/reviewer.md",
        ".claude/commands/ship.md",
        ".claude/skills/demo/SKILL.md",
    ] {
        write_doc(&root, doc);
    }
    let run = check_with(&root, Vec::new());
    assert!(
        !run.output.contains("kind.governs-collision"),
        "the built-in set keeps its claims disjoint, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_naming_no_base_over_a_built_ins_sub_locus_still_draws_a_governs_collision() {
    let root = common::tmpdir("governs-collision-unrelated");
    write_doc(&root, ".claude/rules/decisions/0001.md");
    let run = check_with(
        &root,
        vec![common::kind_facts(
            "decision",
            ".claude/rules/decisions",
            "*.md",
        )],
    );
    assert!(
        run.output.contains("kind.governs-collision")
            && run.output.contains("`decision`")
            && run.output.contains("`rule`"),
        "only a subkind cedes its base's documents, got:\n{}",
        run.output
    );
}
