//! Schema emission — project the active contract into an editor JSON Schema.
//!
//! Implements `specs/50-distribution.md` ("The gate at keystroke — the emitted
//! schema"): `temper schema [--kind]` emits a JSON Schema **from the active
//! contract** (the by-kind floor ⊕ `temper.toml`, via [`crate::compose::effective`])
//! so an editor validates a harness artifact's frontmatter at keystroke — the one
//! gate, shifted as far left as the work allows.
//!
//! ## The validation channel only
//!
//! The spec's schema carries two channels; this module emits **only the
//! validation one** — the *decidable clauses*, each a true positive by
//! construction, so the squiggle never cries wolf (`00-intent.md` law 3). The
//! docs/hover channel (per-field guidance prose) is *deliberately not built*: the
//! [`Contract`] model carries no guidance-prose field, so there is no source to
//! project — wiring one is a separate change, not silently faked here. Taste
//! cannot become a squiggle, and it does not become one here.
//!
//! ## What maps, and what does not
//!
//! Each decidable field/structural clause projects to its JSON-Schema keyword:
//! `required`→`required[]`, `type`→`type` (over the closed lattice, with `list`
//! and `map` spelled `array`/`object` as JSON Schema names them), `min_len`/
//! `max_len`→`minLength`/`maxLength`, `enum`→`enum`, `deny`→`not`/`enum`,
//! `range`→`minimum`/`maximum`, `allowed_chars`→a generated `pattern` charclass,
//! and `forbidden_keys`→a `not`/`required` combinator per key. The remaining
//! predicates name no frontmatter JSON-Schema keyword — a body budget
//! (`max_lines`), a section requirement (`require_sections`), a body marker
//! (`must_define`), the `optional` documentation clause, and the cross-artifact
//! predicates (`name-matches-dir`, `unique-name`, `dependency-exists`) — so they
//! ride no channel here. The emitted validation keywords are therefore *exactly*
//! the decidable clauses the editor can decide against a single artifact's
//! frontmatter.

use serde_json::{Map, Value};

use crate::contract::{Charset, Contract, Predicate};
use crate::extract::Kind;

/// Project `contract` into a JSON Schema [`Value`] over an artifact's frontmatter.
///
/// The result is an `object` schema whose `properties` carry the per-field
/// refinements (`type`/`minLength`/`maxLength`/`enum`/`not`/`minimum`/`maximum`/
/// `pattern`), whose `required` array lists the present-required fields, and whose
/// `allOf` forbids each `forbidden_keys` key. A contract with no *mappable* clause
/// yields the empty-but-valid `{ "$schema", "type": "object" }` — a schema that
/// asserts nothing, exactly as a vacuous contract gates nothing.
#[must_use]
pub fn emit(contract: &Contract) -> Value {
    let mut properties: Map<String, Value> = Map::new();
    let mut required: Vec<String> = Vec::new();
    let mut forbidden: Vec<String> = Vec::new();

    for clause in &contract.clauses {
        match &clause.predicate {
            Predicate::Required { field } => push_unique(&mut required, field),
            Predicate::Type { field, kind } => {
                property(&mut properties, field)
                    .insert("type".to_string(), Value::from(json_type(*kind)));
            }
            Predicate::MinLen { field, min } => {
                property(&mut properties, field).insert("minLength".to_string(), Value::from(*min));
            }
            Predicate::MaxLen { field, max } => {
                property(&mut properties, field).insert("maxLength".to_string(), Value::from(*max));
            }
            Predicate::Range { field, min, max } => {
                let prop = property(&mut properties, field);
                prop.insert("minimum".to_string(), Value::from(*min));
                prop.insert("maximum".to_string(), Value::from(*max));
            }
            Predicate::Enum { field, values } => {
                property(&mut properties, field).insert("enum".to_string(), string_array(values));
            }
            Predicate::Deny { field, values } => {
                // `deny` is the negation of `enum`: the value must be *none* of the
                // forbidden set — `not: { enum: [...] }`.
                let mut not = Map::new();
                not.insert("enum".to_string(), string_array(values));
                property(&mut properties, field).insert("not".to_string(), Value::Object(not));
            }
            Predicate::AllowedChars { field, charset } => {
                property(&mut properties, field)
                    .insert("pattern".to_string(), Value::from(charclass(charset)));
            }
            Predicate::ForbiddenKeys { keys } => {
                for key in keys {
                    push_unique(&mut forbidden, key);
                }
            }
            // The remaining predicates name no frontmatter JSON-Schema keyword —
            // `optional` is documentation, `max_lines`/`require_sections`/
            // `must_define` are body/structural, and the cross-artifact predicates
            // range over the whole corpus. None is a per-artifact frontmatter
            // squiggle, so none rides the validation channel here.
            Predicate::Optional { .. }
            | Predicate::MaxLines { .. }
            | Predicate::RequireSections { .. }
            | Predicate::MustDefine { .. }
            | Predicate::NameMatchesDir
            | Predicate::UniqueName
            | Predicate::DependencyExists => {}
        }
    }

    let mut schema = Map::new();
    schema.insert(
        "$schema".to_string(),
        Value::from("http://json-schema.org/draft-07/schema#"),
    );
    schema.insert("type".to_string(), Value::from("object"));
    if !properties.is_empty() {
        schema.insert("properties".to_string(), Value::Object(properties));
    }
    if !required.is_empty() {
        schema.insert("required".to_string(), string_array(&required));
    }
    if !forbidden.is_empty() {
        // A forbidden key must be *absent*; the JSON-Schema idiom is
        // `not: { required: [key] }`, one per key so any single present key fails
        // (a single `not: { required: [a, b] }` would fire only when *both* are
        // present — the wrong reading).
        let clauses: Vec<Value> = forbidden.iter().map(|key| forbid_key(key)).collect();
        schema.insert("allOf".to_string(), Value::Array(clauses));
    }
    Value::Object(schema)
}

