//! The `type` predicate's whole seam: SDK constructor → lock column → engine verdict.
//!
//! `type` is the one predicate whose declared kind neither the shared `field` column
//! nor `severity` can carry, so the lattice name rides its own `value_type` column —
//! the spelling [`temper::extract::ValueType::from_name`] decodes. These proofs drive
//! the seam end to end: a real SDK program authors the clause, `emit` compiles the
//! lock, [`temper::drift::read_declarations`] reads it back, and the engine decides a
//! member against the lifted predicate.

use std::collections::BTreeMap;

use temper::contract::{self, Clause, Contract, Predicate, Severity};
use temper::drift::{self, EmitOptions};
use temper::engine;
use temper::extract::{Features, ValueType};
use temper::kind::{Extraction, Primitive};

mod common;

/// A harness whose custom `pack` kind expects a `list`-typed `keywords` field — the
/// clause `type` exists for, authored the way an SDK consumer authors it.
const PACK_PROGRAM: &str = r#"
import { clause, emit, harness, kind, text, type } from "@dtmd/temper";

const pack = kind<{ keywords: string[] }>({
  name: "pack",
  locus: { kind: "at", root: ".claude/packs", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [],
});

const program = harness({
  members: [pack({ name: "toolkit", keywords: ["review", "lint"], prose: text`# Toolkit` })],
  expect: [{ kind: pack, clauses: [clause(type("keywords", "list"), { severity: "required" })] }],
});

process.stdout.write(emit(program).seam);
"#;

/// A `pack` unit whose `keywords` field carries `value` — the parsed JSON the
/// extractor preserves the source kind of.
fn pack_unit(value: serde_json::Value) -> temper::kind::Unit {
    common::raw_unit(
        "toolkit",
        BTreeMap::from([("keywords".to_string(), value)]),
        "# Toolkit\n",
        ".claude/packs/toolkit.md",
    )
}

/// The `keywords` features of a `pack` unit carrying `value`.
fn pack_features(value: serde_json::Value) -> Features {
    Extraction::new(vec![Primitive::Field {
        key: "keywords".to_string(),
    }])
    .extract(&pack_unit(value))
}

/// The findings a one-clause contract over `predicate` fires against `features`.
fn findings(predicate: Predicate, features: &Features) -> Vec<temper::check::Diagnostic> {
    let contract = Contract {
        name: "pack".to_string(),
        guidance: None,
        clauses: vec![Clause {
            label: contract::clause_label(Some("pack"), predicate.key(), None),
            severity: Severity::Required,
            predicate,
            guidance: None,
            source: None,
        }],
    };
    engine::validate(&contract, std::slice::from_ref(features))
}

#[test]
fn an_sdk_authored_type_clause_reaches_the_engine_through_the_lock() {
    let (_harness, into) = common::wire_sdk_harness("type-predicate", PACK_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).expect(
        "the `type` seam is driven through a real node + built @dtmd/temper module — the \
         lane fails loud here rather than silently skipping the round-trip",
    );

    // The lock's clause row carries the declared kind in its own column: `field` names
    // what is checked, `value_type` what it must be.
    let declarations = drift::read_declarations(&into).unwrap();
    let row = declarations
        .clauses
        .iter()
        .find(|row| row.predicate == "type")
        .expect("the authored `type` clause reaches the lock as its own row");
    assert_eq!(row.kind.as_deref(), Some("pack"));
    assert_eq!(row.field.as_deref(), Some("keywords"));
    assert_eq!(
        row.value_type.as_deref(),
        Some("list"),
        "the declared kind crosses the lock as its lattice name"
    );

    // The row lifts back into the typed predicate — the closed vocabulary the SDK
    // exports is the vocabulary the engine receives, not a load error.
    let predicate = contract::predicate_from_row(row).expect("a `type` row lifts");
    assert_eq!(
        predicate,
        Predicate::Type {
            field: "keywords".to_string(),
            kind: ValueType::List,
        }
    );

    // And the lifted predicate decides: the author gets the check they asked for.
    let wrong = findings(
        predicate.clone(),
        &pack_features(serde_json::json!("review")),
    );
    assert_eq!(wrong.len(), 1, "a string `keywords` violates the clause");
    assert_eq!(wrong[0].rule, "pack.type");

    assert!(
        findings(
            predicate.clone(),
            &pack_features(serde_json::json!(["review", "lint"]))
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
