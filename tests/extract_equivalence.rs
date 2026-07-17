//! The extraction equivalence baseline.
//!
//! Pins the built-in extractors' output — now the composed generic path,
//! `builtin_kind::skill_features` and `builtin_kind::rule_features`, run over a raw
//! `Unit` lifted straight off an imported `Member` — the exact shape `check`'s
//! `resolve_kind_units` builds, no disk round trip — over real Claude Code fixtures
//! that mirror the actual on-disk
//! layout (`.claude/skills/<name>/SKILL.md`, `.claude/rules/*.md`;
//! `.claude/rules/rust.md` guidance: never a layout invented for the test). Each
//! resulting `Features` is snapshotted (Debug):
//! these `.snap` files were pinned against the retired hand-coded
//! `skill_features`/`rule_features` and stay byte-identical under the generic surface
//! read — the unchanged snapshot *is* the equivalence proof.
//!
//! These snapshots pin extraction only — the frontmatter fields at each parsed
//! kind, the body line count, the ATX headings (fence-excluded, closing-hash
//! stripped), the nested sections, and the source directory — never the contract
//! engine that ranges over them.

use std::path::PathBuf;

mod common;

use temper::builtin_kind;
use temper::extract::{FeatureValue, ValueType};
use temper::frontmatter::Member;
use temper::kind::{DirectiveSyntax, Extraction, Primitive};

/// Path to a fixture under the frozen `tests/fixtures/extract_equivalence/.claude`
/// root, mirroring the real harness layout — so the pinned output is not coupled to
/// the live dogfood files (which change tick to tick).
fn fixture(rel: &str) -> PathBuf {
    common::fixture(&format!("extract_equivalence/.claude/{rel}"))
}

/// The `skill_features` projection over a real-shaped `.claude/skills/<name>/SKILL.md`
/// is the fixed target the surface-read unification must hold byte-identical. The
/// fixture exercises the full surface a skill extractor decides over: typed fields
/// (`name`/`description`/`license`), an unknown list key (`allowed-tools`),
/// multi-level ATX headings with a fence-excluded `#` line and a closing-hash heading,
/// nested sections, and the imported directory name.
#[test]
fn skill_features_over_a_real_skill_fixture() {
    let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
    let skill = Member::from_source(&skill_kind, &fixture("skills/coordinate").join("SKILL.md"))
        .expect("the coordinate skill fixture should parse");
    let unit = common::skill_surface_unit(&skill);
    let features = builtin_kind::skill_features(&unit);
    insta::assert_debug_snapshot!("skill_features_coordinate", features);
}

/// A `directives` primitive with syntax `at-import` composes over a `Unit` and folds
/// the body's `@path` occurrences into `Features.directives` in document order — the
/// end-to-end tie from the closed-vocabulary parse through the composed extractor.
#[test]
fn a_directives_primitive_extracts_at_imports_in_document_order() {
    let extraction = Extraction::new(vec![Primitive::Directives {
        syntax: DirectiveSyntax::AtImport,
    }]);

    // Two real imports (relative then absolute); a bare `@` in prose is not an edge.
    let unit = common::raw_unit(
        "CLAUDE",
        std::collections::BTreeMap::new(),
        "# Project memory\n\nLoad @docs/style.md and @/abs/policy.md here; ping me @ noon.\n",
        "CLAUDE.md",
    );
    let features = extraction.extract(&unit);
    assert_eq!(
        features.directives,
        vec!["docs/style.md".to_string(), "/abs/policy.md".to_string()]
    );

    // Order-stable across re-extraction — the same unit yields the same occurrences.
    assert_eq!(extraction.extract(&unit).directives, features.directives);
}

