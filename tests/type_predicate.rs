//! The `type` predicate's whole seam: SDK constructor → lock column → engine verdict.
//!
//! `type` is the one predicate whose declared kinds neither the shared `field` column
//! nor `severity` can carry, so the lattice names ride their own `value_type` column —
//! the spelling [`temper::extract::ValueType::from_name`] decodes. These proofs drive
//! the seam end to end: a real SDK program authors the clause, `emit` compiles the
//! lock, [`temper::drift::read_declarations`] reads it back, and the engine decides a
//! member against the lifted predicate.
//!
//! The declaration is a set, so the seam carries a set: an external format documenting
//! a field as `string|array` is gateable, a one-element set is the single-kind clause
//! it replaces, and a lock an older engine wrote spells that case as a bare string.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use temper::contract::{self, Clause, Contract, Predicate, Severity};
use temper::drift::{self, ClauseRow, EmitOptions};
use temper::engine;
use temper::extract::{Features, ValueType};
use temper::kind::{Extraction, Primitive};

mod common;

/// A harness whose custom `pack` kind expects a `list`-typed `keywords` field and a
/// `string|array`-typed `paths` field — the single-kind clause and the union the set
/// widening bought, both authored the way an SDK consumer authors them.
const PACK_PROGRAM: &str = r#"
import { clause, emit, harness, kind, text, type } from "@dtmd/temper";

const pack = kind<{ keywords: string[]; paths: string }>({
  name: "pack",
  locus: { kind: "at", root: ".claude/packs", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [],
});

const program = harness({
  members: [pack({ name: "toolkit", keywords: ["review", "lint"], paths: "./src", prose: text`# Toolkit` })],
  expect: [{ kind: pack, clauses: [
    clause(type("keywords", ["list"]), { severity: "required" }),
    clause(type("paths", ["string", "list"]), { severity: "required" }),
  ] }],
});

process.stdout.write(emit(program).seam);
"#;

/// A `pack` unit whose `field` carries `value` — the parsed JSON the extractor
/// preserves the source kind of.
fn pack_unit(field: &str, value: serde_json::Value) -> temper::kind::Unit {
    common::raw_unit(
        "toolkit",
        BTreeMap::from([(field.to_string(), value)]),
        "# Toolkit\n",
        ".claude/packs/toolkit.md",
    )
}

/// The `field` features of a `pack` unit carrying `value`.
fn pack_features(field: &str, value: serde_json::Value) -> Features {
    Extraction::new(vec![Primitive::Field {
        key: field.to_string(),
    }])
    .extract(&pack_unit(field, value))
}

/// A one-clause `pack` contract over `predicate` — the shape both the conformance and
/// the admissibility proofs below judge.
fn one_clause_contract(predicate: Predicate) -> Contract {
    Contract {
        name: "pack".to_string(),
        guidance: None,
        clauses: vec![Clause {
            label: contract::clause_label(Some("pack"), predicate.key(), None),
            severity: Severity::Required,
            predicate,
            guidance: None,
            source: None,
        }],
    }
}

/// The findings a one-clause contract over `predicate` fires against `features`.
fn findings(predicate: Predicate, features: &Features) -> Vec<temper::check::Diagnostic> {
    engine::validate(
        &one_clause_contract(predicate),
        std::slice::from_ref(features),
    )
}

/// The lock's one `type` clause row over `field`.
fn type_row<'a>(clauses: &'a [ClauseRow], field: &str) -> &'a ClauseRow {
    clauses
        .iter()
        .find(|row| row.predicate == "type" && row.field.as_deref() == Some(field))
        .expect("the authored `type` clause reaches the lock as its own row")
}

#[test]
fn an_sdk_authored_type_clause_reaches_the_engine_through_the_lock() {
    let (_harness, into) = common::wire_sdk_harness("type-predicate", PACK_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).expect(
        "the `type` seam is driven through a real node + built @dtmd/temper module — the \
         lane fails loud here rather than silently skipping the round-trip",
    );

    // The lock's clause row carries the declared kinds in its own column: `field` names
    // what is checked, `value_type` what it may be.
    let declarations = drift::read_declarations(&into).unwrap();
    let row = type_row(&declarations.clauses, "keywords");
    assert_eq!(row.kind.as_deref(), Some("pack"));
    assert_eq!(
        row.value_type.as_deref(),
        Some(["list".to_string()].as_slice()),
        "a single declared kind crosses the lock as a one-element array of its lattice name"
    );

    // The row lifts back into the typed predicate — the closed vocabulary the SDK
    // exports is the vocabulary the engine receives, not a load error.
    let predicate = contract::predicate_from_row(row).expect("a `type` row lifts");
    assert_eq!(
        predicate,
        Predicate::Type {
            field: "keywords".to_string(),
            kinds: BTreeSet::from([ValueType::List]),
        }
    );

    // And the lifted predicate decides: the author gets the check they asked for.
    let wrong = findings(
        predicate.clone(),
        &pack_features("keywords", serde_json::json!("review")),
    );
    assert_eq!(wrong.len(), 1, "a string `keywords` violates the clause");
    assert_eq!(wrong[0].rule, "pack.type");

    assert!(
        findings(
            predicate.clone(),
            &pack_features("keywords", serde_json::json!(["review", "lint"]))
        )
        .is_empty(),
        "a list `keywords` holds"
    );

    // Presence is `required`'s concern, so `type` stays silent on an absent field —
    // one missing field is one finding, never a cascade.
    let absent = Extraction::new(vec![Primitive::Field {
        key: "keywords".to_string(),
    }])
    .extract(&common::raw_unit(
        "toolkit",
        BTreeMap::new(),
        "# Toolkit\n",
        ".claude/packs/toolkit.md",
    ));
    assert!(findings(predicate, &absent).is_empty());
}

