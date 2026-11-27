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

/// The files the effective `name` kind discovers under `harness` — the embedded kind set
/// composed, so each kind carries the cedes the set derives.
fn effectively_discovered(harness: &Path, name: &str) -> Vec<PathBuf> {
    let kinds = compose::effective_kinds(temper::builtin_lock::declarations())
        .expect("the embedded kinds compose");
    discover(harness, &kinds[name])
}

#[test]
fn a_claude_md_under_the_rules_folder_is_not_discovered_as_memory() {
    let harness = common::tmpdir("memory-under-rules");
    common::write_sibling(&harness, "CLAUDE.md", "# Root\n");
    common::write_sibling(&harness, ".claude/rules/CLAUDE.md", "# A rule\n");
    common::write_sibling(&harness, ".claude/rules/frontend/CLAUDE.md", "# A rule\n");
    common::write_sibling(&harness, "apps/web/CLAUDE.md", "# Web\n");
    common::write_sibling(&harness, "apps/web/.claude/rules/CLAUDE.md", "# A rule\n");
    common::write_sibling(&harness, "apps/web/.claude/rules/AGENTS.md", "# A rule\n");

    assert_eq!(
        effectively_discovered(&harness, "memory"),
        vec![
            harness.join("CLAUDE.md"),
            harness.join("apps/web/CLAUDE.md")
        ]
    );
    // The rules loader owns it: the same file is a rule.
    let rules = discovered(&harness, "rule");
    assert!(rules.contains(&harness.join(".claude/rules/CLAUDE.md")));
    assert!(rules.contains(&harness.join("apps/web/.claude/rules/CLAUDE.md")));
}

