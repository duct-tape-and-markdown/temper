//! Pins the shipped rule built-in floor.
//!
//! The `rule` floor is a projection of the embedded built-in lock's clause rows
//! — never a hand-written mirror. This
//! test loads it through the same embedded path the shipped `check` uses
//! ([`temper::builtin::contract`]) and pins the exact decidable clause vector it
//! carries, mirroring `tests/contract_template.rs` for the skill built-in.
//!
//! The floor is named for its kind, `rule` — not a `<kind>.<source>` package — and
//! its clauses carry `source` citations and `guidance`, the full four channels a
//! clause row projects losslessly. The clause *vocabulary* is pinned; the
//! guidance/citation prose is product territory, so it is asserted to be present,
//! not pinned verbatim.

use std::collections::BTreeSet;

use temper::contract::{Contract, Predicate, Severity};
use temper::drift::Declarations;
use temper::engine;

mod common;

/// The built-in rule contract, resolved from the embedded built-in lock the same
/// way the shipped tool resolves it.
fn rule_builtin() -> Contract {
    temper::builtin::contract("rule").expect("the rule floor is embedded")
}

/// The decidable `(severity, predicate)` vector the rule built-in must carry, in
/// declaration order — the Cursor-key `forbidden_keys` (required), the `paths`
/// `glob-valid` (required), the lean-rule `max_lines` (advisory), the
/// `mention-reachable` gate check (advisory). Guidance and
/// `source` ride each clause but are product prose, so they are excluded from this
/// structural pin. No `optional` clause over `paths`: the SDK floor asserts nothing
/// decidable for an optional field's *presence*, so the lock carries no such row —
/// but `glob-valid` over `paths` is decidable content, so it does.
fn expected_clauses() -> Vec<(Severity, Predicate)> {
    vec![
        (
            Severity::Required,
            Predicate::ForbiddenKeys {
                keys: vec![
                    "description".to_string(),
                    "globs".to_string(),
                    "alwaysApply".to_string(),
                ],
            },
        ),
        (
            Severity::Required,
            Predicate::GlobValid {
                field: "paths".to_string(),
            },
        ),
        (Severity::Advisory, Predicate::MaxLines { max: 200 }),
        (
            Severity::Advisory,
            Predicate::MentionReachable {
                scope_field: "paths".to_string(),
                gate_field: "paths".to_string(),
            },
        ),
    ]
}

/// The embedded rule built-in carries exactly the decidable clause vector at its
/// declared severities, and its display name is its bare kind label, `rule` — the
/// projection's argument payload (the `forbidden_keys` list, the `max_lines` bound)
/// survives unchanged.
#[test]
fn rule_builtin_carries_the_decidable_clause_vector() {
    let contract = rule_builtin();
    assert_eq!(contract.name, "rule");

    let clauses: Vec<(Severity, Predicate)> = contract
        .clauses
        .iter()
        .map(|clause| (clause.severity, clause.predicate.clone()))
        .collect();
    assert_eq!(clauses, expected_clauses());
}

/// A built-in floor's clauses are *guided and cited* — each pairs a `source`
/// provenance of taste with the hover-sized why. Pinning the
/// presence keeps the update ritual honest (walk the clauses, re-check their
/// citations) without coupling to the citation text — and proves both channels
/// survive the row projection, not just the predicate.
#[test]
fn every_rule_builtin_clause_is_guided_and_cited() {
    for clause in &rule_builtin().clauses {
        assert!(
            clause.guidance.is_some(),
            "a built-in clause must carry its guidance, got: {:?}",
            clause.predicate,
        );
        assert!(
            clause.source.is_some(),
            "a built-in clause must carry its source citation, got: {:?}",
            clause.predicate,
        );
    }
}

/// No undecidable clause survives. The rule built-in — like the skill one — encodes
/// *only* decidable predicates: every clause is a true/false fact over the artifact,
/// never a semantic guess.
#[test]
fn rule_builtin_encodes_only_decidable_clauses() {
    let contract = rule_builtin();

    let kinds: BTreeSet<&str> = contract
        .clauses
        .iter()
        .map(|clause| clause.predicate.key())
        .collect();

    assert_eq!(
        kinds,
        BTreeSet::from([
            "forbidden_keys",
            "glob-valid",
            "max_lines",
            "mention-reachable",
        ]),
        "the rule built-in must carry only its declared decidable predicates",
    );
}