#[test]
fn an_sdk_authored_set_of_kinds_reaches_the_engine_and_holds_for_any_member() {
    let (_harness, into) = common::wire_sdk_harness("type-predicate-set", PACK_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).expect(
        "the `type` seam is driven through a real node + built @dtmd/temper module — the \
         lane fails loud here rather than silently skipping the round-trip",
    );

    // A `string|array` field — the documented shape the set widening exists for —
    // crosses the lock as the whole set, in lattice order rather than the author's.
    let declarations = drift::read_declarations(&into).unwrap();
    let row = type_row(&declarations.clauses, "paths");
    assert_eq!(
        row.value_type.as_deref(),
        Some(["string".to_string(), "list".to_string()].as_slice()),
    );

    let predicate = contract::predicate_from_row(row).expect("a set-valued `type` row lifts");
    assert_eq!(
        predicate,
        Predicate::Type {
            field: "paths".to_string(),
            kinds: BTreeSet::from([ValueType::String, ValueType::List]),
        }
    );

    // Either documented form holds: this is the false positive a single-kind clause
    // could not avoid producing over a union-typed field.
    for form in [serde_json::json!("./src"), serde_json::json!(["./src"])] {
        assert!(
            findings(predicate.clone(), &pack_features("paths", form)).is_empty(),
            "every member of the declared set holds"
        );
    }

    // A kind outside the set fires once, naming the whole set it missed.
    let wrong = findings(predicate, &pack_features("paths", serde_json::json!(7)));
    assert_eq!(wrong.len(), 1);
    assert_eq!(
        wrong[0].message,
        "field `paths` is `integer` but the contract declares `string|list`"
    );
}

#[test]
fn an_empty_set_is_inadmissible_rather_than_a_clause_that_admits_nothing() {
    // The lock can spell it, so the engine must refuse it: no value the lattice carries
    // satisfies an empty set — the vacuity an inverted `range` bound already fails on.
    let row = ClauseRow {
        unit: None,
        field: Some("paths".to_string()),
        value_type: Some(Vec::new()),
        ..common::clause("type", "required")
    };

    // It lifts cleanly: a vacuous clause is the engine's refusal to make, never the
    // decoder's — the row is well-formed, the contract carrying it is not.
    let predicate = contract::predicate_from_row(&row).expect("an empty-set row lifts");
    assert_eq!(
        predicate,
        Predicate::Type {
            field: "paths".to_string(),
            kinds: BTreeSet::new(),
        }
    );

    let diagnostics =
        engine::admissibility(&one_clause_contract(predicate), &engine::Locus::Document);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].severity, temper::check::Severity::Error);
    assert_eq!(
        diagnostics[0].message,
        "`type` clause on field `paths` lists no kinds"
    );
}

#[test]
fn a_lock_row_spelling_one_bare_string_reads_as_the_one_element_set_it_means() {
    // The version skew an upgraded engine owes a committed lock: an engine predating the
    // set widening wrote one lattice name as a bare string, which means the one-element
    // set exactly. Downgrade a real lock's own row to that spelling and read it back.
    let (_harness, into) = common::wire_sdk_harness("type-predicate-older-lock", PACK_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).expect("the pack program emits");

    let lock = into.join("lock.toml");
    let current = fs::read_to_string(&lock).unwrap();
    assert!(
        current.contains(r#"value_type = ["list"]"#),
        "emit writes the canonical array form",
    );
    fs::write(
        &lock,
        current.replace(r#"value_type = ["list"]"#, r#"value_type = "list""#),
    )
    .unwrap();

    let declarations = drift::read_declarations(&into).unwrap();
    let row = type_row(&declarations.clauses, "keywords");
    assert_eq!(
        row.value_type.as_deref(),
        Some(["list".to_string()].as_slice()),
        "the bare string reads as the one-element set it means",
    );
    assert_eq!(
        contract::predicate_from_row(row).expect("it lifts"),
        Predicate::Type {
            field: "keywords".to_string(),
            kinds: BTreeSet::from([ValueType::List]),
        },
        "and decides exactly as the array form does"
    );

    // The upgrade is the rewrite, never a patch of the file: re-emitting restores the
    // canonical spelling whole, which is all an older lock is owed.
    drift::emit_program(&into, EmitOptions::default()).expect("the pack program re-emits");
    assert!(
        fs::read_to_string(&lock)
            .unwrap()
            .contains(r#"value_type = ["list"]"#)
    );
}
