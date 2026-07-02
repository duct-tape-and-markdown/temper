//! The extraction equivalence baseline (`specs/15-kinds.md`, "The extraction
//! algebra — the soundness boundary, as data").
//!
//! Pins the current built-in extractors' output — `extract::skill_features` and
//! `extract::rule_features` — over real Claude Code fixtures that mirror the
//! actual on-disk layout (`.claude/skills/<name>/SKILL.md`, `.claude/rules/*.md`;
//! `.claude/rules/rust.md` guidance: never a layout invented for the test). Each
//! resulting `Features` is snapshotted (Debug), so the later swap to the composed
//! generic path (`BUILTIN-EXTRACT-GENERIC`) can repoint these same tests at the
//! data-driven extractor and prove it yields feature-for-feature identical output:
//! the committed `.snap` files staying byte-identical *is* the equivalence proof.
//!
//! These snapshots pin extraction only — the frontmatter fields at each parsed
//! kind, the body line count, the ATX headings (fence-excluded, closing-hash
//! stripped), the nested sections, and the source directory — never the contract
//! engine that ranges over them.

use std::path::{Path, PathBuf};

use temper::extract;
use temper::rule::Rule;
use temper::skill::Skill;

/// Path to a fixture under `tests/fixtures/extract_equivalence`, resolved from the
/// manifest so the test is independent of the process working directory. The tree
/// mirrors the real harness layout under a frozen `.claude/` root, so the pinned
/// output is not coupled to the live dogfood files (which change tick to tick).
fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/extract_equivalence/.claude")
        .join(rel)
}

/// The `skill_features` projection over a real-shaped `.claude/skills/<name>/SKILL.md`
/// is the fixed target the extraction-unification swap must hold byte-identical. The
/// fixture exercises the full surface a skill extractor decides over: typed fields
/// (`name`/`description`/`version`/`license`), an unknown list key (`allowed-tools`),
/// multi-level ATX headings with a fence-excluded `#` line and a closing-hash heading,
/// nested sections, and the imported directory name.
#[test]
fn skill_features_over_a_real_skill_fixture() {
    let skill = Skill::from_source_dir(&fixture("skills/coordinate"))
        .expect("the coordinate skill fixture should parse");
    let features = extract::skill_features(&skill);
    insta::assert_debug_snapshot!("skill_features_coordinate", features);
}

/// The `rule_features` projection over a `.claude/rules/*.md` rule that carries a
/// `paths:` scope (the `rust.md` dogfood shape) — a typed list field, a body with
/// nested sections, and the discovered `rules` directory.
#[test]
fn rule_features_over_a_paths_rule_fixture() {
    let rule = Rule::from_source_file(&fixture("rules/rust.md"))
        .expect("the rust rule fixture should parse");
    let features = extract::rule_features(&rule);
    insta::assert_debug_snapshot!("rule_features_rust", features);
}

/// The `rule_features` projection over a `.claude/rules/*.md` rule with **no
/// frontmatter** (the `collaboration.md` dogfood shape) — no fields at all, the
/// whole file as the byte-faithful body.
#[test]
fn rule_features_over_a_no_frontmatter_rule_fixture() {
    let rule = Rule::from_source_file(&fixture("rules/collaboration.md"))
        .expect("the collaboration rule fixture should parse");
    let features = extract::rule_features(&rule);
    insta::assert_debug_snapshot!("rule_features_collaboration", features);
}