/// A `fenced` primitive composes over a `Unit` and folds the body's fenced blocks
/// into `Features.fenced_blocks` in document order, each block's interior content paired
/// with its info string — surrounding prose skipped. The end-to-end tie from
/// the closed-vocabulary parse through the composed extractor.
#[test]
fn a_fenced_primitive_extracts_block_interiors_with_info_strings_in_order() {
    let extraction = Extraction::new(vec![Primitive::Fenced]);

    // Prose around two fenced blocks — a shell block and a keyed `toml member.manifest`
    // block, the shape the member fence composes with a TOML parse.
    let unit = common::raw_unit(
        "CLAUDE",
        std::collections::BTreeMap::new(),
        "# Kinds\n\
\n\
Surrounding prose is not captured.\n\
\n\
```sh\n\
cargo test\n\
```\n\
\n\
More prose between the fences.\n\
\n\
```toml member.manifest\n\
name = \"coordinate\"\n\
```\n",
        "CLAUDE.md",
    );
    let features = extraction.extract(&unit);

    // Each block yields its interior content (surrounding prose skipped) keyed by its
    // info string, in document order.
    assert_eq!(features.fenced_blocks.len(), 2);
    assert_eq!(features.fenced_blocks[0].info, "sh");
    assert_eq!(features.fenced_blocks[0].content, "cargo test");
    assert_eq!(features.fenced_blocks[1].info, "toml member.manifest");
    assert_eq!(features.fenced_blocks[1].content, "name = \"coordinate\"");

    // Order-stable across re-extraction — a pure function of the body.
    assert_eq!(
        extraction.extract(&unit).fenced_blocks,
        features.fenced_blocks
    );
}

/// A body with no fenced block yields none — absent, never errored.
/// The default a kind composing `fenced` lands on
/// when a member carries no fence, exactly as `directives` yields none for a body with
/// no `@import`.
#[test]
fn a_fenced_primitive_over_a_body_with_no_fence_yields_none() {
    let extraction = Extraction::new(vec![Primitive::Fenced]);
    let unit = common::raw_unit(
        "CLAUDE",
        std::collections::BTreeMap::new(),
        "# Kinds\n\nJust prose, no fenced block at all.\n",
        "CLAUDE.md",
    );
    assert!(extraction.extract(&unit).fenced_blocks.is_empty());
}

/// A `field` primitive **retains** its frontmatter value as parsed — nesting whole, so
/// the clause that addresses into it has something to walk (`tests/field_addressing.rs`
/// judges that face). An absent key is **absent, never errored**, exactly as an unset
/// optional field is.
#[test]
fn a_field_primitive_retains_its_frontmatter_value_as_parsed() {
    let extraction = Extraction::new(vec![
        Primitive::Field {
            key: "permissions".to_string(),
        },
        Primitive::Field {
            key: "name".to_string(),
        },
        Primitive::Field {
            key: "absent".to_string(),
        },
    ]);

    // A JSON-manifest settings shape: a nested `permissions` table over a top-level
    // scalar.
    let serde_json::Value::Object(frontmatter) = serde_json::json!({
        "name": "settings",
        "permissions": {
            "defaultMode": "acceptEdits",
            "retries": 3
    }
    }) else {
        unreachable!("the fixture is a JSON object")
    };
    let unit = common::raw_unit(
        "settings",
        frontmatter.into_iter().collect(),
        "",
        "settings/settings.md",
    );

    let features = extraction.extract(&unit);

    // The nested table survives extraction whole — its keys, and each leaf's own source
    // kind, are still there to be addressed. A flattening read would have kept the
    // top-level kind and dropped everything under it.
    assert_eq!(
        features.fields.get("permissions"),
        Some(&serde_json::json!({"defaultMode": "acceptEdits", "retries": 3}))
    );
    // The feature the top-level lookup projects is still the container's own kind.
    assert_eq!(
        features.field("permissions").map(|v| v.kind()),
        Some(ValueType::Map)
    );

    // A bare key is the flat lookup, kind-preserving as ever.
    assert_eq!(
        features.field("name"),
        Some(FeatureValue::scalar(ValueType::String, "settings"))
    );

    // Absent, never errored — extraction stays total.
    assert!(features.field("absent").is_none());

    // Order-stable across re-extraction — a pure function of the surface.
    assert_eq!(extraction.extract(&unit).fields, features.fields);
}

/// The `rule_features` projection over a `.claude/rules/*.md` rule that carries a
/// `paths:` scope (the `rust.md` dogfood shape) — a typed list field, a body with
/// nested sections, and the discovered `rules` directory.
#[test]
fn rule_features_over_a_paths_rule_fixture() {
    let rule_kind = builtin_kind::definition("rule").unwrap().unwrap();
    let rule = Member::from_source(&rule_kind, &fixture("rules/rust.md"))
        .expect("the rust rule fixture should parse");
    let unit = common::surface_unit(&rule);
    let features = builtin_kind::rule_features(&unit);
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
    let unit = common::surface_unit(&rule);
    let features = builtin_kind::rule_features(&unit);
    insta::assert_debug_snapshot!("rule_features_collaboration", features);
}
