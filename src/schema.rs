//! Schema emission — project the active contract into an editor JSON Schema.
//!
//! The gate at keystroke — the emitted
//! schema: `temper schema [--kind]` emits a JSON Schema **from the active
//! contract** (the lock's declared clause rows for the kind when it names any, else
//! the embedded by-kind floor) so an editor validates a harness artifact's
//! frontmatter at keystroke — the one gate, shifted as far left as the work allows.
//!
//! ## Two channels, kept disjoint
//!
//! The spec's schema carries two channels, and the split is the guarantee:
//!
//! - **validation** (the squiggle) — the *decidable clauses only*, each a true
//!   positive by construction, so the squiggle never cries wolf. These are the
//!   JSON-Schema *validation* keywords ([`emit`] below).
//! - **docs** (hover) — the per-field [`guidance`](crate::contract::Clause::guidance)
//!   prose kept *out of checks*, projected onto each field's property
//!   `description` keyword, **strictly alongside** the validation keywords and
//!   never mixed into them. A field's guided clauses join into that one keyword,
//!   just as its shapes compose into `allOf`. Advisory; it never gates.
//!
//! Taste cannot become a squiggle — the closed algebra has no syntax for it, and
//! neither does the schema — so it can only ride the docs channel. The medium
//! enforces the keystroke: the editor delivers the decidable contract as validation and
//! the guidance as documentation, and cannot confuse the two.
//!
//! ## What maps, and what does not
//!
//! Each decidable field/structural clause projects to its JSON-Schema keyword:
//! `required`→`required[]`, `type`→`type` (over the closed lattice, with `list`
//! and `map` spelled `array`/`object` as JSON Schema names them), `min_len`/
//! `max_len`→`minLength`/`maxLength`, `enum`→`enum`, `deny`→`not`/`enum`,
//! `range`→`minimum`/`maximum`, `allowed_chars`→a generated `pattern` charclass,
//! `shape`→the named shape's own expression, composed into the property's `allOf`,
//! `forbidden_keys`→a `not`/`required` combinator per key, and `closed-keys`→the whole
//! object's `additionalProperties: false`, the one clause whose face is the object's
//! rather than a property's. The remaining
//! predicates name no frontmatter JSON-Schema keyword — a body budget
//! (`max_lines`), a section requirement (`require_sections`), a body marker
//! (`must_define`), the `optional` documentation clause, and the cross-artifact
//! predicates (`name-matches-dir`, `unique-name`, `dependency-exists`) — so they
//! ride no channel here. The emitted validation keywords are therefore *exactly*
//! the decidable clauses the editor can decide against a single artifact's
//! frontmatter.
//!
//! A clause whose `field` addresses past the top level (`owner.name`,
//! `plugins[*].source`) names no property of this object, so it rides neither channel:
//! a nested key spelled as a top-level one would have the editor demand a key the
//! format never documents. The gate decides those clauses; the schema says only what an
//! editor can check against the flat object in front of it.

use std::collections::BTreeSet;

use serde_json::{Map, Value};

