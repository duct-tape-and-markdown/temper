//! The extraction equivalence baseline.
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
use temper::extract::{FeatureValue, ValueType};
use temper::frontmatter::Member;
use temper::kind::{DirectiveSyntax, Extraction, Primitive, Unit};

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
/// (`name`/`description`/`license`), an unknown list key (`allowed-tools`),
/// multi-level ATX headings with a fence-excluded `#` line and a closing-hash heading,
/// nested sections, and the imported directory name.
#[test]
fn skill_features_over_a_real_skill_fixture() {
    let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
    let skill = Member::from_source(&skill_kind, &fixture("skills/coordinate").join("SKILL.md"))
        .expect("the coordinate skill fixture should parse");
    let unit = skill_surface_unit(&skill, "coordinate");
    let features = builtin_kind::skill_features(&unit);
    insta::assert_debug_snapshot!("skill_features_coordinate", features);
}

/// A raw memory-shaped `Unit` — no frontmatter, its whole body the byte-faithful
/// markdown a `CLAUDE.md` carries — over an arbitrary `source_path`, for driving a
/// composed `directives` extractor without touching disk.
fn memory_unit(body: &str) -> Unit {
    Unit {
        id: "CLAUDE".to_string(),
        frontmatter: std::collections::BTreeMap::new(),
        body: body.to_string(),
        source_path: PathBuf::from("CLAUDE.md"),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
        published_requirements: Vec::new(),
    }
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
    let unit = memory_unit(
        "# Project memory\n\nLoad @docs/style.md and @/abs/policy.md here; ping me @ noon.\n",
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

    // Prose around two fenced blocks — a shell block and a keyed `toml genre.manifest`
    // block, the shape the genre fence composes with a TOML parse.
    let unit = memory_unit(
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
```toml genre.manifest\n\
name = \"coordinate\"\n\
```\n",
    );
    let features = extraction.extract(&unit);

    // Each block yields its interior content (surrounding prose skipped) keyed by its
    // info string, in document order.
    assert_eq!(features.fenced_blocks.len(), 2);
    assert_eq!(features.fenced_blocks[0].info, "sh");
    assert_eq!(features.fenced_blocks[0].content, "cargo test");
    assert_eq!(features.fenced_blocks[1].info, "toml genre.manifest");
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
    let unit = memory_unit("# Kinds\n\nJust prose, no fenced block at all.\n");
    assert!(extraction.extract(&unit).fenced_blocks.is_empty());
}

/// A `Unit` carrying the given parsed frontmatter (the nested shape the
/// yaml-frontmatter adapter yields from a nested YAML map / a JSON-manifest settings
/// kind), for driving a composed `field` extractor over a key-path without touching
/// disk. The body is empty — the key-path case exercises the frontmatter locus only.
fn frontmatter_unit(frontmatter: serde_json::Map<String, serde_json::Value>) -> Unit {
    Unit {
        id: "settings".to_string(),
        frontmatter: frontmatter.into_iter().collect(),
        body: String::new(),
        source_path: PathBuf::from("settings/settings.md"),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
        published_requirements: Vec::new(),
    }
}

/// A key-path `field` primitive walks nested frontmatter tables to the leaf — the
/// traversal its doc-comment promises, the settings kind's
/// nested-key consumer. The leaf preserves its source scalar kind, and an unresolved
/// path is **absent, never errored** — a missing segment or a scalar met before the
/// leaf yields no feature, exactly as an unset optional field does.
#[test]
fn a_field_primitive_reads_a_nested_key_path_over_a_units_frontmatter() {
    let extraction = Extraction::new(vec![
        Primitive::Field {
            key: "permissions.defaultMode".to_string(),
        },
        Primitive::Field {
            key: "permissions.retries".to_string(),
        },
        Primitive::Field {
            key: "name".to_string(),
        },
        Primitive::Field {
            key: "permissions.missing.leaf".to_string(),
        },
        Primitive::Field {
            key: "name.nope".to_string(),
        },
    ]);

    // A JSON-manifest settings shape: a nested `permissions` table over a top-level
    // scalar, the exact carriage the key-path half serves (entry note, 07-04).
    let serde_json::Value::Object(frontmatter) = serde_json::json!({
        "name": "settings",
        "permissions": {
            "defaultMode": "acceptEdits",
            "retries": 3
    }
    }) else {
        unreachable!("the fixture is a JSON object")
    };
    let unit = frontmatter_unit(frontmatter);

    let features = extraction.extract(&unit);

    // The nested string leaf reads over the `a.b` key-path, keyed by the whole path,
    // preserving its source scalar kind (`string`, not a collapsed container).
    assert_eq!(
        features.field("permissions.defaultMode"),
        Some(&FeatureValue::scalar(ValueType::String, "acceptEdits"))
    );
    // A nested integer leaf keeps `integer` — the source kind survives the walk.
    assert_eq!(
        features
            .field("permissions.retries")
            .map(FeatureValue::kind),
        Some(ValueType::Integer)
    );
    // A bare key stays the flat lookup — the common case is unchanged.
    assert_eq!(
        features.field("name"),
        Some(&FeatureValue::scalar(ValueType::String, "settings"))
    );

    // Absent, never errored: a missing segment past a real table, and a scalar met
    // before the leaf (`name` is a string, so `name.nope` has no sub-key) both yield
    // no feature — extraction stays total.
    assert!(features.field("permissions.missing.leaf").is_none());
    assert!(features.field("name.nope").is_none());

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
    let unit = rule_surface_unit(&rule, "rust");
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
    let unit = rule_surface_unit(&rule, "collaboration");
    let features = builtin_kind::rule_features(&unit);
    insta::assert_debug_snapshot!("rule_features_collaboration", features);
}
