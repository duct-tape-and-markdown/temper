//! Behavioral contract for the slice-1 skill lint rules
//! (`spec/RELEASE-v0.1.md`, "Check behavior (the lint engine)").
//!
//! Each rule owns a deliberately-broken fixture under `tests/fixtures/rules/`
//! plus the `clean` control. The fixtures are *source* skill directories (a
//! `SKILL.md` with frontmatter); loading one through [`Skill::from_source_dir`]
//! and wrapping it in a [`Workspace`] exercises the rules over the real IR,
//! including `name-matches-dir`, which reads the directory off the provenance
//! path.
//!
//! The fixtures are tuned so each broken one trips *exactly* its target rule
//! (the engine's "one root cause, one diagnostic" discipline), which lets the
//! table below assert the full fired-rule set, not just membership.

use std::collections::BTreeSet;
use std::path::Path;

use temper::check::{Diagnostic, Severity, Workspace, run};
use temper::rules::all_rules;
use temper::skill::Skill;

/// Load a fixture skill directory and run the full rule set over it.
fn diagnostics_for(fixture: &str) -> Vec<Diagnostic> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rules")
        .join(fixture);
    let skill = Skill::from_source_dir(&dir).expect("fixture skill should parse");
    let ws = Workspace {
        skills: vec![skill],
    };
    run(&ws, &all_rules())
}

/// The set of distinct rule ids that fired.
fn fired_rules(diagnostics: &[Diagnostic]) -> BTreeSet<&str> {
    diagnostics.iter().map(|d| d.rule.as_str()).collect()
}

/// Every rule fires on its own broken fixture — and only that rule — at the
/// severity the spec table assigns it.
#[test]
fn each_rule_fires_on_its_fixture_at_the_right_severity() {
    let cases = [
        (
            "frontmatter-valid",
            "skill.frontmatter-valid",
            Severity::Error,
        ),
        ("Name-Format", "skill.name-format", Severity::Error),
        (
            "name-matches-dir",
            "skill.name-matches-dir",
            Severity::Error,
        ),
        (
            "description-length",
            "skill.description-length",
            Severity::Error,
        ),
        (
            "description-third-person",
            "skill.description-third-person",
            Severity::Warn,
        ),
        (
            "description-has-trigger",
            "skill.description-has-trigger",
            Severity::Warn,
        ),
        (
            "description-has-anti-trigger",
            "skill.description-has-anti-trigger",
            Severity::Warn,
        ),
        ("body-length", "skill.body-length", Severity::Warn),
        (
            "companion-refs-resolve",
            "skill.companion-refs-resolve",
            Severity::Error,
        ),
        (
            "refs-one-level-deep",
            "skill.refs-one-level-deep",
            Severity::Warn,
        ),
    ];

    for (fixture, rule, severity) in cases {
        let diagnostics = diagnostics_for(fixture);
        assert_eq!(
            fired_rules(&diagnostics),
            BTreeSet::from([rule]),
            "fixture `{fixture}` should trip exactly `{rule}`, got {diagnostics:?}",
        );
        for diagnostic in &diagnostics {
            assert_eq!(
                diagnostic.severity, severity,
                "`{rule}` should be {severity:?}",
            );
        }
    }
}

/// The clean control trips no rule — proving every rule stays silent on valid
/// input, not merely that it can fire.
#[test]
fn clean_fixture_is_silent() {
    let diagnostics = diagnostics_for("clean");
    assert!(
        diagnostics.is_empty(),
        "clean fixture should be silent, got {diagnostics:?}",
    );
}

/// A body referencing a file that is not on disk is an *error*, distinct from
/// the depth advisory — the contract the entry calls out explicitly.
#[test]
fn missing_companion_reference_is_an_error() {
    let diagnostics = diagnostics_for("companion-refs-resolve");
    let missing = diagnostics
        .iter()
        .find(|d| d.rule == "skill.companion-refs-resolve")
        .expect("the missing reference should be reported");
    assert_eq!(missing.severity, Severity::Error);
    assert!(
        missing.message.contains("MISSING.md"),
        "the message should name the unresolved path, got {:?}",
        missing.message,
    );
}