/// The JSON-Schema `type` name for a lattice [`Kind`]. The scalar kinds share
/// their spelling; the two containers are renamed to JSON Schema's own vocabulary
/// (`list`→`array`, `map`→`object`), a faithful projection of the same closed
/// lattice.
fn json_type(kind: Kind) -> &'static str {
    match kind {
        Kind::String => "string",
        Kind::Integer => "integer",
        Kind::Number => "number",
        Kind::Boolean => "boolean",
        Kind::Null => "null",
        Kind::List => "array",
        Kind::Map => "object",
    }
}

/// Get (or create) the `properties` entry for `field` as a mutable JSON object —
/// the accumulator each field-scoped clause adds its keyword to, so a field named
/// by several clauses collects all of them into one property schema.
fn property<'a>(properties: &'a mut Map<String, Value>, field: &str) -> &'a mut Map<String, Value> {
    properties
        .entry(field.to_string())
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
        // Invariant: the entry is only ever inserted as `Value::Object` above, so
        // it is always an object — never a foreign variant to unwrap.
        .expect("a property entry is always a JSON object")
}

/// The `not: { required: [key] }` combinator that forbids one key's presence.
fn forbid_key(key: &str) -> Value {
    let mut required = Map::new();
    required.insert(
        "required".to_string(),
        Value::Array(vec![Value::from(key.to_owned())]),
    );
    let mut not = Map::new();
    not.insert("not".to_string(), Value::Object(required));
    Value::Object(not)
}

/// The generated `^[<ranges><chars>]*$` character-class pattern for a [`Charset`]
/// — a faithful projection of the decidable `allowed_chars` clause, not an
/// authored regex. Ranges render `lo-hi`, then the individual chars follow, each
/// metacharacter escaped so the class stays a literal set.
fn charclass(charset: &Charset) -> String {
    let mut class = String::from("^[");
    for (lo, hi) in &charset.ranges {
        class.push_str(&escape_in_class(*lo));
        class.push('-');
        class.push_str(&escape_in_class(*hi));
    }
    for c in &charset.chars {
        class.push_str(&escape_in_class(*c));
    }
    class.push_str("]*$");
    class
}

/// Escape one character for safe literal use inside a `[...]` character class. The
/// four class metacharacters (`\`, `]`, `^`, `-`) are backslash-escaped; every
/// other character stands for itself.
fn escape_in_class(c: char) -> String {
    match c {
        '\\' | ']' | '^' | '-' => format!("\\{c}"),
        _ => c.to_string(),
    }
}

/// A JSON array of the given strings, cloned — the shape `enum`, `required`, and
/// the `not`/`enum` negation all serialize to.
fn string_array(values: &[String]) -> Value {
    Value::Array(values.iter().map(|v| Value::from(v.clone())).collect())
}

