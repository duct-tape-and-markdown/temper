//! Field addressing — the declared RFC 9535 subset a clause's `field` spells, judged
//! through the engine that decides it.
//!
//! Four properties, and they are the whole bargain: a name path reads a nested value and
//! fires on it; `[*]` is the each-grain over an array's elements, indicting each offender
//! by its own address; a path spelling anything past the subset — a filter, a slice, an
//! index, a recursive descent — is **inadmissible**, not silently skipped, which is what
//! keeps the RFC engine hidden mechanics rather than an author-facing pattern language;
//! and a bare name addresses exactly what it always did.

use std::collections::BTreeSet;

use serde_json::json;

use temper::check::{Diagnostic, Severity, any_error};
use temper::contract::{self, Clause, Contract, Predicate, Severity as ClauseSeverity};
use temper::engine::{self, Locus};
use temper::extract::{Features, ValueType};

mod common;

/// A catalog-shaped member: the fields are the retained parse, exactly as the
/// `json-document` read face hands them over.
fn member(fields: serde_json::Value) -> Features {
    let serde_json::Value::Object(fields) = fields else {
        unreachable!("the fixture is a JSON object")
    };
    Features {
        id: "acme-tools".to_string(),
        fields: fields.into_iter().collect(),
        body_lines: 0,
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

/// A one-clause contract over `predicate`, at error severity.
fn contract(predicate: Predicate) -> Contract {
    Contract {
        name: "marketplace".to_string(),
        guidance: None,
        clauses: vec![Clause {
            label: contract::clause_label(Some("marketplace"), predicate.key(), None),
            severity: ClauseSeverity::Required,
            predicate,
            guidance: None,
            source: None,
        }],
    }
}

/// The findings a one-clause contract over `predicate` fires against `features`.
fn findings(predicate: Predicate, features: &Features) -> Vec<Diagnostic> {
    engine::validate(&contract(predicate), std::slice::from_ref(features))
}

/// Each finding's message.
fn messages(diagnostics: &[Diagnostic]) -> Vec<&str> {
    diagnostics.iter().map(|d| d.message.as_str()).collect()
}

#[test]
fn a_name_path_reads_the_nested_value_and_fires_on_it() {
    let filled = member(json!({"owner": {"name": "DevTools Team"}}));
    let bare = member(json!({"owner": {"email": "tools@acme.example"}}));

    let required = || Predicate::Required {
        field: "owner.name".to_string(),
    };
    assert!(
        findings(required(), &filled).is_empty(),
        "a filled name holds"
    );

    // An `owner` object present but missing `name` fires — the clause reaches a key the
    // flattening read could never have seen.
    let diags = findings(required(), &bare);
    assert_eq!(diags.len(), 1, "{:?}", messages(&diags));
    assert_eq!(diags[0].message, "required field `owner.name` is absent");
    assert_eq!(diags[0].severity, Severity::Error);

    // A value predicate reads the *nested leaf's* own value, not the container's: the
    // path walks, and the leaf keeps the source kind the retained parse carried.
    let diags = findings(
        Predicate::MinLen {
            field: "owner.name".to_string(),
            min: 100,
        },
        &filled,
    );
    assert_eq!(
        messages(&diags),
        vec!["field `owner.name` is 13 characters (min 100)"]
    );
    let diags = findings(
        Predicate::Type {
            field: "owner.name".to_string(),
            kinds: BTreeSet::from([ValueType::Integer]),
        },
        &filled,
    );
    assert_eq!(
        messages(&diags),
        vec!["field `owner.name` is `string` but the contract declares `integer`"]
    );
}

#[test]
fn a_name_path_that_resolves_nowhere_is_absent_rather_than_errored() {
    // A missing key, and a scalar met before the leaf (`owner` is a string, so
    // `owner.name` has no sub-key): the value predicates stay silent — absence is the
    // `required` clause's concern, exactly as for a bare name.
    let scalar_owner = member(json!({"owner": "DevTools Team"}));
    let no_owner = member(json!({"name": "acme-tools"}));

    for features in [&scalar_owner, &no_owner] {
        assert!(
            findings(
                Predicate::MinLen {
                    field: "owner.name".to_string(),
                    min: 100,
                },
                features,
            )
            .is_empty()
        );
        // `required` *does* fire: a parent that carries no such key is exactly the
        // absence it decides. A parent that is not an object carries no key either.
        let diags = findings(
            Predicate::Required {
                field: "owner.name".to_string(),
            },
            features,
        );
        assert_eq!(diags.len(), usize::from(features == &scalar_owner));
    }
}

#[test]
fn each_element_grains_over_the_array_and_indicts_each_offender_by_its_index() {
    let catalog = member(json!({
        "plugins": [
            {"name": "formatter", "source": "./plugins/formatter"},
            {"name": "nameless-source"},
            {"source": "./plugins/anonymous"},
        ]
    }));

    // One finding per entry that omits the key — never one for the array, and never one
    // for the entries that carry it.
    let diags = findings(
        Predicate::Required {
            field: "plugins[*].source".to_string(),
        },
        &catalog,
    );
    assert_eq!(
        messages(&diags),
        vec!["required field `plugins[1].source` is absent"]
    );

    let diags = findings(
        Predicate::Required {
            field: "plugins[*].name".to_string(),
        },
        &catalog,
    );
    assert_eq!(
        messages(&diags),
        vec!["required field `plugins[2].name` is absent"]
    );

    // A value predicate grains the same way: each element is judged on its own value and
    // named by its own address.
    let diags = findings(
        Predicate::MinLen {
            field: "plugins[*].name".to_string(),
            min: 12,
        },
        &catalog,
    );
    assert_eq!(
        messages(&diags),
        vec!["field `plugins[0].name` is 9 characters (min 12)"]
    );

    // An empty catalog grains over nothing — no element, no finding.
    let empty = member(json!({"plugins": []}));
    assert!(
        findings(
            Predicate::Required {
                field: "plugins[*].source".to_string(),
            },
            &empty,
        )
        .is_empty()
    );
}

#[test]
fn a_path_beyond_the_declared_subset_is_inadmissible_rather_than_skipped() {
    // A filter, a slice, an index, a recursive descent, and a quoted name: each is real
    // RFC 9535 the engine underneath would happily evaluate, and each is refused *at
    // admissibility* — the contract fails to load rather than quietly gaining a pattern
    // language. That refusal is the bound; without it the subset is a suggestion.
    for spelling in [
        "plugins[?@.source].name",
        "plugins[0:2].name",
        "plugins[0].name",
        "owner..name",
        "plugins['name']",
        "plugins.*.name",
    ] {
        let diags = engine::admissibility(
            &contract(Predicate::Required {
                field: spelling.to_string(),
            }),
            &Locus::Document,
        );
        assert_eq!(diags.len(), 1, "`{spelling}` must be refused: {diags:?}");
        assert_eq!(
            diags[0].severity,
            Severity::Error,
            "an inadmissible contract fails the run"
        );
        assert!(any_error(&diags));
        // The refusal names the subset, so an author reads the bound rather than the RFC.
        assert!(
            diags[0].message.contains("`owner.name`")
                && diags[0].message.contains("`plugins[*].source`"),
            "got: {}",
            diags[0].message
        );
    }

    // Every predicate that addresses a field is fenced, not just `required`.
    for predicate in [
        Predicate::Required {
            field: "plugins[0]".to_string(),
        },
        Predicate::Type {
            field: "plugins[0]".to_string(),
            kinds: BTreeSet::from([ValueType::Map]),
        },
        Predicate::MinLen {
            field: "plugins[0]".to_string(),
            min: 1,
        },
        Predicate::Enum {
            field: "plugins[0]".to_string(),
            values: vec!["a".to_string()],
        },
        Predicate::GlobValid {
            field: "plugins[0]".to_string(),
        },
    ] {
        let key = predicate.key();
        let diags = engine::admissibility(&contract(predicate), &Locus::Document);
        assert_eq!(diags.len(), 1, "`{key}` must be fenced: {diags:?}");
    }

    // The paths inside the subset are admissible — the fence refuses the spelling, never
    // the reach.
    for spelling in ["name", "owner.name", "plugins[*].source"] {
        assert!(
            engine::admissibility(
                &contract(Predicate::Required {
                    field: spelling.to_string(),
                }),
                &Locus::Document,
            )
            .is_empty(),
            "`{spelling}` is inside the subset"
        );
    }
}

#[test]
fn a_presence_clause_addressing_elements_rather_than_a_key_is_inadmissible() {
    // `required("plugins[*]")` names each element, so there is no key of it that could be
    // absent — the clause could never fire. Vacuous, and refused where the other vacuous
    // shapes are.
    let diags = engine::admissibility(
        &contract(Predicate::Required {
            field: "plugins[*]".to_string(),
        }),
        &Locus::Document,
    );
    assert_eq!(diags.len(), 1, "{diags:?}");
    assert!(
        diags[0].message.contains("ends in a name segment"),
        "the refusal names the rule: {}",
        diags[0].message
    );

    // A *value* predicate over the same path is fine: it judges each element's own value,
    // which is a question the elements answer.
    assert!(
        engine::admissibility(
            &contract(Predicate::Type {
                field: "plugins[*]".to_string(),
                kinds: BTreeSet::from([ValueType::Map]),
            }),
            &Locus::Document,
        )
        .is_empty()
    );
}

#[test]
fn a_bare_name_addresses_exactly_what_it_addresses_today() {
    // The widening's compatibility bar. A lone name segment is the flat top-level lookup:
    // same verdict, same wording, and a key the RFC's own name shorthand cannot spell
    // (`disable-model-invocation`) is an ordinary name segment here — the subset's names
    // are the format's keys, never the RFC's identifiers.
    let features = member(json!({
        "name": "acme-tools",
        "disable-model-invocation": true,
        "paths": ["src/**/*.rs", "["],
    }));

    assert!(
        findings(
            Predicate::Required {
                field: "name".to_string()
            },
            &features
        )
        .is_empty()
    );
    let diags = findings(
        Predicate::MaxLen {
            field: "name".to_string(),
            max: 3,
        },
        &features,
    );
    assert_eq!(
        messages(&diags),
        vec!["field `name` is 10 characters (max 3)"]
    );

    let diags = findings(
        Predicate::Type {
            field: "disable-model-invocation".to_string(),
            kinds: BTreeSet::from([ValueType::String]),
        },
        &features,
    );
    assert_eq!(
        messages(&diags),
        vec!["field `disable-model-invocation` is `boolean` but the contract declares `string`"]
    );

    // A list field still reads as one value the predicate ranges over, element-wise where
    // the predicate says so — `glob-valid`'s own grain, untouched by addressing.
    let diags = findings(
        Predicate::GlobValid {
            field: "paths".to_string(),
        },
        &features,
    );
    assert_eq!(diags.len(), 1, "{:?}", messages(&diags));
    assert!(diags[0].message.contains("field `paths` glob `[`"));

    // An absent bare name is absent, never errored.
    assert!(
        findings(
            Predicate::MaxLen {
                field: "absent".to_string(),
                max: 1
            },
            &features
        )
        .is_empty()
    );
}

#[test]
fn addressing_rides_the_real_extraction_rather_than_a_hand_built_map() {
    // The seam the properties above assume: a kind's `field` primitive retains the parsed
    // value whole, so a clause's path has real nesting to walk. Built through the same
    // composed extractor every member's features come from.
    let serde_json::Value::Object(frontmatter) = json!({
        "owner": {"name": "DevTools Team"},
        "plugins": [{"name": "formatter"}, {}],
    }) else {
        unreachable!("the fixture is a JSON object")
    };
    let unit = common::raw_unit(
        "acme-tools",
        frontmatter.into_iter().collect(),
        "",
        ".claude-plugin/marketplace.json",
    );
    let features = temper::kind::Extraction::new(vec![
        temper::kind::Primitive::Field {
            key: "owner".to_string(),
        },
        temper::kind::Primitive::Field {
            key: "plugins".to_string(),
        },
    ])
    .extract(&unit);

    assert!(
        findings(
            Predicate::Required {
                field: "owner.name".to_string()
            },
            &features
        )
        .is_empty()
    );
    let diags = findings(
        Predicate::Required {
            field: "plugins[*].name".to_string(),
        },
        &features,
    );
    assert_eq!(
        messages(&diags),
        vec!["required field `plugins[1].name` is absent"]
    );
}
