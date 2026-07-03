//! The extraction equivalence baseline (`specs/architecture/15-kinds.md`, "The extraction
//! algebra — the soundness boundary, as data").
//!
//! Pins the built-in extractors' output — now the composed generic path,
//! `builtin_kind::skill_features` and `builtin_kind::rule_features`, run over a
//! surface member document reloaded through the one generic `Unit` loader (`Unit::from_member_document`)
//! `check` reads — over real Claude Code fixtures that mirror the actual on-disk
//! layout (`.claude/skills/<name>/SKILL.md`, `.claude/rules/*.md`;
//! `.claude/rules/rust.md` guidance: never a layout invented for the test). Each
//! fixture is imported, projected to its authored surface document
//! (`Member::to_document`), then re-read as a generic `Unit` — the exact check
//! read path, no IR→Unit adapter. Each resulting `Features` is snapshotted (Debug):
//! these `.snap` files were pinned against the retired hand-coded
//! `skill_features`/`rule_features` and stay byte-identical under the generic surface
//! read — the unchanged snapshot *is* the equivalence proof.
//!
//! These snapshots pin extraction only — the frontmatter fields at each parsed
//! kind, the body line count, the ATX headings (fence-excluded, closing-hash
//! stripped), the nested sections, and the source directory — never the contract
//! engine that ranges over them.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::builtin_kind;
use temper::frontmatter::Member;
use temper::kind::Unit;

/// Path to a fixture under `tests/fixtures/extract_equivalence`, resolved from the
/// manifest so the test is independent of the process working directory. The tree
/// mirrors the real harness layout under a frozen `.claude/` root, so the pinned
/// output is not coupled to the live dogfood files (which change tick to tick).
fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/extract_equivalence/.claude")
        .join(rel)
}

/// A fresh, empty temp directory unique to this test run — the surface the imported
/// fixture is projected into before it is read back generically.
fn tmpdir(label: &str) -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "extract-equivalence-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Write an imported skill's authored surface member document `<name>/SKILL.md`
/// (`Member::to_document`) and reload it through the generic `Unit` loader `check`
/// reads — the built-in kind's member-document read, no IR→Unit adapter.
fn skill_surface_unit(member: &Member, name: &str) -> Unit {
    let dir = tmpdir(name).join(name);
    std::fs::create_dir_all(&dir).unwrap();
    let doc_path = dir.join("SKILL.md");
    std::fs::write(&doc_path, member.to_document().emit()).unwrap();
    Unit::from_member_document(&dir, &doc_path).unwrap()
}

/// The rule counterpart to [`skill_surface_unit`]: project the imported rule to its
/// `<name>/RULE.md` surface document (`Member::to_document`) and reload it as a
/// generic `Unit`.
fn rule_surface_unit(member: &Member, name: &str) -> Unit {
    let dir = tmpdir(name).join(name);
    std::fs::create_dir_all(&dir).unwrap();
    let doc_path = dir.join("RULE.md");
    std::fs::write(&doc_path, member.to_document().emit()).unwrap();
    Unit::from_member_document(&dir, &doc_path).unwrap()
}

/// The `skill_features` projection over a real-shaped `.claude/skills/<name>/SKILL.md`
/// is the fixed target the surface-read unification must hold byte-identical. The
/// fixture exercises the full surface a skill extractor decides over: typed fields
/// (`name`/`description`/`version`/`license`), an unknown list key (`allowed-tools`),
/// multi-level ATX headings with a fence-excluded `#` line and a closing-hash heading,
/// nested sections, and the imported directory name.
#[test]
fn skill_features_over_a_real_skill_fixture() {
    let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
    let skill = Member::from_source(&skill_kind, &fixture("skills/coordinate").join("SKILL.md"))
        .expect("the coordinate skill fixture should parse");
    let unit = skill_surface_unit(&skill, "coordinate");
    let features =
        builtin_kind::skill_features(&unit).expect("the embedded skill extraction should run");
    insta::assert_debug_snapshot!("skill_features_coordinate", features);
}

/// The `rule_features` projection over a `.claude/rules/*.md` rule that carries a
/// `paths:` scope (the `rust.md` dogfood shape) — a typed list field, a body with
/// nested sections, and the discovered `rules` directory.
#[test]
fn rule_features_over_a_paths_rule_fixture() {
    let rule_kind = builtin_kind::definition("rule").unwrap().unwrap();
    let rule = Member::from_source(&rule_kind, &fixture("rules/rust.md"))
        .expect("the rust rule fixture should parse");
    let unit = rule_surface_unit(&rule, "rust");
    let features =
        builtin_kind::rule_features(&unit).expect("the embedded rule extraction should run");
    insta::assert_debug_snapshot!("rule_features_rust", features);
}

/// The `rule_features` projection over a `.claude/rules/*.md` rule with **no
/// frontmatter** (the `collaboration.md` dogfood shape) — no fields at all, the
/// whole file as the byte-faithful body.
#[test]
fn rule_features_over_a_no_frontmatter_rule_fixture() {
    let rule_kind = builtin_kind::definition("rule").unwrap().unwrap();
    let rule = Member::from_source(&rule_kind, &fixture("rules/collaboration.md"))
        .expect("the collaboration rule fixture should parse");
    let unit = rule_surface_unit(&rule, "collaboration");
    let features =
        builtin_kind::rule_features(&unit).expect("the embedded rule extraction should run");
    insta::assert_debug_snapshot!("rule_features_collaboration", features);
}
