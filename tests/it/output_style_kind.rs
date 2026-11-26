//! The `output-style` built-in kind: a markdown file under `.claude/output-styles/`,
//! optional frontmatter over the style's instructions, named by its `name` field when it
//! carries one and by its file stem otherwise. Driven at the crate-public read API over
//! fixtures mirroring the real Claude Code layout, plus the one-shot `check --harness`
//! boundary for the coverage note.

use std::path::{Path, PathBuf};

use crate::common;

use temper::builtin_kind;
use temper::frontmatter::Member;
use temper::import;

/// Write an output style at `<root>/.claude/output-styles/<stem>.md`, the documented
/// project location.
fn write_style(root: &Path, stem: &str, body: &str) -> PathBuf {
    let rel = format!(".claude/output-styles/{stem}.md");
    common::write_sibling(root, &rel, body);
    root.join(rel)
}

/// Discover the output-style files under `harness` and name each one as `check` reads it.
fn style_members(harness: &Path) -> Vec<(PathBuf, String)> {
    let kind = builtin_kind::definition("output-style").expect("output-style is embedded");
    let governs = kind.governs.clone().unwrap();
    let disc = import::Discovery::new(harness);
    let base = temper::drift::fold_base(&harness.join(&governs.root), &governs.glob);
    import::discover_kind_files(&disc, &kind, &governs, import::LocalOverride::Honored)
        .into_iter()
        .map(|file| {
            let id = Member::from_source_rooted(&kind, &file, &base).unwrap().id;
            (file, id)
        })
        .collect()
}

#[test]
fn a_markdown_file_under_claude_output_styles_is_an_output_style_member() {
    let harness = common::tmpdir("output-style-member");
    let named = write_style(
        &harness,
        "teaching",
        "---\nname: Teaching Mode\ndescription: Explains as it goes.\nkeep-coding-instructions: true\n---\n\nExplain each change before making it.\n",
    );

    assert_eq!(
        style_members(&harness),
        vec![(named, "Teaching Mode".to_string())],
        "a style's `name` is its identity, never the filename"
    );
}

#[test]
fn an_output_style_with_no_name_takes_its_file_stem() {
    let harness = common::tmpdir("output-style-stem");
    let bare = write_style(&harness, "terse", "Answer in one sentence.\n");
    let described = write_style(
        &harness,
        "plain",
        "---\ndescription: No jargon.\n---\n\nAvoid jargon.\n",
    );
    // Unparseable YAML still loads under the file name with no fields set.
    let malformed = write_style(
        &harness,
        "broken",
        "---\ndescription: a: b: c\n---\n\nBody.\n",
    );

    assert_eq!(
        style_members(&harness),
        vec![
            (malformed, "broken".to_string()),
            (described, "plain".to_string()),
            (bare, "terse".to_string()),
        ]
    );
}

#[test]
fn a_claude_output_styles_folder_never_fires_unclaimed_entry() {
    let harness = common::tmpdir("output-style-coverage");
    write_style(&harness, "terse", "Answer in one sentence.\n");
    common::write_sibling(&harness, ".claude/stray.md", "");

    let (findings, success) = common::check_harness(&harness);

    let unclaimed = common::findings_for(&findings, "coverage.unclaimed-entry");
    assert!(
        unclaimed.iter().any(|f| f.contains(".claude/stray.md")),
        "the scan still runs and names a real stray, got: {findings:#?}"
    );
    assert!(
        !unclaimed
            .iter()
            .any(|f| f.contains(".claude/output-styles")),
        "the output-styles folder is the output-style kind's locus, got: {unclaimed:#?}"
    );
    let checked = common::findings_for(&findings, "coverage.checked");
    assert!(
        checked.iter().any(|f| f.contains("output-style (1)")),
        "the style is checked as an output-style member, got: {checked:#?}"
    );
    assert!(success, "{findings:#?}");
}

