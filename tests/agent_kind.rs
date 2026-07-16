//! The `agent` built-in kind: a subagent definition (`specs/builtins.md`, "The
//! shipped kinds").
//!
//! Discovery folds `.claude/agents/**/*.md` recursively into `agent` members whose
//! id is the frontmatter `name` — the third identity mode (named-field), never the
//! filename or a containing subdirectory (organizational only). Driven at the
//! crate-public API a real `import`/`check` read takes — `import::discover_kind_files`,
//! `Member::from_source`, `builtin_kind::features`, and `builtin::contract` +
//! `engine::validate` for the floor's charset/uniqueness clauses — over fixtures
//! mirroring the real Claude Code layout (`.claude/rules/rust.md`, "Harness-input
//! fixtures mirror the real Claude Code layout").

use std::fs;
use std::path::PathBuf;

mod common;

use temper::builtin;
use temper::builtin_kind;
use temper::contract::{Charset, Predicate};
use temper::engine;
use temper::frontmatter::Member;
use temper::import;
use temper::kind::{Registration, UnitShape};

/// An agent file in the real Claude Code shape: YAML frontmatter carrying `name`
/// then `description` over a markdown system-prompt body.
fn agent_source(name: &str) -> String {
    format!(
        "---\nname: {name}\ndescription: Use when reviewing a pull request for correctness.\n---\n# Reviewer\n\nReview the diff.\n"
    )
}

/// Write an agent member at `<root>/.claude/agents/<relative>` — the real Claude
/// Code locus (`.claude/agents/**/*.md`), never a layout invented for the test.
fn write_agent(root: &std::path::Path, relative: &str, name: &str) -> PathBuf {
    let path = root.join(".claude").join("agents").join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, agent_source(name)).unwrap();
    path
}

fn agent_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("agent")
        .unwrap()
        .expect("agent is embedded")
}

#[test]
fn discovery_over_the_embedded_governs_finds_nested_agent_files() {
    let harness = common::tmpdir("discover");
    write_agent(&harness, "coordinate.md", "coordinate");
    // A subdirectory is purely organizational — still discovered, per the docs'
    // own `agents/review/`, `agents/research/` example.
    write_agent(&harness, "review/reviewer.md", "reviewer");

    let kind = agent_kind();
    let found =
        import::discover_kind_files(&harness, &kind, kind.governs.as_ref().unwrap()).unwrap();

    assert_eq!(
        found,
        vec![
            harness.join(".claude/agents/coordinate.md"),
            harness.join(".claude/agents/review/reviewer.md"),
        ]
    );
}

#[test]
fn an_agents_id_is_its_frontmatter_name_not_the_filename() {
    let harness = common::tmpdir("named-field-id");
    // The filename (`foo.md`) deliberately differs from the frontmatter `name`
    // (`my-agent`) — identity travels on the declared field, never the path.
    let source = write_agent(&harness, "foo.md", "my-agent");

    let member = Member::from_source(&agent_kind(), &source).unwrap();

    assert_eq!(member.id, "my-agent");
}

#[test]
fn a_nested_agents_id_is_still_its_frontmatter_name() {
    let harness = common::tmpdir("nested-named-field-id");
    let source = write_agent(&harness, "review/security.md", "security-reviewer");

    let member = Member::from_source(&agent_kind(), &source).unwrap();

    assert_eq!(member.id, "security-reviewer");
}

#[test]
fn the_agent_kind_declares_the_named_field_identity_mode() {
    assert_eq!(
        agent_kind().unit_shape,
        Some(UnitShape::NamedField {
            field: "name".to_string()
        })
    );
}

#[test]
fn an_agent_member_registers_on_the_description_trigger_channel_only() {
    // No user-invoked channel: an agent is delegated to by description, not
    // invoked as a `/name` slash command (`specs/builtins.md`, "The shipped kinds").
    assert_eq!(
        agent_kind().registration,
        vec![Registration::DescriptionTrigger {
            field: "description".to_string()
        }]
    );
}

#[test]
fn an_agent_member_extracts_its_declared_field_schema() {
    let harness = common::tmpdir("field-schema");
    let source = write_agent(&harness, "reviewer.md", "reviewer");

    let kind = agent_kind();
    let member = Member::from_source(&kind, &source).unwrap();
    let unit = common::surface_unit(&member);
    let features = builtin_kind::features(&kind, &unit, &[]);

    assert_eq!(
        features.field("name").and_then(|f| f.as_scalar()),
        Some("reviewer")
    );
    assert_eq!(
        features.field("description").and_then(|f| f.as_scalar()),
        Some("Use when reviewing a pull request for correctness.")
    );
}

/// The `[a-z-]` charset the `name` `allowed_chars` clause declares — lowercase
/// letters and hyphens only, no digits (unlike a skill's `[a-z0-9-]` name).
fn agent_name_charset() -> Charset {
    Charset {
        ranges: vec![('a', 'z')],
        chars: std::collections::BTreeSet::from(['-']),
    }
}

#[test]
fn agent_builtin_carries_the_decidable_clause_vector() {
    let contract = builtin::contract("agent").expect("the agent floor is embedded");
    assert_eq!(contract.name, "agent");

    let predicates: Vec<Predicate> = contract
        .clauses
        .iter()
        .map(|clause| clause.predicate.clone())
        .collect();
    assert_eq!(
        predicates,
        vec![
            Predicate::Required {
                field: "name".to_string()
            },
            Predicate::AllowedChars {
                field: "name".to_string(),
                charset: agent_name_charset(),
            },
            Predicate::UniqueName,
            Predicate::Required {
                field: "description".to_string()
            },
        ]
    );
    for clause in &contract.clauses {
        assert!(
            clause.guidance.is_some(),
            "{:?} carries no guidance",
            clause.predicate
        );
        assert!(
            clause.source.is_some(),
            "{:?} carries no source cite",
            clause.predicate
        );
    }
}

