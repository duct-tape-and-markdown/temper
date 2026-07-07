//! Pins the shipped Anthropic skill built-in floor (`specs/model/contract.md`,
//! "The clause — the atom of a contract").
//!
//! The `skill` floor is a projection of the embedded built-in lock's clause rows
//! (`specs/distribution.md`, "Decision: the built-in lock is derived
//! from the SDK module, never transcribed"). This test loads it through the same
//! embedded path the shipped `check` uses ([`temper::builtin::contract`]) and pins
//! the exact decidable clause vector it carries.
//!
//! Pinning the vector proves both halves of the registry-kill decision at once. The
//! surviving *decidable* clauses are present at their declared severities, and the
//! *dropped* heuristics (third-person, has-trigger, companion-refs — semantic or
//! weak proxies, out of the closed algebra entirely) are absent, because any extra
//! or missing clause breaks the equality. A floor is named for its kind, not a
//! package — the built-in's display label is `skill`.
//! The clause *vocabulary* is pinned; the guidance/citation prose is product
//! territory, so it is asserted present, not pinned verbatim.

use std::collections::BTreeSet;

use temper::contract::{Charset, Contract, Predicate, Severity};
use temper::engine;

/// The built-in skill contract, resolved from the embedded built-in lock the same
/// way the shipped tool resolves it.
fn skill_builtin() -> Contract {
    temper::builtin::contract("skill").expect("the skill floor is embedded")
}

/// A contract's decidable `(severity, predicate)` vector, in declaration order —
/// the structural pin, excluding the per-clause guidance/`source` prose (product
/// territory, asserted present elsewhere).
fn predicate_vector(contract: &Contract) -> Vec<(Severity, Predicate)> {
    contract
        .clauses
        .iter()
        .map(|clause| (clause.severity, clause.predicate.clone()))
        .collect()
}

/// The `[a-z0-9-]` slug charset the `name` `allowed_chars` clause declares.
fn slug_charset() -> Charset {
    Charset {
        ranges: vec![('a', 'z'), ('0', '9')],
        chars: BTreeSet::from(['-']),
    }
}

/// The decidable clause vector the shipped skill built-in must carry, in declaration
/// order: name required/non-empty/charset/length/deny + matches-dir; description
/// required/non-empty/length; the optional `compatibility` cap; the advisory body
/// budget; the forbidden Cursor keys.
fn expected_skill_clauses() -> Vec<(Severity, Predicate)> {
    vec![
        (
            Severity::Required,
            Predicate::Required {
                field: "name".to_string(),
            },
        ),
        (
            Severity::Required,
            Predicate::MinLen {
                field: "name".to_string(),
                min: 1,
            },
        ),
        (
            Severity::Required,
            Predicate::AllowedChars {
                field: "name".to_string(),
                charset: slug_charset(),
            },
        ),
        (
            Severity::Required,
            Predicate::MaxLen {
                field: "name".to_string(),
                max: 64,
            },
        ),
        (
            Severity::Required,
            Predicate::Deny {
                field: "name".to_string(),
                values: vec!["anthropic".to_string(), "claude".to_string()],
            },
        ),
        (Severity::Required, Predicate::NameMatchesDir),
        (
            Severity::Required,
            Predicate::Required {
                field: "description".to_string(),
            },
        ),
        (
            Severity::Required,
            Predicate::MinLen {
                field: "description".to_string(),
                min: 1,
            },
        ),
        (
            Severity::Required,
            Predicate::MaxLen {
                field: "description".to_string(),
                max: 1024,
            },
        ),
        (
            Severity::Required,
            Predicate::MaxLen {
                field: "compatibility".to_string(),
                max: 500,
            },
        ),
        (Severity::Advisory, Predicate::MaxLines { max: 500 }),
        (
            Severity::Required,
            Predicate::ForbiddenKeys {
                keys: vec!["globs".to_string(), "alwaysApply".to_string()],
            },
        ),
    ]
}

/// The embedded skill built-in carries exactly the decidable clause vector at its
/// declared severities, and its display name is its bare kind label, `skill`.
#[test]
fn skill_builtin_carries_the_decidable_clause_vector() {
    let contract = skill_builtin();
    assert_eq!(contract.name, "skill");
    assert_eq!(predicate_vector(&contract), expected_skill_clauses());
}

