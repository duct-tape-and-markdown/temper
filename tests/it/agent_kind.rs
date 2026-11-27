//! The `agent` built-in kind: a subagent definition (`specs/builtins.md`, "The
//! shipped kinds").
//!
//! Discovery folds `.claude/agents/**/*.md` recursively into `agent` members whose
//! id is the frontmatter `name` — the third identity mode (named-field), never the
//! filename or a containing subdirectory (organizational only). Driven at the
//! crate-public API a real `import`/`check` read takes — `import::discover_kind_files`,
//! `Member::from_source`, `builtin_kind::features`, and `temper::builtin::contract` +
//! `engine::validate` for the floor's name/uniqueness clauses — over fixtures
//! mirroring the real Claude Code layout (`.claude/rules/rust.md`, "Harness-input
//! fixtures mirror the real Claude Code layout").

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use crate::common;

use temper::builtin_kind;
use temper::contract::{CharPosition, Charset, Predicate};
use temper::engine;
use temper::extract::FeatureValue;
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
    builtin_kind::definition("agent").expect("agent is embedded")
}

#[test]
fn discovery_over_the_embedded_governs_finds_nested_agent_files() {
    let harness = common::tmpdir("discover");
    write_agent(&harness, "coordinate.md", "coordinate");
    // A subdirectory is purely organizational — still discovered, per the docs'
    // own `agents/review/`, `agents/research/` example.
    write_agent(&harness, "review/reviewer.md", "reviewer");

    let kind = agent_kind();
    let found = import::discover_kind_files(
        &import::Discovery::new(&harness),
        &kind,
        kind.governs.as_ref().unwrap(),
        import::LocalOverride::Honored,
    );

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
        features
            .field("name")
            .as_ref()
            .and_then(FeatureValue::as_scalar),
        Some("reviewer")
    );
    assert_eq!(
        features
            .field("description")
            .as_ref()
            .and_then(FeatureValue::as_scalar),
        Some("Use when reviewing a pull request for correctness.")
    );
}

#[test]
fn agent_builtin_carries_the_decidable_clause_vector() {
    let contract = common::builtin_floor("agent");
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
            Predicate::MaxLen {
                field: "name".to_string(),
                max: 256,
            },
            Predicate::AllowedChars {
                field: "name".to_string(),
                charset: Charset {
                    ranges: Vec::new(),
                    chars: BTreeSet::from([':']),
                    negated: true,
                    position: CharPosition::Every,
                },
            },
            Predicate::AllowedChars {
                field: "name".to_string(),
                charset: Charset {
                    ranges: Vec::new(),
                    chars: BTreeSet::from(['-']),
                    negated: true,
                    position: CharPosition::Leading,
                },
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
    let diagnostics =
        engine::admissibility(&common::builtin_floor("agent"), &engine::Locus::Document);
    assert!(diagnostics.is_empty(), "got: {diagnostics:?}");
}

/// Every diagnostic the agent floor raises over one member named `name`.
fn name_diagnostics(name: &str) -> Vec<temper::check::Diagnostic> {
    let harness = common::tmpdir("name-clauses");
    let source = write_agent(&harness, "agent.md", name);

    let kind = agent_kind();
    let member = Member::from_source(&kind, &source).unwrap();
    let unit = common::surface_unit(&member);
    let features = builtin_kind::features(&kind, &unit, &[]);

    engine::validate(&common::builtin_floor("agent"), &[features])
}

#[test]
fn an_agent_named_after_an_overridable_built_in_draws_no_name_finding() {
    let diagnostics = name_diagnostics("Explore");
    assert!(
        diagnostics.is_empty(),
        "a project `Explore` overrides the built-in and must check clean, got: {diagnostics:#?}"
    );
}

#[test]
fn a_name_with_digits_draws_no_name_finding() {
    let diagnostics = name_diagnostics("reviewer-v2");
    assert!(diagnostics.is_empty(), "got: {diagnostics:#?}");
}

#[test]
fn an_agent_named_explore_passes_the_name_clauses() {
    let diagnostics = name_diagnostics("explore");
    assert!(diagnostics.is_empty(), "got: {diagnostics:#?}");
}

#[test]
fn an_agent_name_containing_a_colon_fails_check() {
    let diagnostics = name_diagnostics("my:reviewer");
    let rules: Vec<&str> = diagnostics.iter().map(|d| d.rule.as_str()).collect();
    assert_eq!(
        rules,
        vec!["agent.allowed-chars.name=negated"],
        "got: {diagnostics:#?}"
    );
}

#[test]
fn an_agent_name_with_a_leading_dash_fails_check() {
    let diagnostics = name_diagnostics("-reviewer");
    let rules: Vec<&str> = diagnostics.iter().map(|d| d.rule.as_str()).collect();
    assert_eq!(
        rules,
        vec!["agent.allowed-chars.name=leading+negated"],
        "got: {diagnostics:#?}"
    );
}

#[test]
fn a_name_past_256_characters_trips_the_length_clause() {
    let diagnostics = name_diagnostics(&"a".repeat(257));
    assert!(
        diagnostics.iter().any(|d| d.rule == "agent.max-len.name"),
        "got: {diagnostics:#?}"
    );
    assert!(name_diagnostics(&"a".repeat(256)).is_empty());
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

    let contract = common::builtin_floor("agent");
    let diagnostics = engine::validate(&contract, &[first_features, second_features]);

    let collisions: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule == "agent.unique-name")
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

    let contract = common::builtin_floor("agent");
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

#[test]
fn an_agent_whose_frontmatter_yaml_does_not_parse_keeps_the_load_fault() {
    let harness = common::tmpdir("non-portable-agent");
    let path = harness.join(".claude/agents/reviewer.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        &path,
        "---\nname: reviewer\ndescription: Use when: reviewing a pull request\n---\n# Reviewer\n",
    )
    .unwrap();

    assert!(matches!(
        Member::from_source(&agent_kind(), &path),
        Err(temper::frontmatter::FrontmatterError::Malformed { .. })
    ));

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    assert_eq!(
        common::findings_for(&findings, "member.load-fault").len(),
        1,
        "the runtime skips the agent, so the read faults: {findings:#?}"
    );
    assert!(common::findings_for(&findings, "member.non-portable-yaml").is_empty());
}