#[test]
fn the_agent_builtin_is_admissible() {
    let diagnostics = engine::admissibility(
        &builtin::contract("agent").unwrap(),
        &engine::Locus::Document,
    );
    assert!(diagnostics.is_empty(), "got: {diagnostics:?}");
}

#[test]
fn an_uppercase_name_trips_the_charset_clause() {
    let harness = common::tmpdir("bad-charset");
    let source = write_agent(&harness, "reviewer.md", "Reviewer");

    let kind = agent_kind();
    let member = Member::from_source(&kind, &source).unwrap();
    let unit = common::surface_unit(&member);
    let features = builtin_kind::features(&kind, &unit, &[]);

    let contract = builtin::contract("agent").unwrap();
    let diagnostics = engine::validate(&contract, &[features]);

    assert!(
        diagnostics.iter().any(|d| d.rule == "allowed_chars"),
        "an uppercase name must trip the charset clause, got: {diagnostics:#?}"
    );
}

#[test]
fn a_lowercase_hyphenated_name_trips_no_charset_clause() {
    let harness = common::tmpdir("good-charset");
    let source = write_agent(&harness, "code-reviewer.md", "code-reviewer");

    let kind = agent_kind();
    let member = Member::from_source(&kind, &source).unwrap();
    let unit = common::surface_unit(&member);
    let features = builtin_kind::features(&kind, &unit, &[]);

    let contract = builtin::contract("agent").unwrap();
    let diagnostics = engine::validate(&contract, &[features]);

    assert!(
        diagnostics.iter().all(|d| d.rule != "allowed_chars"),
        "a clean lowercase-hyphenated name must not trip the charset clause, got: {diagnostics:#?}"
    );
}

#[test]
fn two_agents_sharing_a_name_in_one_scope_trip_the_uniqueness_clause() {
    let harness = common::tmpdir("duplicate-name");
    let kind = agent_kind();

    // A raw `Unit` straight off each imported `Member`, skipping the surface
    // round-trip: two independently-imported same-named members would project to
    // the *same* surface directory (a real collision the import pipeline handles
    // elsewhere), so exercising `engine::validate` directly over two `Unit`s
    // sharing an `id` is how the uniqueness clause's per-artifact scope is driven
    // here.
    let first_source = write_agent(&harness, "one.md", "reviewer");
    let first_member = Member::from_source(&kind, &first_source).unwrap();
    let first_features = builtin_kind::features(
        &kind,
        &common::raw_unit(
            &first_member.id,
            first_member.fields.iter().cloned().collect(),
            &first_member.body,
            first_member.provenance.source_path.to_str().unwrap(),
        ),
        &[],
    );

    let second_source = write_agent(&harness, "review/two.md", "reviewer");
    let second_member = Member::from_source(&kind, &second_source).unwrap();
    let second_features = builtin_kind::features(
        &kind,
        &common::raw_unit(
            &second_member.id,
            second_member.fields.iter().cloned().collect(),
            &second_member.body,
            second_member.provenance.source_path.to_str().unwrap(),
        ),
        &[],
    );

    let contract = builtin::contract("agent").unwrap();
    let diagnostics = engine::validate(&contract, &[first_features, second_features]);

    let collisions: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule == "unique-name")
        .collect();
    assert_eq!(
        collisions.len(),
        2,
        "both members sharing the name must each fire the uniqueness clause, got: {diagnostics:#?}"
    );
    assert!(collisions.iter().all(|d| d.message.contains("not unique")));
}

#[test]
fn two_agents_with_distinct_names_trip_no_uniqueness_clause() {
    let harness = common::tmpdir("distinct-names");
    let kind = agent_kind();

    let first_source = write_agent(&harness, "one.md", "reviewer");
    let first_member = Member::from_source(&kind, &first_source).unwrap();
    let first_features = builtin_kind::features(
        &kind,
        &common::raw_unit(
            &first_member.id,
            first_member.fields.iter().cloned().collect(),
            &first_member.body,
            first_member.provenance.source_path.to_str().unwrap(),
        ),
        &[],
    );

    let second_source = write_agent(&harness, "two.md", "planner");
    let second_member = Member::from_source(&kind, &second_source).unwrap();
    let second_features = builtin_kind::features(
        &kind,
        &common::raw_unit(
            &second_member.id,
            second_member.fields.iter().cloned().collect(),
            &second_member.body,
            second_member.provenance.source_path.to_str().unwrap(),
        ),
        &[],
    );

    let contract = builtin::contract("agent").unwrap();
    let diagnostics = engine::validate(&contract, &[first_features, second_features]);

    assert!(
        diagnostics.iter().all(|d| d.rule != "unique-name"),
        "distinct names must not trip the uniqueness clause, got: {diagnostics:#?}"
    );
}

#[test]
fn a_source_missing_the_name_field_is_a_load_error() {
    let harness = common::tmpdir("no-name");
    let dir = harness.join(".claude").join("agents");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("nameless.md");
    fs::write(
        &path,
        "---\ndescription: No name field at all.\n---\n# Nameless\n",
    )
    .unwrap();

    let err = Member::from_source(&agent_kind(), &path).unwrap_err();
    assert!(matches!(
        err,
        temper::frontmatter::FrontmatterError::NoNamedFieldId { .. }
    ));
}
