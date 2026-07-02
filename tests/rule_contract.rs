//! Pins the shipped rule built-in package (`specs/10-contracts.md`, "Packages —
//! best practices as data").
//!
//! `packages/rule.anthropic/PACKAGE.md` is the std-lib contract for the `rule`
//! artifact kind (`.claude/rules/*.md`) — curated product source the build *embeds*
//! (`specs/10-contracts.md`, the `contracts/` retirement) and resolves by name. This
//! test loads it through the same embedded path the shipped `check` uses
//! ([`temper::builtin::contract`]) and pins the exact decidable clause vector it
//! carries, mirroring `tests/contract_template.rs` for the skill built-in.
//!
//! Two facts are load-bearing here beyond the skill mirror: the package is now named
//! `rule.anthropic` (renamed from the bare `rule` — `specs/10-contracts.md`, "named
//! for its source"), and its clauses carry `source` citations, the expected posture
//! for a built-in whose legitimacy is *sourced* opinion. The clause *vocabulary* is
//! pinned; the guidance/citation prose is product territory, so it is asserted to be
//! present, not pinned verbatim.

use std::collections::BTreeSet;

use temper::contract::{Contract, Predicate, Severity};
use temper::engine;

/// The built-in rule contract, resolved from the embedded `packages/` std-lib the
/// same way the shipped tool resolves it.
fn rule_builtin() -> Contract {
    temper::builtin::contract(temper::builtin::RULE_PACKAGE)
        .expect("the embedded rule package should parse")
        .expect("the rule package is embedded")
}

/// The decidable `(severity, predicate)` vector the rule built-in must carry, in
/// declaration order — `optional` on `paths` (advisory), the Cursor-key
/// `forbidden_keys` (required), the lean-rule `max_lines` (advisory). Guidance and
/// `source` ride each clause but are product prose, so they are excluded from this
/// structural pin.
fn expected_clauses() -> Vec<(Severity, Predicate)> {
    vec![
        (
            Severity::Advisory,
            Predicate::Optional {
                field: "paths".to_string(),
            },
        ),
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
/// declared severities, and its display name derives to `rule.anthropic` from the
/// package directory (`specs/10-contracts.md`, "named for its source": the rename
/// lands with the embed).
#[test]
fn rule_builtin_carries_the_decidable_clause_vector() {
    let contract = rule_builtin();
    assert_eq!(contract.name, "rule.anthropic");

    let clauses: Vec<(Severity, Predicate)> = contract
        .clauses
        .iter()
        .map(|clause| (clause.severity, clause.predicate.clone()))
        .collect();
    assert_eq!(clauses, expected_clauses());
}

/// A built-in package's clauses are *cited* — each carries a `source` provenance of
/// taste (`specs/10-contracts.md`, "a built-in package's clauses ... it is the
/// expected posture"). Pinning the presence keeps the update ritual honest (walk the
/// clauses, re-check their citations) without coupling to the citation text.
#[test]
fn every_rule_builtin_clause_is_cited() {
    for clause in &rule_builtin().clauses {
        assert!(
            clause.source.is_some(),
            "a built-in clause must carry its source citation, got: {:?}",
            clause.predicate,
        );
    }
}

/// No undecidable clause survives. The rule built-in — like the skill one — encodes
/// *only* decidable predicates: every clause is a true/false fact over the artifact,
/// never a semantic guess (`specs/10-contracts.md`, "best practices as data").
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
        BTreeSet::from(["optional", "forbidden_keys", "max_lines"]),
        "the rule built-in must carry only its declared decidable predicates",
    );
}

/// The rule built-in is itself admissible — it passes the second green
/// (`specs/10-contracts.md`, "Decision: the contract is itself checked"). It carries
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