/// A built-in package's clauses are *cited* and carry guidance — each pairs a
/// `source` provenance of taste with the hover-sized why (`specs/model/contract.md`,
/// "a built-in package's clauses ... it is the expected posture"). Pinning presence
/// (not text) keeps the update ritual honest — walk the clauses, re-check their
/// citations — without coupling the build test to product prose.
#[test]
fn every_skill_builtin_clause_is_guided_and_cited() {
    for clause in &skill_builtin().clauses {
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

/// No dropped heuristic survives as a clause. Whole-vector equality above already
/// implies this, but pinning the predicate kind set explicitly documents the
/// registry-kill decision: the built-in encodes *only* decidable predicates — every
/// clause is a true/false fact over the artifact, never a semantic guess
/// (third-person / has-trigger / companion-refs were undecidable and stay prose
/// guidance, `specs/model/contract.md`).
#[test]
fn skill_builtin_encodes_only_decidable_clauses() {
    let kinds: BTreeSet<&str> = skill_builtin()
        .clauses
        .iter()
        .map(|clause| clause.predicate.key())
        .collect();

    assert_eq!(
        kinds,
        BTreeSet::from([
            "required",
            "min_len",
            "max_len",
            "deny",
            "allowed_chars",
            "max_lines",
            "name-matches-dir",
            "forbidden_keys",
        ]),
        "the built-in must carry only its declared decidable predicates",
    );
}

/// Both shipped built-in packages are themselves admissible — they pass the second
/// green (`specs/model/contract.md`, "Decision: the contract is itself checked —
/// admissibility"). Every embedded package carries only closed-vocabulary clauses
/// and no vacuous list clause, so `engine::admissibility` returns no findings.
#[test]
fn the_shipped_built_in_packages_are_admissible() {
    let builtins = temper::builtin::contracts();
    assert!(
        !builtins.is_empty(),
        "at least the skill and rule built-ins must be embedded"
    );
    for (name, contract) in &builtins {
        let diagnostics = engine::admissibility(contract);
        assert!(
            diagnostics.is_empty(),
            "{name} should be admissible, got: {diagnostics:?}",
        );
    }
}

// ---- the each-grain `kind` predicate `requirement.kind` sources -------------
//
// SATISFIER-KIND-CLAUSE (`specs/model/contract.md`, "selection"): one new predicate
// in the closed vocabulary expressing "this satisfier is of the declared kind K" —
// the each-grain clause a typed requirement's `kind` facet sources instead of
// narrowing the candidate set.

/// The shipped `kind` clause parses/loads: [`temper::builtin::kind_narrowing_clause`]
/// synthesizes it at `required` severity from a bare kind label, and it carries the
/// closed-vocabulary `Predicate::Kind` shape with no guidance/cite of its own.
#[test]
fn the_kind_narrowing_clause_loads_at_required_severity() {
    let clause = temper::builtin::kind_narrowing_clause("skill");
    assert_eq!(clause.severity, Severity::Required);
    assert_eq!(
        clause.predicate,
        Predicate::Kind {
            kind: "skill".to_string()
        }
    );
    assert_eq!(clause.predicate.key(), "kind");
}

/// The clause round-trips in a requirement's clause set: attaching it to a
/// [`temper::compose::Requirement`] and reading it back off `clauses` yields the
/// identical predicate — the same shape [`temper::roster::check`] evaluates over the
/// kind-blind satisfier set.
#[test]
fn the_kind_narrowing_clause_round_trips_in_a_requirements_clause_set() {
    let clause = temper::builtin::kind_narrowing_clause("skill");
    let requirement = temper::compose::Requirement {
        name: "planner".to_string(),
        means: None,
        kind: Some("skill".to_string()),
        required: false,
        clauses: vec![clause.clone()],
        verified_by: None,
    };
    assert_eq!(requirement.clauses, vec![clause]);
    assert_eq!(
        requirement.clauses[0].predicate,
        Predicate::Kind {
            kind: "skill".to_string()
        }
    );
}

/// A named `kind` clause is admissible; an empty `kind` is vacuous — it names
/// nothing to match, so it is rejected exactly as an empty `enum`/`deny` list is.
#[test]
fn an_empty_kind_clause_is_inadmissible_a_named_one_is_not() {
    let bare_contract = |predicate: Predicate| Contract {
        name: "kind-clause-fixture".to_string(),
        clauses: vec![temper::contract::Clause {
            severity: Severity::Required,
            predicate,
            guidance: None,
            source: None,
        }],
        guidance: None,
    };

    assert!(
        engine::admissibility(&bare_contract(Predicate::Kind {
            kind: "skill".to_string()
        }))
        .is_empty()
    );

    let empty = engine::admissibility(&bare_contract(Predicate::Kind {
        kind: String::new(),
    }));
    assert_eq!(empty.len(), 1);
    assert!(empty[0].message.contains("kind"));
}
