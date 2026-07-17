//! `closed-keys` — the clause declaring a kind's already-declared key set exhaustive,
//! judged through the engine that decides it.
//!
//! Four properties, and they are the whole bargain: an undeclared key is a finding at the
//! clause's declared severity; a member carrying only declared keys holds; a contract that
//! declares no key at all is **inadmissible** rather than a clause indicting every key of
//! every member; and the allow-list is *read* from the kind's own `required`/`optional`
//! rows, so admitting a key is one row and never a second edit here — which is the
//! difference between consuming the key set and authoring it twice.

use serde_json::json;

use temper::check::{Diagnostic, Severity};
use temper::contract::{self, Clause, Contract, Predicate, Severity as ClauseSeverity};
use temper::engine::{self, Locus};
use temper::extract::Features;

/// A manifest-shaped member: the fields are the retained parse, exactly as the
/// `json-document` read face hands them over.
fn member(fields: serde_json::Value) -> Features {
    let serde_json::Value::Object(fields) = fields else {
        unreachable!("the fixture is a JSON object")
    };
    Features {
        id: "acme-tools".to_string(),
        fields: fields.into_iter().collect(),
        body_lines: 0,
        rendered_lines: 0,
        rendered_chars: 0,
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

/// One clause at `severity`, addressed as a lifted row would be.
fn clause(severity: ClauseSeverity, predicate: Predicate) -> Clause {
    Clause {
        label: contract::clause_label(Some("plugin-manifest"), predicate.key(), predicate.target()),
        severity,
        predicate,
        guidance: None,
        source: None,
    }
}

/// A contract carrying `clauses` — the sibling set `closed-keys` reads its allow-list from.
fn contract(clauses: Vec<Clause>) -> Contract {
    Contract {
        name: "plugin-manifest".to_string(),
        guidance: None,
        clauses,
    }
}

/// `required("name")`, the declaring row every fixture below opens with.
fn declares_name() -> Clause {
    clause(
        ClauseSeverity::Required,
        Predicate::Required {
            field: "name".to_string(),
        },
    )
}

/// `optional(field)` — one more key admitted, the whole edit that admits it.
fn declares(field: &str) -> Clause {
    clause(
        ClauseSeverity::Required,
        Predicate::Optional {
            field: field.to_string(),
        },
    )
}

/// `closedKeys()` at `severity`.
fn closes(severity: ClauseSeverity) -> Clause {
    clause(severity, Predicate::ClosedKeys)
}

/// Each finding's message.
fn messages(diagnostics: &[Diagnostic]) -> Vec<&str> {
    diagnostics.iter().map(|d| d.message.as_str()).collect()
}

#[test]
fn a_key_the_kind_declares_neither_required_nor_optional_is_a_finding() {
    let contract = contract(vec![
        declares_name(),
        declares("version"),
        closes(ClauseSeverity::Required),
    ]);
    let foreign = member(json!({"name": "acme-tools", "version": "1.0.0", "contributes": {}}));

    let diagnostics = engine::validate(&contract, std::slice::from_ref(&foreign));
    assert_eq!(
        messages(&diagnostics),
        vec![
            "key `contributes` is not one of the keys this contract declares, and the \
             declared set is exhaustive"
        ]
    );
    // The clause's own address, so a finding names the clause that produced it.
    assert_eq!(diagnostics[0].rule, "plugin-manifest.closed-keys");
}

#[test]
fn every_undeclared_key_points_at_itself() {
    let contract = contract(vec![declares_name(), closes(ClauseSeverity::Required)]);
    let foreign = member(json!({"name": "acme-tools", "engines": {}, "publisher": "acme"}));

    // One finding per offending key, never one lumping them: the author fixes keys, not
    // clauses.
    let diagnostics = engine::validate(&contract, std::slice::from_ref(&foreign));
    assert_eq!(diagnostics.len(), 2);
    assert!(messages(&diagnostics)[0].contains("`engines`"));
    assert!(messages(&diagnostics)[1].contains("`publisher`"));
}

#[test]
fn the_finding_lands_at_the_clauses_declared_severity() {
    // The author dials the weight, never the tool: the same undeclared key blocks under a
    // `required` clause and reports under an `advisory` one.
    let foreign = member(json!({"name": "acme-tools", "engines": {}}));
    for (declared, expected) in [
        (ClauseSeverity::Required, Severity::Error),
        (ClauseSeverity::Advisory, Severity::Warn),
    ] {
        let contract = contract(vec![declares_name(), closes(declared)]);
        let diagnostics = engine::validate(&contract, std::slice::from_ref(&foreign));
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, expected);
    }
}

#[test]
fn a_member_carrying_only_declared_keys_holds() {
    let contract = contract(vec![
        declares_name(),
        declares("version"),
        declares("keywords"),
        closes(ClauseSeverity::Required),
    ]);

    // Every declared key present, and a declared key *absent* — `optional` is satisfied
    // either way, so a closed key set is never a required one.
    let full = member(json!({"name": "acme-tools", "version": "1.0.0", "keywords": ["ci"]}));
    assert!(engine::validate(&contract, std::slice::from_ref(&full)).is_empty());
    let sparse = member(json!({"name": "acme-tools"}));
    assert!(engine::validate(&contract, std::slice::from_ref(&sparse)).is_empty());
}

#[test]
fn the_allow_list_is_read_from_the_kinds_rows_so_one_row_admits_a_key() {
    // The point of the widening: the key set is declared once. Adding the `optional` row is
    // the whole edit that admits the key — the clause itself carries no list to keep in
    // step, so the two cannot drift apart.
    let carrying = member(json!({"name": "acme-tools", "displayName": "Acme Tools"}));

    let closed = contract(vec![declares_name(), closes(ClauseSeverity::Required)]);
    assert_eq!(
        engine::validate(&closed, std::slice::from_ref(&carrying)).len(),
        1
    );

    let widened = contract(vec![
        declares_name(),
        declares("displayName"),
        closes(ClauseSeverity::Required),
    ]);
    assert!(engine::validate(&widened, std::slice::from_ref(&carrying)).is_empty());
}

#[test]
fn a_path_declares_its_top_level_key_never_the_nested_one_it_addresses() {
    // `required("owner.name")` says the member carries an `owner` key. The nested leaf is
    // no top-level key of its own, so a closed set neither admits `name` at the top level
    // nor indicts the `owner` object the path walks into.
    let contract = contract(vec![
        declares_name(),
        clause(
            ClauseSeverity::Required,
            Predicate::Required {
                field: "owner.name".to_string(),
            },
        ),
        clause(
            ClauseSeverity::Required,
            Predicate::Optional {
                field: "plugins[*].source".to_string(),
            },
        ),
        closes(ClauseSeverity::Required),
    ]);

    let nested = member(json!({
        "name": "acme",
        "owner": {"name": "DevTools Team"},
        "plugins": [{"source": "./a"}],
    }));
    assert!(engine::validate(&contract, std::slice::from_ref(&nested)).is_empty());
}

#[test]
fn a_closed_keys_clause_on_a_kind_declaring_no_keys_is_inadmissible() {
    // Vacuous in the mirror image of an empty `forbidden_keys`: over no declared key the
    // clause indicts every key of every member, which the author cannot have meant. The
    // contract fails admissibility rather than gating on it.
    let vacuous = contract(vec![
        clause(
            ClauseSeverity::Required,
            Predicate::Extent {
                unit: contract::ExtentUnit::Lines,
                max: 9,
                whole: false,
            },
        ),
        closes(ClauseSeverity::Required),
    ]);

    let diagnostics = engine::admissibility(&vacuous, &Locus::Document);
    assert_eq!(diagnostics.len(), 1);
    assert!(messages(&diagnostics)[0].contains("declares no `required` or `optional` key"));
    // Admissibility is never advisory: a contract that cannot be trusted cannot be used.
    assert_eq!(diagnostics[0].severity, Severity::Error);

    // One declaring row is all it takes to earn admission.
    let live = contract(vec![declares_name(), closes(ClauseSeverity::Required)]);
    assert!(engine::admissibility(&live, &Locus::Document).is_empty());
}

#[test]
fn an_embedded_member_is_no_less_decidable_than_a_document_one() {
    // `closed-keys` reads the member's own keys, and an embedded member's leaves *are*
    // fields — so unlike the body-shaped predicates it stays decidable off its host's
    // declared surface, and the bodyless fence never reaches it.
    let contract = contract(vec![declares_name(), closes(ClauseSeverity::Required)]);
    assert!(
        engine::admissibility(&contract, &Locus::Embedded("hook".to_string())).is_empty(),
        "`closed-keys` decides over an embedded member's own leaves"
    );
}