use crate::contract::{self, Charset, Contract, Predicate, Shape};
use crate::extract::ValueType;

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
    let mut closed = false;

    for clause in &contract.clauses {
        // A clause whose `field` addresses past the top level names no property of this
        // schema: `owner.name` is a key of the *`owner` object*, and `plugins[*].source`
        // one of each array element. Spelling either as a top-level `required` key or
        // `properties` entry would have the editor demand a key the format never
        // documents — a forged squiggle at the keystroke, which is the one thing the
        // validation channel may never emit. The gate still decides them; the schema
        // carries the subset an editor can check against a flat frontmatter object.
        if !addresses_a_property(&clause.predicate) {
            continue;
        }
        match &clause.predicate {
            Predicate::Required { field } => push_unique(&mut required, field),
            Predicate::Type { field, kinds } => {
                property(&mut properties, field).insert("type".to_string(), json_types(kinds));
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
            // A shape is decidable against the flat object in front of the editor, so it
            // rides the validation channel like any other decidable field clause: the
            // engine's own expression, projected. That is not the author-facing `pattern`
            // clause arriving by the back door — the author named a shape, this is machine
            // output, and the same [`Shape::pattern`] the gate judges by is what lands
            // here, so the squiggle cannot say more or less than the gate does.
            Predicate::Shape { field, shape } => {
                push_subschema(property(&mut properties, field), shape_subschema(*shape));
            }
            Predicate::ForbiddenKeys { keys } => {
                for key in keys {
                    push_unique(&mut forbidden, key);
                }
            }
            // The one clause whose schema face is the whole object's rather than a
            // property's — the editor's `additionalProperties: false` is exactly "the
            // declared key set is exhaustive". Emitted after the loop, where the declared
            // set is known whichever order the clauses were authored in.
            Predicate::ClosedKeys => closed = true,
            // The remaining predicates name no frontmatter JSON-Schema keyword —
            // `optional` is documentation, `max_lines`/`require_sections`/
            // `must_define`/`section_contains` are body/structural, the
            // cross-artifact predicates range over the whole corpus, and
            // `count`/`unique`/`membership`/`degree`/`kind`/`format-places-edges`
            // range over a node-set or the edge graph, never a single artifact's
            // frontmatter. `glob-valid`
            // does name a field, but "parses under globset" is no JSON-Schema
            // keyword — the engine owns that check — so it emits none here; its
            // guidance still rides the field's `description` via `documented_field`.
            // None is a per-artifact frontmatter squiggle, so none rides the
            // validation channel here.
            Predicate::Optional { .. }
            | Predicate::MaxLines { .. }
            | Predicate::RequireSections { .. }
            | Predicate::MustDefine { .. }
            | Predicate::SectionContains { .. }
            | Predicate::NameMatchesDir
            | Predicate::UniqueName
            | Predicate::DependencyExists
            | Predicate::Count { .. }
            | Predicate::Unique { .. }
            | Predicate::Membership { .. }
            | Predicate::Degree { .. }
            | Predicate::Kind { .. }
            | Predicate::FormatPlacesEdges
            // `mention-reachable` reads the *mentioned* member's gate field across the
            // graph, so it constrains no property of the document being validated —
            // keystroke validation is decidable over one document's own frontmatter,
            // and this predicate is not. The schema channel is honestly silent here.
            | Predicate::MentionReachable { .. }
            | Predicate::GlobValid { .. } => {}
        }
    }

    // The docs (hover) channel, emitted **strictly alongside** the validation
    // keywords above, never mixed into them: a field clause's advisory `guidance` prose rides its JSON
    // Schema property's `description`, joining whatever the field's earlier clauses
    // taught. This is the on-law guarantee made concrete —
    // taste can only become documentation, never a squiggle. Guidance on a
    // field-less predicate (`forbidden_keys`, `max_lines`, the cross-artifact ones)
    // names no frontmatter property, so it rides no channel here, exactly as those
    // predicates' validation does not. Absent guidance ⇒ no `description`.
    for clause in &contract.clauses {
        if !addresses_a_property(&clause.predicate) {
            continue;
        }
        if let (Some(guidance), Some(field)) =
            (&clause.guidance, clause.predicate.documented_field())
        {
            push_description(property(&mut properties, field), guidance);
        }
    }

    // A closed key set only reads correctly once every declared key is a named property:
    // `additionalProperties: false` rejects whatever `properties` does not list, and a key
    // whose only clause has no schema face (`optional`, and any clause the property gate
    // above skipped) would otherwise be squiggled as unrecognized — a forged error over a
    // key the contract declares. An empty subschema names the key while asserting nothing
    // about its value, which is exactly what `optional` says.
    if closed {
        for key in contract::declared_keys(&contract.clauses) {
            property(&mut properties, &key);
        }
    }

    let mut schema = Map::new();
    schema.insert(
        "$schema".to_string(),
        Value::from("http://json-schema.org/draft-07/schema#"),
    );
    schema.insert("type".to_string(), Value::from("object"));
    if closed {
        schema.insert("additionalProperties".to_string(), Value::from(false));
    }
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

/// Whether this predicate's `field` names a **top-level property** of the frontmatter
/// object this schema describes — true for a bare name, false for a path that steps into
/// an object or grains over an array's elements.
///
/// A predicate that documents no frontmatter property (`forbidden_keys`, the body and
/// cross-artifact ones) addresses none either way, so it passes through to the per-
/// predicate mapping below, which is where its silence is already decided.
fn addresses_a_property(predicate: &Predicate) -> bool {
    let Some(field) = predicate.documented_field() else {
        return true;
    };
    crate::address::FieldPath::parse(field).is_ok_and(|path| path.is_bare_name())
}

/// The JSON-Schema `type` keyword for a `type` clause's declared set. JSON Schema
/// spells a union of kinds as an array of type names and a single kind as a bare one,
/// so the projection is the clause's own set said in the schema's vocabulary — a
/// one-element set still emits the bare name, exactly as before the widening.
///
/// The array is in lattice order (the set's own), never the author's write order.
fn json_types(kinds: &BTreeSet<ValueType>) -> Value {
    match kinds.iter().copied().collect::<Vec<ValueType>>().as_slice() {
        [one] => Value::from(json_type(*one)),
        many => Value::Array(
            many.iter()
                .map(|kind| Value::from(json_type(*kind)))
                .collect(),
        ),
    }
}

/// The JSON-Schema `type` name for a lattice [`ValueType`]. The scalar kinds share
/// their spelling; the two containers are renamed to JSON Schema's own vocabulary
/// (`list`→`array`, `map`→`object`), a faithful projection of the same closed
/// lattice.
fn json_type(kind: ValueType) -> &'static str {
    match kind {
        ValueType::String => "string",
        ValueType::Integer => "integer",
        ValueType::Number => "number",
        ValueType::Boolean => "boolean",
        ValueType::Null => "null",
        ValueType::List => "array",
        ValueType::Map => "object",
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

/// The JSON-Schema face of one declared [`Shape`]: the engine's own expression as a
/// `pattern`, wrapped in `not` where matching it is the violation — the spelling `deny`
/// already uses to say "none of these".
fn shape_subschema(shape: Shape) -> Value {
    let mut pattern = Map::new();
    pattern.insert("pattern".to_string(), Value::from(shape.pattern()));
    if shape.match_holds() {
        return Value::Object(pattern);
    }
    let mut negated = Map::new();
    negated.insert("not".to_string(), Value::Object(pattern));
    Value::Object(negated)
}

/// Add `subschema` to a property's `allOf`, the composition keyword.
///
/// A shape's keywords compose rather than overwrite: `allowed_chars` already writes the
/// property's own `pattern` and `deny` its `not`, and a skill's `name` carries both plus a
/// shape. Writing a second `pattern` straight onto the property would leave only the last
/// clause's standing — silently dropping the other from the schema, with nothing to catch
/// it. `allOf` accumulates, so every clause on a field is asserted.
fn push_subschema(property: &mut Map<String, Value>, subschema: Value) {
    if let Some(entries) = property
        .entry("allOf")
        .or_insert_with(|| Value::Array(Vec::new()))
        .as_array_mut()
    {
        entries.push(subschema);
    }
}

/// Append `prose` to a property's `description`, the docs channel's one slot.
///
/// Guidance faces the same hazard [`push_subschema`] answers, one channel over: a field
/// carrying several guided clauses — a skill's `name` carries six — has one `description`
/// to reach the author through, and writing each straight onto the property would leave
/// only the last clause's teaching, silently dropping the rest. Paragraphs join in the
/// contract's compiled clause order, so adding a clause adds prose rather than re-picking
/// which teaching an author sees at the keystroke.
fn push_description(property: &mut Map<String, Value>, prose: &str) {
    let joined = match property.get("description").and_then(Value::as_str) {
        Some(existing) => format!("{existing}\n\n{prose}"),
        None => prose.to_string(),
    };
    property.insert("description".to_string(), Value::from(joined));
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
            label: crate::contract::clause_label(Some("skill"), predicate.key(), None),
            source: None,
            severity: Severity::Required,
            guidance: None,
            predicate,
        }
    }

    /// A contract exercising every *mappable* predicate, several piling onto the
    /// single `name` field so the accumulator is proven to merge them.
    fn representative() -> Contract {
        Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses: vec![
                clause(Predicate::Required {
                    field: "name".to_string(),
                }),
                clause(Predicate::Type {
                    field: "name".to_string(),
                    kinds: BTreeSet::from([ValueType::String]),
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
            guidance: None,
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
            guidance: None,
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
            guidance: None,
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
            guidance: None,
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
            guidance: None,
            clauses: vec![
                clause(Predicate::Type {
                    field: "tags".to_string(),
                    kinds: BTreeSet::from([ValueType::List]),
                }),
                clause(Predicate::Type {
                    field: "meta".to_string(),
                    kinds: BTreeSet::from([ValueType::Map]),
                }),
            ],
        };
        let schema = emit(&contract);
        assert_eq!(schema["properties"]["tags"]["type"], "array");
        assert_eq!(schema["properties"]["meta"]["type"], "object");
    }

    #[test]
    fn a_declared_set_of_kinds_emits_json_schemas_own_type_array() {
        // JSON Schema spells a union as an array of type names, so the clause's set
        // needs no encoding of temper's own: the squiggle admits exactly the forms the
        // clause does, and the editor validates the documented `string|array` field
        // rather than half of it.
        let contract = Contract {
            name: "unions".to_string(),
            guidance: None,
            clauses: vec![clause(Predicate::Type {
                field: "skills".to_string(),
                kinds: BTreeSet::from([ValueType::String, ValueType::List]),
            })],
        };
        assert_eq!(
            emit(&contract)["properties"]["skills"]["type"],
            json!(["string", "array"]),
        );
    }

    /// A field clause carrying `guidance` — the docs (hover) channel.
    fn guided(predicate: Predicate, guidance: &str) -> Clause {
        Clause {
            label: crate::contract::clause_label(Some("skill"), predicate.key(), None),
            source: None,
            severity: Severity::Advisory,
            guidance: Some(guidance.to_string()),
            predicate,
        }
    }

    #[test]
    fn guidance_rides_the_property_description_alongside_validation() {
        // A field clause's `guidance` becomes the property's `description`, sitting
        // *beside* the validation keyword the same field carries — never mixed into
        // it. `name` carries a `max_len` (validation) and guidance (docs);
        // `description` carries only a `min_len` and no guidance, so it gets no
        // `description` keyword.
        let contract = Contract {
            name: "docs".to_string(),
            guidance: None,
            clauses: vec![
                guided(
                    Predicate::MaxLen {
                        field: "name".to_string(),
                        max: 64,
                    },
                    "keep the skill name short and slug-like",
                ),
                clause(Predicate::MinLen {
                    field: "description".to_string(),
                    min: 1,
                }),
            ],
        };
        let schema = emit(&contract);
        assert_eq!(
            schema["properties"]["name"],
            json!({
            "maxLength": 64,
                "description": "keep the skill name short and slug-like"
            })
        );
        // Absent guidance ⇒ no `description` keyword on the property.
        assert_eq!(
            schema["properties"]["description"],
            json!({ "minLength": 1 })
        );
        assert!(
            schema["properties"]["description"]
                .get("description")
                .is_none()
        );
    }

    #[test]
    fn every_guided_clause_on_a_field_keeps_its_teaching() {
        // One `description` slot, several guided clauses on the field — a skill's `name`
        // carries six. Each teaching joins in clause order rather than overwriting the
        // last, so the author hovers on the whole contract, not on whichever clause the
        // compiler happened to emit last.
        let contract = Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses: vec![
                guided(
                    Predicate::Required {
                        field: "name".to_string(),
                    },
                    "every skill declares a name",
                ),
                guided(
                    Predicate::MaxLen {
                        field: "name".to_string(),
                        max: 64,
                    },
                    "64 characters is the cap the loader enforces",
                ),
                guided(
                    Predicate::AllowedChars {
                        field: "name".to_string(),
                        charset: Charset {
                            ranges: vec![('a', 'z')],
                            chars: BTreeSet::from(['-']),
                        },
                    },
                    "kebab-case, no spaces",
                ),
            ],
        };
        let schema = emit(&contract);
        assert_eq!(
            schema["properties"]["name"]["description"],
            json!(
                "every skill declares a name\n\n\
                 64 characters is the cap the loader enforces\n\n\
                 kebab-case, no spaces"
            )
        );
        // The joined prose sits strictly beside the validation keywords, never inside one.
        assert_eq!(schema["properties"]["name"]["maxLength"], json!(64));
        assert_eq!(schema["required"], json!(["name"]));
    }

    #[test]
    fn guidance_never_becomes_a_validation_keyword() {
        // Guidance rides `description` only; it never appears as a validation
        // keyword and never lands at the schema root. A `required` field clause
        // carrying guidance still projects `required[]` (validation) *and* a
        // property `description` (docs), the two disjoint.
        let contract = Contract {
            name: "law".to_string(),
            guidance: None,
            clauses: vec![guided(
                Predicate::Required {
                    field: "name".to_string(),
                },
                "every skill declares a name",
            )],
        };
        let schema = emit(&contract);
        // Validation channel: `name` is required.
        assert_eq!(schema["required"], json!(["name"]));
        // Docs channel: the guidance is the property `description`, nothing else.
        assert_eq!(
            schema["properties"]["name"],
            json!({ "description": "every skill declares a name" })
        );
        // The prose never leaked into a validation keyword: no `enum`/`pattern`/
        // `const` carries it, and it is not a root-level key.
        let text = serde_json::to_string(&schema).unwrap();
        assert!(!text.contains("\"enum\""));
        assert!(!text.contains("\"pattern\""));
        assert!(schema.get("description").is_none());
    }

    #[test]
    fn guidance_on_a_field_less_predicate_rides_no_channel() {
        // A field-less predicate (`forbidden_keys`) names no frontmatter property,
        // so guidance authored on it has nowhere to ride — exactly as its
        // validation projects to a root `allOf`, not a property. The schema is the
        // same one the un-guided clause would emit.
        let contract = Contract {
            name: "fieldless".to_string(),
            guidance: None,
            clauses: vec![guided(
                Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string()],
                },
                "Cursor keys Claude Code ignores",
            )],
        };
        let schema = emit(&contract);
        assert!(schema.get("properties").is_none());
        let text = serde_json::to_string(&schema).unwrap();
        assert!(!text.contains("Cursor keys"));
    }

    /// A contract closing the key set over three declared keys, one of them declared by a
    /// nested path and one carrying a refinement of its own.
    fn closed() -> Contract {
        Contract {
            name: "plugin-manifest".to_string(),
            guidance: None,
            clauses: vec![
                clause(Predicate::Required {
                    field: "name".to_string(),
                }),
                clause(Predicate::MinLen {
                    field: "name".to_string(),
                    min: 1,
                }),
                clause(Predicate::Optional {
                    field: "keywords".to_string(),
                }),
                clause(Predicate::Required {
                    field: "author.name".to_string(),
                }),
                clause(Predicate::ClosedKeys),
            ],
        }
    }

    #[test]
    fn closed_keys_projects_the_objects_own_additional_properties_keyword() {
        // The one clause whose schema face is the whole object's: `additionalProperties:
        // false` *is* "the declared key set is exhaustive", so the editor squiggles an
        // unrecognized key at the keystroke rather than at the gate.
        let schema = emit(&closed());
        assert_eq!(schema["additionalProperties"], json!(false));
        assert_eq!(schema["type"], json!("object"));
    }

    #[test]
    fn a_closed_schema_names_every_declared_key_as_a_property() {
        // `additionalProperties: false` rejects whatever `properties` does not list, so a
        // key whose only clause has no schema face — `optional`, and the `author.name` path
        // the property gate skips — has to be named anyway. An empty subschema names it
        // while asserting nothing, which is what `optional` says. Leaving it out would
        // forge a squiggle over a key the contract declares.
        let schema = emit(&closed());
        let properties = schema["properties"].as_object().unwrap();
        assert_eq!(
            properties.keys().map(String::as_str).collect::<Vec<&str>>(),
            vec!["author", "keywords", "name"]
        );
        assert_eq!(properties["keywords"], json!({}));
        // The path declares its top-level key, and says nothing about the object's shape:
        // `author.name` is no property of *this* object.
        assert_eq!(properties["author"], json!({}));
        // A declared key with a refinement keeps it — naming the key does not blank it.
        assert_eq!(properties["name"], json!({"minLength": 1}));
    }

    #[test]
    fn an_open_contract_emits_no_additional_properties_keyword() {
        // The keyword is a clause's, never a default: a contract with no `closed-keys`
        // says nothing about keys it did not declare, and a schema that volunteered
        // `additionalProperties: false` would gate what no author asked to gate.
        assert!(
            emit(&representative())
                .get("additionalProperties")
                .is_none()
        );
    }
}
