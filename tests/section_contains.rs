//! Fixture-driven proof of the `section_contains` structural predicate and its
//! `## Decision`-block extraction primitive (`specs/architecture/10-contracts.md`, "The
//! primitive algebra"; the decisions-name-alternatives capability the spec kind's
//! package awaits, `specs/architecture/15-kinds.md`).
//!
//! The predicate is decided end-to-end over the *new* section-body extraction: a
//! spec-shaped [`Unit`] runs through the composed `sections` primitive
//! ([`temper::kind`]) into [`Features::sections`], then the `section_contains`
//! clause is evaluated over it ([`temper::engine`]). This proves the predicate is a
//! true positive — it fires on a `## Decision` section missing its `Rejected` marker
//! and stays silent when every matching section carries it — and that the closed
//! vocabulary still rejects an out-of-vocabulary primitive/predicate at load.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use temper::check::{Diagnostic, Severity as DiagnosticSeverity};
use temper::contract::{Clause, Contract, ContractError, Predicate, Severity};
use temper::engine;
use temper::extract::Features;
use temper::kind::{Extraction, KindError, Unit};

/// The composed `sections` extractor — the one primitive the `section_contains`
/// predicate reads.
fn sections_extraction() -> Extraction {
    Extraction::parse(
        "[[extraction]]\nprimitive = \"sections\"\n",
        Path::new("temper.toml"),
    )
    .expect("the `sections` primitive is in the closed vocabulary")
}

/// A spec-shaped raw unit (no frontmatter; the whole file is body) carrying `body`.
fn spec_unit(body: &str) -> Unit {
    Unit {
        id: "10-contracts".to_string(),
        frontmatter: BTreeMap::new(),
        body: body.to_string(),
        source_path: PathBuf::from("specs/architecture/10-contracts.md"),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
        published_requirements: Vec::new(),
    }
}

/// The `section_contains { heading = "Decision", marker = "Rejected" }` contract —
/// the decisions-name-alternatives clause: every `## Decision` section must name a
/// rejected alternative.
fn decision_contract() -> Contract {
    Contract {
        name: "spec".to_string(),
        guidance: None,
        clauses: vec![Clause {
            severity: Severity::Required,
            predicate: Predicate::SectionContains {
                heading: "Decision".to_string(),
                marker: "Rejected".to_string(),
            },
            guidance: None,
            source: None,
        }],
    }
}

/// Extract a spec body's sections and validate the decision contract over them —
/// the full path a `check` run takes for the `spec` kind.
fn check(body: &str) -> Vec<Diagnostic> {
    let features: Features = sections_extraction().extract(&spec_unit(body));
    engine::validate(&decision_contract(), std::slice::from_ref(&features))
}

/// Every `## Decision` section carries the `Rejected` marker (and a non-Decision
/// section that lacks it is *not* governed) — the clause holds, no diagnostics.
#[test]
fn every_decision_section_carrying_the_marker_passes() {
    let clean = "# Contracts\n\
\n\
## Decision: kill the registry\n\
\n\
Chosen: a generic engine. Rejected: the hardcoded registry.\n\
\n\
## Decision: a closed algebra\n\
\n\
Chosen: a bespoke vocabulary. Rejected: a general policy engine.\n\
\n\
## The primitive algebra\n\
\n\
Prose with no rejected alternative — but this heading is not a Decision, so the\n\
clause never governs it.\n";

    assert!(
        check(clean).is_empty(),
        "a spec whose every Decision section names a rejected alternative is clean"
    );
}

/// A `## Decision` section missing its `Rejected` marker fires exactly one
/// diagnostic — a true positive naming the bare section — while the sibling
/// Decision that carries it stays silent.
#[test]
fn a_bare_decision_section_fires_exactly_one_diagnostic() {
    let bare = "# Contracts\n\
\n\
## Decision: kill the registry\n\
\n\
Chosen: a generic engine. Rejected: the hardcoded registry.\n\
\n\
## Decision: severity is declared\n\
\n\
Chosen: author-declared severity — but this section names no alternative.\n";

    let diags = check(bare);
    assert_eq!(diags.len(), 1, "exactly one bare Decision section fires");
    assert_eq!(diags[0].rule, "section_contains");
    assert_eq!(diags[0].artifact, "10-contracts");
    assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
    // The finding names the offending section, not the one that carried the marker.
    assert!(diags[0].message.contains("Decision: severity is declared"));
    assert!(diags[0].message.contains("Rejected"));
}

/// A spec with no `Decision` section at all trips nothing — the clause governs an
/// empty set, so there is nothing to violate (no false positive on missing prose).
#[test]
fn a_spec_with_no_decision_sections_trips_nothing() {
    let none = "# Contracts\n\
\n\
## The primitive algebra\n\
\n\
Prose with no rejected alternative named anywhere.\n\
\n\
## Requirements\n\
\n\
More prose, still no marker.\n";

    assert!(
        check(none).is_empty(),
        "no section matches the `Decision` prefix, so the clause governs nothing"
    );
}

/// The closed vocabulary holds on both sides: an out-of-vocabulary extraction
/// primitive is rejected at load, exactly as an out-of-vocabulary predicate is —
/// `section_contains` and `sections` are additions to the vocabulary, not a
/// trapdoor that opened it (`specs/architecture/10-contracts.md`, "Decision: the contract is
/// itself checked — admissibility").
#[test]
fn an_out_of_vocabulary_primitive_or_predicate_is_still_rejected_at_load() {
    let primitive_err = Extraction::parse(
        "[[extraction]]\nprimitive = \"decision_meaning\"\n",
        Path::new("temper.toml"),
    )
    .expect_err("an unknown extraction primitive is a load error");
    assert!(matches!(
        primitive_err,
        KindError::UnknownPrimitive { ref primitive, .. } if primitive == "decision_meaning"
    ));

    let predicate_err = Contract::parse(
        "[[clause]]\nseverity = \"required\"\npredicate = \"section_matches\"\nheading = \"Decision\"\n",
        Path::new("spec.contract.toml"),
    )
    .expect_err("an unknown predicate is a load error");
    assert!(matches!(
        predicate_err,
        ContractError::UnknownPredicate { ref predicate, .. } if predicate == "section_matches"
    ));
}
