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