#[test]
fn a_claude_md_inside_the_output_styles_folder_is_never_memory() {
    let harness = common::tmpdir("output-style-not-memory");
    write_style(
        &harness,
        "CLAUDE",
        "A style that happens to be named CLAUDE.\n",
    );
    common::write_sibling(&harness, "CLAUDE.md", "# Project\n");

    let kinds = temper::compose::effective_kinds(temper::builtin_lock::declarations())
        .expect("the embedded kinds compose");
    let memory = &kinds["memory"];
    let governs = memory.governs.clone().unwrap();
    let found = import::discover_kind_files(
        &import::Discovery::new(&harness),
        memory,
        &governs,
        import::LocalOverride::Honored,
    );
    assert_eq!(found, vec![harness.join("CLAUDE.md")]);
}

/// Stand `harness` up as an adopted harness whose program composes the built-in kinds and
/// no member: the shipped lock's declaration rows, committed. The gate reads its edges off
/// the committed lock, so this is the lock an `emit` of a memberless program writes.
fn adopt_builtins(harness: &Path) {
    common::write_lock(harness, temper::builtin_lock::declarations().clone());
}

/// The findings of rule `rule` a `check --harness` run over `harness` reports.
fn findings_of(harness: &Path, rule: &str) -> Vec<String> {
    let (findings, _) = common::check_harness(harness);
    common::findings_for(&findings, rule)
        .into_iter()
        .cloned()
        .collect()
}

/// The `graph.route` findings a `check --harness` run over `harness` reports.
fn route_findings(harness: &Path) -> Vec<String> {
    findings_of(harness, "graph.route")
}

/// The off-tree coverage notes a `check --harness` run over `harness` reports.
fn off_tree_notes(harness: &Path) -> Vec<String> {
    findings_of(harness, "coverage.off-tree-targets")
}

#[test]
fn a_settings_output_style_naming_an_output_style_member_resolves_to_it() {
    let harness = common::tmpdir("output-style-settings-edge");
    write_style(
        &harness,
        "teaching",
        "---\nname: Teaching Mode\n---\n\nExplain each change before making it.\n",
    );
    common::write_settings(&harness, "{ \"outputStyle\": \"Teaching Mode\" }\n");
    adopt_builtins(&harness);

    let narration = common::explain_in(&harness, "settings");
    assert!(
        narration
            .contains("it points at `Teaching Mode` (output-style) via its `outputStyle` field"),
        "the settings member selects the style it names, got: {narration}"
    );
    let style = common::explain_in(&harness, "output-style:Teaching Mode");
    assert!(
        style.contains("`settings`") && style.contains("`outputStyle`"),
        "the style narrates the settings edge that selects it, got: {style}"
    );
    assert_eq!(route_findings(&harness), Vec::<String>::new());
}

#[test]
fn a_settings_output_style_matches_a_style_name_exactly_and_case_sensitively() {
    let harness = common::tmpdir("output-style-settings-exact");
    write_style(&harness, "terse", "Answer in one sentence.\n");
    // The documented case-mismatch example: the runtime falls back to the default style.
    common::write_settings(&harness, "{ \"outputStyle\": \"Terse\" }\n");
    adopt_builtins(&harness);

    // The mis-cased name never resolves to `terse`; a user or plugin style may still carry
    // it, so it is noted off-tree rather than dangling.
    assert_eq!(route_findings(&harness), Vec::<String>::new());
    let notes = off_tree_notes(&harness);
    assert_eq!(notes.len(), 1, "got: {notes:#?}");
    assert!(
        notes[0].contains("`settings` `outputStyle` → `Terse`"),
        "the note names the field and the value it wrote, got: {notes:#?}"
    );
}

