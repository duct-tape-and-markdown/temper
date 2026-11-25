//! The built-in loci claim exactly what Claude Code loads as each kind: `rule` recurses
//! under `.claude/rules/`, and `memory` leaves the `.claude/` folders another built-in
//! loader owns. Driven through `import::discover_kind_files`, the walk `check` takes,
//! over fixtures at the real Claude Code layout.

use std::path::{Path, PathBuf};

use temper::builtin_kind;
use temper::compose;
use temper::drift::Declarations;
use temper::import;
use temper::kind::CustomKind;

use crate::common;

/// The files the built-in `name` kind discovers under `harness`, walking its own locus.
fn discovered(harness: &Path, name: &str) -> Vec<PathBuf> {
    let kind = builtin_kind::definition(name).expect("the kind is built in");
    discover(harness, &kind)
}

fn discover(harness: &Path, kind: &CustomKind) -> Vec<PathBuf> {
    import::discover_kind_files(
        &import::Discovery::new(harness),
        kind,
        kind.governs.as_ref().expect("a file locus"),
        import::LocalOverride::Honored,
    )
}

#[test]
fn a_rule_in_a_subdirectory_of_the_rules_folder_is_discovered_as_a_rule() {
    let harness = common::tmpdir("rule-subdirectory");
    common::write_sibling(&harness, ".claude/rules/rust.md", "# Rust\n");
    common::write_sibling(
        &harness,
        ".claude/rules/frontend/react.md",
        "---\npaths:\n  - \"src/**/*.tsx\"\n---\n# React\n",
    );

    assert_eq!(
        discovered(&harness, "rule"),
        vec![
            harness.join(".claude/rules/frontend/react.md"),
            harness.join(".claude/rules/rust.md"),
        ]
    );
}

#[test]
fn a_claude_md_under_the_rules_folder_is_not_discovered_as_memory() {
    let harness = common::tmpdir("memory-under-rules");
    common::write_sibling(&harness, "CLAUDE.md", "# Root\n");
    common::write_sibling(&harness, ".claude/rules/CLAUDE.md", "# A rule\n");
    common::write_sibling(&harness, ".claude/rules/frontend/CLAUDE.md", "# A rule\n");

    assert_eq!(
        discovered(&harness, "memory"),
        vec![harness.join("CLAUDE.md")]
    );
    // The rules loader owns it: the same file is a rule.
    assert!(discovered(&harness, "rule").contains(&harness.join(".claude/rules/CLAUDE.md")));
}

#[test]
fn a_claude_md_under_the_agents_commands_or_skills_folder_is_not_discovered_as_memory() {
    let harness = common::tmpdir("memory-under-loaders");
    common::write_sibling(&harness, "CLAUDE.md", "# Root\n");
    for folder in ["agents", "commands", "skills"] {
        common::write_sibling(&harness, &format!(".claude/{folder}/CLAUDE.md"), "# x\n");
        common::write_sibling(
            &harness,
            &format!(".claude/{folder}/nested/CLAUDE.md"),
            "# x\n",
        );
    }

    assert_eq!(
        discovered(&harness, "memory"),
        vec![harness.join("CLAUDE.md")]
    );
}

#[test]
fn a_claude_md_under_claude_outside_the_loader_folders_stays_memory() {
    let harness = common::tmpdir("memory-under-claude");
    common::write_sibling(&harness, ".claude/CLAUDE.md", "# Project\n");
    common::write_sibling(&harness, "packages/web/CLAUDE.md", "# Web\n");

    assert_eq!(
        discovered(&harness, "memory"),
        vec![
            harness.join(".claude/CLAUDE.md"),
            harness.join("packages/web/CLAUDE.md"),
        ]
    );
}

#[test]
fn a_committed_lock_rows_exclude_overlays_the_built_in_kinds() {
    let builtin = builtin_kind::definition("memory").expect("memory is built in");
    let mut row = temper::builtin_lock::declarations()
        .kinds
        .iter()
        .find(|row| row.name == "memory")
        .expect("the embedded lock carries memory")
        .clone();
    row.exclude = vec!["vendor".to_string()];
    let declarations = Declarations {
        kinds: vec![row],
        ..Declarations::default()
    };

    let overlaid = compose::overlay_builtin_kind(&builtin, &declarations).unwrap();
    assert_eq!(overlaid.excluded, vec!["vendor".to_string()]);

    let harness = common::tmpdir("memory-overlaid-exclude");
    common::write_sibling(&harness, "CLAUDE.md", "# Root\n");
    common::write_sibling(&harness, "vendor/lib/CLAUDE.md", "# Vendored\n");
    common::write_sibling(&harness, ".claude/rules/CLAUDE.md", "# Now memory\n");
    assert_eq!(
        discover(&harness, &overlaid),
        vec![
            harness.join(".claude/rules/CLAUDE.md"),
            harness.join("CLAUDE.md"),
        ]
    );
}

#[test]
fn every_built_in_kind_glob_is_a_placement_template() {
    let population = builtin_kind::definitions();
    assert!(!population.is_empty());
    for row in &temper::builtin_lock::declarations().kinds {
        // The lift refuses a `governs_glob` outside the placement-template grammar.
        let lifted = CustomKind::from_kind_fact_row(row)
            .unwrap_or_else(|err| panic!("`{}`'s glob is a placement template: {err}", row.name));
        if let Some(spelled) = population.get(&row.name) {
            assert_eq!(lifted.governs, spelled.governs, "{}: governs", row.name);
        }
    }
}
