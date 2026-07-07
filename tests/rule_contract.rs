//! Pins the shipped rule built-in floor (`specs/model/contract.md`, "Packages —
//! best practices as data").
//!
//! The `rule` floor is a projection of the embedded built-in lock's clause rows
//! (`specs/distribution.md`, "Decision: the built-in lock is derived
//! from the SDK module, never transcribed") — never a hand-written mirror. This
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
use temper::engine;

/// The built-in rule contract, resolved from the embedded built-in lock the same
/// way the shipped tool resolves it.
fn rule_builtin() -> Contract {
    temper::builtin::contract("rule").expect("the rule floor is embedded")
}

/// The decidable `(severity, predicate)` vector the rule built-in must carry, in
/// declaration order — the Cursor-key `forbidden_keys` (required), the lean-rule
/// `max_lines` (advisory). Guidance and `source` ride each clause but are product
/// prose, so they are excluded from this structural pin. No `optional` clause over
/// `paths`: the SDK floor asserts nothing decidable for an optional field, so the
/// lock carries no such row and this projection carries none either.
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
        (Severity::Advisory, Predicate::MaxLines { max: 200 }),
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
/// provenance of taste with the hover-sized why (`specs/model/contract.md`,
/// "a built-in package's clauses ... it is the expected posture"). Pinning the
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
/// never a semantic guess (`specs/model/contract.md`, "best practices as data").
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
        BTreeSet::from(["forbidden_keys", "max_lines"]),
        "the rule built-in must carry only its declared decidable predicates",
    );
}

/// The rule built-in is itself admissible — it passes the second green
/// (`specs/model/contract.md`, "Decision: the contract is itself checked"). It carries
/// only closed-vocabulary clauses with no vacuous list, so `engine::admissibility`
/// returns nothing.
#[test]
fn the_rule_builtin_is_admissible() {
    let diagnostics = engine::admissibility(&rule_builtin());
    assert!(
        diagnostics.is_empty(),
        "the rule built-in should be admissible, got: {diagnostics:?}",
    );
}