#[test]
fn a_settings_output_style_naming_a_built_in_style_draws_no_dangling_edge() {
    // The runtime ships these styles itself, spelled as the output-styles page spells them
    // (code.claude.com/docs/en/output-styles, retrieved 2026-11-25): it loads what they
    // name, so no member behind them is no fault (decision 0101).
    for builtin in ["default", "Proactive", "Concise", "Explanatory", "Learning"] {
        let harness = common::tmpdir("output-style-settings-builtin");
        common::write_settings(&harness, &format!("{{ \"outputStyle\": \"{builtin}\" }}\n"));
        adopt_builtins(&harness);
        assert_eq!(
            route_findings(&harness),
            Vec::<String>::new(),
            "`{builtin}` is a style the runtime provides"
        );
        assert_eq!(off_tree_notes(&harness), Vec::<String>::new());
    }

    // The exemption is the exact spelling, never a case-folded one.
    let harness = common::tmpdir("output-style-settings-builtin-cased");
    common::write_settings(&harness, "{ \"outputStyle\": \"explanatory\" }\n");
    adopt_builtins(&harness);
    assert_eq!(off_tree_notes(&harness).len(), 1);
}

#[test]
fn a_lockless_harness_resolves_a_provided_outputstyle_name() {
    // No committed lock: the embedded built-in lock's edges are the ones judged. The
    // project's style resolves to its member; a built-in style is provided.
    let harness = common::tmpdir("output-style-lockless-provided");
    write_style(&harness, "terse", "Answer in one sentence.\n");
    common::write_settings(&harness, "{ \"outputStyle\": \"Proactive\" }\n");
    common::write_sibling(
        &harness,
        ".claude/settings.local.json",
        "{ \"outputStyle\": \"terse\" }\n",
    );

    let narration = common::explain_in(&harness, "settings-local:settings.local");
    assert!(
        narration.contains("it points at `terse` (output-style) via its `outputStyle` field"),
        "the lockless overlay selects the style it names, got: {narration}"
    );
    assert_eq!(route_findings(&harness), Vec::<String>::new());
    assert_eq!(
        off_tree_notes(&harness),
        Vec::<String>::new(),
        "a provided name is never off-tree"
    );
}

#[test]
fn a_lockless_harness_notes_an_off_tree_outputstyle_as_one_coverage_note() {
    let harness = common::tmpdir("output-style-lockless-off-tree");
    write_style(&harness, "terse", "Answer in one sentence.\n");
    common::write_settings(&harness, "{ \"outputStyle\": \"Nowhere\" }\n");
    common::write_sibling(
        &harness,
        ".claude/settings.local.json",
        "{ \"outputStyle\": \"Personal\" }\n",
    );

    assert_eq!(
        route_findings(&harness),
        Vec::<String>::new(),
        "a name the tree cannot show is never dangling"
    );
    let notes = off_tree_notes(&harness);
    assert_eq!(
        notes.len(),
        1,
        "one note for every off-tree name, got: {notes:#?}"
    );
    assert!(
        notes[0].contains("`settings` `outputStyle` → `Nowhere`")
            && notes[0].contains("`settings.local` `outputStyle` → `Personal`")
            && notes[0].contains("~/.claude/output-styles"),
        "the note names each reference and the scopes temper does not read, got: {notes:#?}"
    );
}

/// Write an output style at `<root>/<dir>/.claude/output-styles/<stem>.md`, a nested
/// `.claude` folder's documented location.
fn write_nested_style(root: &Path, dir: &str, stem: &str, body: &str) -> PathBuf {
    let rel = format!("{dir}/.claude/output-styles/{stem}.md");
    common::write_sibling(root, &rel, body);
    root.join(rel)
}

/// A root `terse` and an `apps/web` one sharing its name, the shadowing pair.
fn shadowing_pair(harness: &Path) {
    write_style(harness, "terse", "Answer in one sentence.\n");
    write_nested_style(
        harness,
        "apps/web",
        "terse",
        "---\nname: terse\n---\n\nAnswer in one short sentence.\n",
    );
}

#[test]
fn an_output_style_in_a_nested_claude_folder_is_claimed() {
    let harness = common::tmpdir("output-style-nested");
    let root = write_style(&harness, "plain", "Avoid jargon.\n");
    let nested = write_nested_style(&harness, "apps/web", "terse", "Answer in one sentence.\n");
    let named = write_nested_style(
        &harness,
        "apps",
        "teach",
        "---\nname: Teaching Mode\n---\n\nExplain as you go.\n",
    );

    assert_eq!(
        style_members(&harness),
        vec![
            (root, "plain".to_string()),
            (named, "apps:Teaching Mode".to_string()),
            (nested, "apps:web:terse".to_string()),
        ],
        "a nested style's identity is its directory's segments, then its name or stem"
    );
}