/// Append `value` to `list` unless it is already present, preserving declaration
/// order — so a field required (or forbidden) by two clauses appears once.
fn push_unique(list: &mut Vec<String>, value: &str) {
    if !list.iter().any(|existing| existing == value) {
        list.push(value.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{Clause, Contract, Predicate, Severity};
    use serde_json::json;
    use std::collections::BTreeSet;

    /// Wrap a predicate in a clause; severity is irrelevant to emission (the
    /// schema is the validation channel — severity rides the diagnostic, not the
    /// squiggle), so every clause here is `Required`.
    fn clause(predicate: Predicate) -> Clause {
        Clause {
            severity: Severity::Required,
            predicate,
        }
    }

    /// A contract exercising every *mappable* predicate, several piling onto the
    /// single `name` field so the accumulator is proven to merge them.
    fn representative() -> Contract {
        Contract {
            name: "skill".to_string(),
            clauses: vec![
                clause(Predicate::Required {
                    field: "name".to_string(),
                }),
                clause(Predicate::Type {
                    field: "name".to_string(),
                    kind: Kind::String,
                }),
                clause(Predicate::MaxLen {
                    field: "name".to_string(),
                    max: 64,
                }),
                clause(Predicate::Deny {
                    field: "name".to_string(),
                    values: vec!["anthropic".to_string(), "claude".to_string()],
                }),
                clause(Predicate::AllowedChars {
                    field: "name".to_string(),
                    charset: Charset {
                        ranges: vec![('a', 'z'), ('0', '9')],
                        chars: BTreeSet::from(['-']),
                    },
                }),
                clause(Predicate::MinLen {
                    field: "description".to_string(),
                    min: 1,
                }),
                clause(Predicate::Range {
                    field: "priority".to_string(),
                    min: 0.0,
                    max: 9.0,
                }),
                clause(Predicate::Enum {
                    field: "status".to_string(),
                    values: vec![
                        "draft".to_string(),
                        "active".to_string(),
                        "deprecated".to_string(),
                    ],
                }),
                clause(Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string(), "alwaysApply".to_string()],
                }),
                // A structural predicate that must NOT surface as a keyword.
                clause(Predicate::MaxLines { max: 500 }),
                clause(Predicate::NameMatchesDir),
            ],
        }
    }

    #[test]
    fn every_decidable_predicate_maps_to_its_json_schema_keyword() {
        let schema = emit(&representative());
        assert_eq!(
            schema,
            json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "maxLength": 64,
                        "not": { "enum": ["anthropic", "claude"] },
                        "pattern": "^[a-z0-9\\-]*$"
                    },
                    "description": { "minLength": 1 },
                    "priority": { "minimum": 0.0, "maximum": 9.0 },
                    "status": { "enum": ["draft", "active", "deprecated"] }
                },
                "required": ["name"],
                "allOf": [
                    { "not": { "required": ["globs"] } },
                    { "not": { "required": ["alwaysApply"] } }
                ]
            })
        );
    }

    #[test]
    fn structural_and_cross_artifact_predicates_ride_no_channel() {
        // A contract of *only* non-mappable predicates emits the same empty-but-
        // valid schema a vacuous contract does — none of them is a frontmatter
        // keyword, so none surfaces.
        let contract = Contract {
            name: "structural".to_string(),
            clauses: vec![
                clause(Predicate::MaxLines { max: 500 }),
                clause(Predicate::RequireSections {
                    sections: vec!["Usage".to_string()],
                }),
                clause(Predicate::MustDefine {
                    marker: "disable-model-invocation".to_string(),
                }),
                clause(Predicate::Optional {
                    field: "paths".to_string(),
                }),
                clause(Predicate::UniqueName),
                clause(Predicate::DependencyExists),
            ],
        };
        assert_eq!(
            emit(&contract),
            json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object"
            })
        );
    }

    #[test]
    fn allowed_chars_yields_the_expected_charclass_pattern() {
        // `]`, `^`, `-`, and `\` are the four class metacharacters — each escaped
        // so the class stays a literal set, ranges first then individual chars.
        let contract = Contract {
            name: "charset".to_string(),
            clauses: vec![clause(Predicate::AllowedChars {
                field: "id".to_string(),
                charset: Charset {
                    ranges: vec![('A', 'Z')],
                    chars: BTreeSet::from(['_', '-', ']', '^']),
                },
            })],
        };
        let pattern = emit(&contract)["properties"]["id"]["pattern"]
            .as_str()
            .unwrap()
            .to_string();
        // BTreeSet orders the chars: '-' < ']' < '^' < '_' by codepoint.
        assert_eq!(pattern, "^[A-Z\\-\\]\\^_]*$");
    }

    #[test]
    fn a_vacuous_contract_yields_an_empty_but_valid_schema() {
        let schema = emit(&Contract {
            name: "empty".to_string(),
            clauses: Vec::new(),
        });
        assert_eq!(
            schema,
            json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object"
            })
        );
    }

    #[test]
    fn the_emitted_json_round_trips_through_serde_json() {
        let schema = emit(&representative());
        let text = serde_json::to_string(&schema).unwrap();
        let reparsed: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(schema, reparsed);
    }

    #[test]
    fn a_field_required_by_two_clauses_lists_once() {
        let contract = Contract {
            name: "dup".to_string(),
            clauses: vec![
                clause(Predicate::Required {
                    field: "name".to_string(),
                }),
                clause(Predicate::Required {
                    field: "name".to_string(),
                }),
            ],
        };
        assert_eq!(emit(&contract)["required"], json!(["name"]));
    }

    #[test]
    fn the_container_kinds_are_renamed_to_json_schema_vocabulary() {
        let contract = Contract {
            name: "containers".to_string(),
            clauses: vec![
                clause(Predicate::Type {
                    field: "tags".to_string(),
                    kind: Kind::List,
                }),
                clause(Predicate::Type {
                    field: "meta".to_string(),
                    kind: Kind::Map,
                }),
            ],
        };
        let schema = emit(&contract);
        assert_eq!(schema["properties"]["tags"]["type"], "array");
        assert_eq!(schema["properties"]["meta"]["type"], "object");
    }
}