#[test]
fn a_rule_under_a_subdirectorys_own_rules_folder_is_discovered_at_every_depth() {
    let harness = common::tmpdir("rule-nested-folder");
    common::write_sibling(&harness, ".claude/rules/rust.md", "# Rust\n");
    common::write_sibling(&harness, "apps/web/.claude/rules/x.md", "# X\n");
    common::write_sibling(
        &harness,
        "apps/web/.claude/rules/frontend/react.md",
        "# React\n",
    );
    // A markdown file in the subdirectory beside its rules folder is no rule.
    common::write_sibling(&harness, "apps/web/notes.md", "# Notes\n");

    assert_eq!(
        discovered(&harness, "rule"),
        vec![
            harness.join(".claude/rules/rust.md"),
            harness.join("apps/web/.claude/rules/frontend/react.md"),
            harness.join("apps/web/.claude/rules/x.md"),
        ]
    );
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
fn a_claude_md_inside_a_nested_skills_directory_is_not_discovered_as_memory() {
    let harness = common::tmpdir("memory-under-nested-skill");
    common::write_sibling(&harness, "CLAUDE.md", "# Root\n");
    common::write_sibling(&harness, "apps/web/CLAUDE.md", "# Web\n");
    common::write_sibling(
        &harness,
        "apps/web/.claude/skills/deploy/SKILL.md",
        "---\nname: deploy\ndescription: Deploy the web app.\n---\n# Deploy\n",
    );
    common::write_sibling(
        &harness,
        "apps/web/.claude/skills/deploy/CLAUDE.md",
        "# x\n",
    );
    common::write_sibling(
        &harness,
        "apps/web/.claude/skills/deploy/notes/deep/CLAUDE.md",
        "# x\n",
    );

    let kinds = compose::effective_kinds(temper::builtin_lock::declarations())
        .expect("the embedded kinds compose");
    let memory = &kinds["memory"];
    // Ceded by the skill's own template claim, never an exclusion on memory's row.
    assert!(
        !memory.excluded.iter().any(|path| path.contains("apps")),
        "{:?}",
        memory.excluded
    );
    assert_eq!(
        discover(&harness, memory),
        vec![
            harness.join("CLAUDE.md"),
            harness.join("apps/web/CLAUDE.md")
        ]
    );
    // The skill's own walk still finds its entry file.
    assert_eq!(
        discover(&harness, &kinds["skill"]),
        vec![harness.join("apps/web/.claude/skills/deploy/SKILL.md")]
    );
}

#[test]
fn a_claude_local_md_is_claimed_by_memory_local_never_memory() {
    let harness = common::tmpdir("memory-local-claim");
    common::write_sibling(&harness, "CLAUDE.md", "# Team\n");
    common::write_sibling(&harness, "CLAUDE.local.md", "# Mine\n");
    common::write_sibling(&harness, "apps/web/CLAUDE.local.md", "# Mine\n");

    assert_eq!(
        effectively_discovered(&harness, "memory"),
        vec![harness.join("CLAUDE.md")]
    );
    assert_eq!(
        effectively_discovered(&harness, "memory-local"),
        vec![
            harness.join("CLAUDE.local.md"),
            harness.join("apps/web/CLAUDE.local.md"),
        ]
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
        // The lift refuses a `locus_glob` outside the placement-template grammar.
        let lifted = CustomKind::from_kind_fact_row(row)
            .unwrap_or_else(|err| panic!("`{}`'s glob is a placement template: {err}", row.name));
        if let Some(spelled) = population.get(&row.name) {
            assert_eq!(lifted.governs, spelled.governs, "{}: governs", row.name);
        }
    }
}

/// The embedded lock's own `skill` row with `edit` applied, overlaid onto the built-in
/// the way the lock reader overlays an SDK relocation row.
fn overlaid_skill(edit: impl FnOnce(&mut temper::drift::KindFactRow)) -> CustomKind {
    let builtin = builtin_kind::definition("skill").expect("skill is built in");
    let mut row = temper::builtin_lock::declarations()
        .kinds
        .iter()
        .find(|row| row.name == "skill")
        .expect("the embedded lock carries skill")
        .clone();
    edit(&mut row);
    let declarations = Declarations {
        kinds: vec![row],
        ..Declarations::default()
    };
    compose::overlay_builtin_kind(&builtin, &declarations).unwrap()
}

#[test]
fn a_relocation_rows_exclusion_leaves_a_fixture_skill_ungoverned() {
    let harness = common::tmpdir("skill-relocation-exclude");
    let skill_md = "---\nname: x\ndescription: x\n---\n# x\n";
    common::write_sibling(&harness, ".claude/skills/deploy/SKILL.md", skill_md);
    common::write_sibling(&harness, ".claude/skills/fixture/SKILL.md", skill_md);
    let fixture = "tests/fixtures/extract_equivalence/.claude/skills/coordinate/SKILL.md";
    common::write_sibling(&harness, fixture, skill_md);

    // Non-vacuity: the built-in any-depth locus claims the fixture skill.
    assert!(discovered(&harness, "skill").contains(&harness.join(fixture)));

    // An exclude-only relocation row: the built-in's locus, authored exclusions.
    let scoped = overlaid_skill(|row| {
        row.exclude = vec![
            ".claude/skills/fixture".to_string(),
            "tests/fixtures/extract_equivalence".to_string(),
        ]
    });
    assert_eq!(
        discover(&harness, &scoped),
        vec![harness.join(".claude/skills/deploy/SKILL.md")]
    );
}

const SKILL_MD: &str = "---\nname: deploy\ndescription: Ship it.\n---\n# Deploy\n";

#[test]
fn a_skill_under_a_subdirectorys_own_skills_folder_is_a_member() {
    let harness = common::tmpdir("skill-subdirectory");
    common::write_sibling(&harness, ".claude/skills/deploy/SKILL.md", SKILL_MD);
    common::write_sibling(
        &harness,
        "apps/web/.claude/skills/deploy/SKILL.md",
        SKILL_MD,
    );

    assert_eq!(
        discovered(&harness, "skill"),
        vec![
            harness.join(".claude/skills/deploy/SKILL.md"),
            harness.join("apps/web/.claude/skills/deploy/SKILL.md"),
        ]
    );
}

#[test]
fn a_nested_skills_identity_prefixes_its_subdirectory_segments_joined_with_colons() {
    let harness = common::tmpdir("skill-subdirectory-identity");
    common::write_sibling(&harness, ".claude/skills/deploy/SKILL.md", SKILL_MD);
    common::write_sibling(
        &harness,
        "apps/web/.claude/skills/deploy/SKILL.md",
        SKILL_MD,
    );
    let skill = builtin_kind::definition("skill").expect("skill is built in");
    let address = |relative: &str| {
        compose::governed_member_address(
            &skill,
            &harness,
            &harness.join(relative),
            compose::SiteIdentity::OnDisk,
        )
    };

    assert_eq!(
        address(".claude/skills/deploy/SKILL.md").as_deref(),
        Some("skill:deploy")
    );
    assert_eq!(
        address("apps/web/.claude/skills/deploy/SKILL.md").as_deref(),
        Some("skill:apps:web:deploy")
    );

    // `check` reads the same identity: a finding on the nested skill names it so.
    common::write_sibling(
        &harness,
        "apps/web/.claude/skills/deploy/SKILL.md",
        "---\nname: deploy\n---\n# Deploy\n",
    );
    let (findings, _) = common::check_harness(&harness);
    assert!(
        findings
            .iter()
            .any(|line| line.contains("title=skill.required.description::apps:web:deploy:")),
        "{findings:#?}"
    );
}

#[test]
fn a_nested_harnesss_skills_stay_outside_the_outer_harness() {
    let harness = common::tmpdir("skill-nested-harness");
    common::write_sibling(&harness, ".claude/skills/deploy/SKILL.md", SKILL_MD);
    common::write_sibling(
        &harness,
        "apps/web/.claude/skills/deploy/SKILL.md",
        SKILL_MD,
    );
    // `apps/web` governs itself: its own workspace lock fences it from the outer walk.
    common::write_sibling(&harness, "apps/web/.temper/lock.toml", "");

    assert_eq!(
        discovered(&harness, "skill"),
        vec![harness.join(".claude/skills/deploy/SKILL.md")]
    );
}