#[test]
fn a_root_and_a_nested_output_style_sharing_a_name_check_with_no_admissibility_finding() {
    let harness = common::tmpdir("output-style-shared-name");
    shadowing_pair(&harness);

    let (findings, success) = common::check_harness(&harness);

    assert_eq!(
        common::findings_for(&findings, "member.admissibility"),
        Vec::<&String>::new(),
        "two placements are two members, never one address refused"
    );
    let checked = common::findings_for(&findings, "coverage.checked");
    assert!(
        checked.iter().any(|f| f.contains("output-style (2)")),
        "both styles are checked as output-style members, got: {checked:#?}"
    );
    assert!(success, "{findings:#?}");
}

#[test]
fn a_nested_output_style_sharing_a_root_style_name_draws_the_shadowing_advisory() {
    let harness = common::tmpdir("output-style-shadowing");
    shadowing_pair(&harness);
    write_nested_style(&harness, "apps/api", "plain", "Avoid jargon.\n");

    let (findings, success) = common::check_harness(&harness);

    let shadowing = common::findings_for(&findings, "output-style.unshadowed");
    assert_eq!(
        shadowing.len(),
        1,
        "only the shadowing style, got: {findings:#?}"
    );
    assert!(
        shadowing[0].starts_with("::warning")
            && shadowing[0].contains("apps:web:terse")
            && shadowing[0].contains("at the root")
            && shadowing[0].contains("at or below `apps/web`"),
        "the advisory names the shadowed style and the sessions it applies in, got: {shadowing:#?}"
    );
    assert!(success, "an advisory never fails the gate: {findings:#?}");
}

#[test]
fn a_root_settings_output_style_resolves_to_the_root_style() {
    let harness = common::tmpdir("output-style-root-selection");
    shadowing_pair(&harness);
    common::write_settings(&harness, "{ \"outputStyle\": \"terse\" }\n");

    let narration = common::explain_in(&harness, "settings");
    assert!(
        narration.contains("it points at `terse` (output-style) via its `outputStyle` field"),
        "the project settings select the style at their own level, got: {narration}"
    );
    assert_eq!(route_findings(&harness), Vec::<String>::new());
    assert_eq!(off_tree_notes(&harness), Vec::<String>::new());
}

#[test]
fn a_claude_md_in_a_nested_output_styles_folder_draws_no_governs_collision() {
    let harness = common::tmpdir("output-style-nested-claude-md");
    write_nested_style(&harness, "apps/web", "CLAUDE", "A style named CLAUDE.\n");
    common::write_sibling(&harness, "CLAUDE.md", "# Project\n");

    let (findings, success) = common::check_harness(&harness);

    assert_eq!(
        common::findings_for(&findings, "kind.governs-collision"),
        Vec::<&String>::new()
    );
    assert!(success, "{findings:#?}");
}

#[test]
fn a_claude_md_in_a_nested_output_styles_folder_is_the_output_styles_not_memorys() {
    let harness = common::tmpdir("output-style-nested-not-memory");
    let style = write_nested_style(&harness, "apps/web", "CLAUDE", "A style named CLAUDE.\n");
    common::write_sibling(&harness, "apps/web/CLAUDE.md", "# Web\n");

    let kinds = temper::compose::effective_kinds(temper::builtin_lock::declarations())
        .expect("the embedded kinds compose");
    let memory = &kinds["memory"];
    let governs = memory.governs.clone().unwrap();
    let found = import::discover_kind_files(
        &import::Discovery::new(&harness),
        memory,
        &governs,
        import::LocalOverride::Honored,
    );
    assert_eq!(found, vec![harness.join("apps/web/CLAUDE.md")]);
    assert_eq!(
        style_members(&harness),
        vec![(style, "apps:web:CLAUDE".to_string())]
    );
}
