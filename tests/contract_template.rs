//! Pins the shipped Anthropic skill built-in floor.
//!
//! The `skill` floor is a projection of the embedded built-in lock's clause rows.
//! This test loads it through the same
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

use std::collections::{BTreeMap, BTreeSet};

use temper::check;
use temper::contract::{Charset, Contract, Predicate, Severity};
use temper::engine;
use temper::extract::Features;

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
        (
            Severity::Required,
            Predicate::GlobValid {
                field: "paths".to_string(),
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
/// `source` provenance of taste with the hover-sized why. Pinning presence
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
/// clause is a true/false fact over the artifact, never a semantic guess.
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
            "glob-valid",
        ]),
        "the built-in must carry only its declared decidable predicates",
    );
}

/// Both shipped built-in packages are themselves admissible — they pass the second
/// green. Every embedded package carries only closed-vocabulary clauses
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
// SATISFIER-KIND-CLAUSE: one new predicate
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
        prose: None,
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

// ---- the each-grain `format-places-edges` predicate -------------------------
//
// A format that omits an edge its kind declares renders a contract the prose does not
// represent. The engine never sees the format, so it decides over the placement `emit`
// observed while rendering and lowered into the member's own declaration row.

/// A member of a kind declaring the edges `placements` names, each paired with whether
/// its format placed it — the `Features` shape `main`'s embedded-member lift builds off a
/// `nested_member` row's `placed_edges` column.
fn cited(placements: &[(&str, bool)]) -> Features {
    Features {
        id: "the-standard".to_string(),
        fields: BTreeMap::new(),
        body_lines: 0,
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: Vec::new(),
        edge_placements: placements
            .iter()
            .map(|(field, placed)| ((*field).to_string(), *placed))
            .collect(),
    }
}

/// A one-clause contract over `format-places-edges` at `severity` — the whole surface an
/// author declares to adopt the check.
fn places_edges_contract(severity: Severity) -> Contract {
    Contract {
        name: "citation".to_string(),
        clauses: vec![temper::contract::Clause {
            severity,
            predicate: Predicate::FormatPlacesEdges,
            guidance: None,
            source: None,
        }],
        guidance: None,
    }
}

/// The clause loads off a lock row and carries the closed-vocabulary shape: no argument
/// columns, since the selection is the member's whole incident edge set at the `each`
/// grain.
#[test]
fn the_format_places_edges_clause_loads_off_a_bare_predicate_row() {
    assert_eq!(Predicate::FormatPlacesEdges.key(), "format-places-edges");
    assert_eq!(Predicate::FormatPlacesEdges.target(), None);
    assert!(
        engine::admissibility(&places_edges_contract(Severity::Required)).is_empty(),
        "the predicate carries no list or bound, so nothing about it can be vacuous",
    );
}

/// A format placing every edge its kind declares holds — no finding.
#[test]
fn a_format_placing_every_declared_edge_passes() {
    let diagnostics = engine::validate(
        &places_edges_contract(Severity::Required),
        &[cited(&[("source", true), ("supersedes", true)])],
    );
    assert!(
        diagnostics.is_empty(),
        "every declared edge was placed, got: {diagnostics:?}",
    );
}

/// A format omitting one declared edge is a finding naming that field — one per omitted
/// edge, so each points at the reference the format left unrepresented, and the other
/// edges' placement does not mask it.
#[test]
fn a_format_omitting_a_declared_edge_is_a_finding_naming_the_field() {
    let diagnostics = engine::validate(
        &places_edges_contract(Severity::Required),
        &[cited(&[("source", true), ("supersedes", false)])],
    );
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule, "format-places-edges");
    assert_eq!(diagnostics[0].artifact, "the-standard");
    assert!(
        diagnostics[0].message.contains("supersedes"),
        "the finding must name the omitted edge, got: {}",
        diagnostics[0].message,
    );
    assert!(!diagnostics[0].message.contains("source"));
}

/// The severity is the clause author's, never the tool's: the identical omission gates as
/// an error under a `required` clause and reports as a warning under an `advisory` one.
#[test]
fn the_omission_findings_severity_is_the_clause_authors() {
    let omitted = [cited(&[("source", false)])];
    let blocking = engine::validate(&places_edges_contract(Severity::Required), &omitted);
    assert_eq!(blocking[0].severity, check::Severity::Error);

    let reported = engine::validate(&places_edges_contract(Severity::Advisory), &omitted);
    assert_eq!(reported[0].severity, check::Severity::Warn);
}

/// A member carrying no placement fact decides nothing — a kind declaring no edge, and a
/// value no format rendered (one embedded in a layout document is read off its host's
/// declared layout, so there is no format to indict). Never a fabricated pass, never a
/// fabricated finding.
#[test]
fn a_member_with_no_placement_fact_yields_no_finding() {
    let diagnostics = engine::validate(&places_edges_contract(Severity::Required), &[cited(&[])]);
    assert!(
        diagnostics.is_empty(),
        "an absent placement fact is undecidable, not a violation: {diagnostics:?}",
    );
}

/// No shipped default contract adopts the clause: the predicate exists so an author can
/// declare it, and adding it to the vocabulary is the whole language change.
#[test]
fn no_built_in_contract_adopts_the_format_places_edges_clause() {
    for (name, contract) in &temper::builtin::contracts() {
        assert!(
            !contract
                .clauses
                .iter()
                .any(|clause| clause.predicate == Predicate::FormatPlacesEdges),
            "{name} adopts `format-places-edges` unbidden — the author declares it",
        );
    }
}