/// The shipped rule floor's `glob-valid` clause fires over a `paths` carrying an
/// unparseable glob and stays silent over a valid brace-expansion glob — the
/// decision-0022 acceptance, exercised end to end against the real embedded floor
/// (not a synthetic one-clause contract).
#[test]
fn the_rule_floor_glob_valid_clause_fires_on_an_unparseable_paths_glob() {
    use temper::check::Severity as FindingSeverity;
    use temper::engine;

    // A rule whose `paths` carries an unparseable glob (`[` opens a character class
    // that never closes) alongside a well-formed one — only the broken entry is a
    // finding.
    let broken = rule_features(&["src/**/*.rs", "["]);
    let diagnostics = engine::validate(&rule_builtin(), std::slice::from_ref(&broken));
    let glob_findings: Vec<&temper::check::Diagnostic> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.rule == "glob-valid")
        .collect();
    assert_eq!(
        glob_findings.len(),
        1,
        "exactly the one unparseable glob fires, got: {diagnostics:?}",
    );
    // The clause ships at `required`, so the finding blocks the gate.
    assert_eq!(glob_findings[0].severity, FindingSeverity::Error);

    // A valid glob (brace expansion included) passes — no `glob-valid` finding.
    let valid = rule_features(&["src/**/*.{rs,toml}", "docs/*.md"]);
    let clean = engine::validate(&rule_builtin(), std::slice::from_ref(&valid));
    assert!(
        clean
            .iter()
            .all(|diagnostic| diagnostic.rule != "glob-valid"),
        "a valid brace-expansion glob must not fire glob-valid, got: {clean:?}",
    );
}

/// A `Features` carrying only a `paths` list — the one field the rule floor's
/// `glob-valid` clause reads.
fn rule_features(paths: &[&str]) -> temper::extract::Features {
    use std::collections::BTreeMap;
    use temper::extract::{FeatureValue, Features};

    let mut fields = BTreeMap::new();
    fields.insert(
        "paths".to_string(),
        FeatureValue::List(paths.iter().map(|glob| (*glob).to_string()).collect()),
    );
    Features {
        id: "demo".to_string(),
        fields,
        body_lines: 1,
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: Vec::new(),
        edge_placements: None,
    }
}

/// Drive a bare harness through the shipped floor: a rule `style` scoped by
/// `rule_paths` mentioning a skill `standards` gated by `skill_paths`. The lock carries
/// the mention row and **nothing else** — no requirement, no `satisfies` — so whatever
/// fires here fires off the embedded `rule` default contract alone. That absence is the
/// acceptance: coverage with zero opt-in.
fn bare_harness_run(
    slug: &str,
    rule_paths: Option<&str>,
    skill_paths: Option<&str>,
) -> common::CheckRun {
    let root = common::tmpdir(slug);
    common::write_rule_skill_harness(
        &root,
        "style",
        &common::scoped_rule(rule_paths),
        "standards",
        &common::gated_skill("standards", skill_paths),
    );
    common::write_lock(
        &root,
        Declarations {
            mentions: vec![common::mention("rule:style", "skill:standards")],
            ..Declarations::default()
        },
    );

    common::check_in(&root, &[], None)
}

/// The shipped floor's `mention-reachable` clause fires on a rule whose `paths` sit
/// outside the gate of the skill it mentions — with no opt-in anywhere. The rule loads
/// under `src/**`; the skill is invocable only once a `docs/**` file is read, and
/// invoking it outside that gate hard-errors. The finding is **advisory**, so the gate
/// does not block: literal containment can be wrong, and a check that can be wrong must
/// not fail a run (0028; intent.md invariant 5 leaves escalation to the corpus).
#[test]
fn the_rule_floor_flags_a_mention_of_a_gated_skill_with_zero_opt_in() {
    let run = bare_harness_run("rule-floor-mr", Some("src/**"), Some("docs/**"));
    assert!(
        run.output.contains("mention-reachable") && run.output.contains("standards"),
        "the shipped rule floor alone must flag the unreachable mention, got:\n{}",
        run.output
    );
    assert!(
        run.ok,
        "the clause ships advisory — literal containment can be wrong, so the finding must \
         not block the run ⇒ zero, got:\n{}",
        run.output
    );
}

/// The floor's clause stays silent where the mention is reachable: the rule's every
/// scope glob appears verbatim in the skill's gate, so the mention fires only where the
/// skill can be invoked. Pins that the default contract is not a blanket alarm on any
/// harness that mentions a gated skill.
#[test]
fn the_rule_floor_is_silent_when_the_mention_sits_inside_the_targets_gate() {
    let run = bare_harness_run("rule-floor-mr-clean", Some("docs/**"), Some("docs/**"));
    assert!(
        !run.output.contains("mention-reachable"),
        "a scope literally contained in the gate is no finding, got:\n{}",
        run.output
    );
    assert!(run.ok, "and the run is clean ⇒ zero, got:\n{}", run.output);
}

/// The rule built-in is itself admissible — it passes the second green.
/// It carries
/// only closed-vocabulary clauses with no vacuous list, so `engine::admissibility`
/// returns nothing.
#[test]
fn the_rule_builtin_is_admissible() {
    let diagnostics = engine::admissibility(&rule_builtin(), &engine::Locus::Document);
    assert!(
        diagnostics.is_empty(),
        "the rule built-in should be admissible, got: {diagnostics:?}",
    );
}
