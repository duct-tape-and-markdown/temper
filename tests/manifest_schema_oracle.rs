//! The `plugin-manifest` coverage oracle: the shipped clause set diffed against the
//! platform's published JSON Schema, vendored at `fixtures/plugin-manifest/schema.json`
//! from <https://json.schemastore.org/claude-code-plugin-manifest.json> (retrieved
//! 2026-07-16) — the `$schema` value the platform's own docs name.
//!
//! It reports and never gates the format: the diff is a number a human reads, so the lag
//! behind the platform's validation bar is stated rather than argued from scratch. A
//! schema-validation predicate is the rejected alternative — one clause carries one
//! severity and no per-rule guidance for every violation inside it.
//!
//! Vendored, never fetched at test time: a network read would make the gate
//! non-deterministic and offline-hostile.
//!
//! **The measured slice** is the schema's root object — its `required` list and each
//! top-level property's own keywords. Keywords nested inside a property's subschemas are
//! out of scope: they constrain values no clause can address today, so counting them would
//! inflate the lag with rules no widening on the books closes.

use std::collections::BTreeSet;

use serde_json::Value;

use temper::contract::Predicate;
use temper::extract::ValueType;

const SCHEMA: &str = include_str!("fixtures/plugin-manifest/schema.json");

/// Every rule the published schema states that the shipped clauses do not carry. Each
/// widening shrinks this list; re-stating it is how the shrink becomes a reviewable diff.
///
/// The shape of what is here: fifteen `type` rules over fields no clause types at all;
/// three union-typed *experimental* component fields, which no clause types either; and
/// `homepage`'s `format`, which names no widening — a URI check is not on the books.
///
/// The six documented component-path fields left with the `type` predicate's set
/// widening: `skills`, `commands`, `agents`, `hooks`, `mcpServers` and `lspServers` are
/// each gated over their whole documented union now. `monitors`, `outputStyles` and
/// `themes` are union-typed too and stay — not for want of the set, but because they are
/// components the shipped floor types with no clause at all.
const EXPECTED_LAG: &[&str] = &[
    "$schema: type=string",
    "author: type=object",
    "channels: type=array",
    "dependencies: type=array",
    "description: type=string",
    "homepage: format",
    "homepage: type=string",
    "license: type=string",
    "monitors: type in {array|string}",
    "name: type=string",
    "outputStyles: type in {array|string}",
    "repository: type=string",
    "settings: type=object",
    "themes: type in {array|string}",
    "userConfig: type=object",
    "version: type=string",
];

/// The JSON Schema spelling of a lattice kind. The lattice says `list`/`map` where the
/// schema says `array`/`object`; every other name coincides.
fn json_schema_type(kind: ValueType) -> &'static str {
    match kind {
        ValueType::List => "array",
        ValueType::Map => "object",
        ValueType::String => "string",
        ValueType::Integer => "integer",
        ValueType::Number => "number",
        ValueType::Boolean => "boolean",
        ValueType::Null => "null",
    }
}

/// The schema types one property names, deduplicated and ordered — one entry for a plain
/// `type`, the branch union for an `anyOf`.
///
/// The walk recurses through `anyOf` branches, because a branch may be an `anyOf` of its
/// own: the schema spells `commands`' string form as a two-branch `anyOf` over the same
/// `string` under different `pattern`s, and reading only the outer branches' own `type`
/// keys would drop the string form from a union the property plainly states. That is
/// still the property's *own* type union — the measured slice — and not a keyword nested
/// inside a value's subschema.
fn declared_types(property: &Value) -> Vec<String> {
    fn walk(schema: &Value, into: &mut BTreeSet<String>) {
        match schema.get("anyOf").and_then(Value::as_array) {
            Some(branches) => branches.iter().for_each(|branch| walk(branch, into)),
            None => {
                if let Some(named) = schema.get("type").and_then(Value::as_str) {
                    into.insert(named.to_string());
                }
            }
        }
    }
    let mut types = BTreeSet::new();
    walk(property, &mut types);
    types.into_iter().collect()
}

/// Every rule the vendored schema's root object states, in this file's rule-id vocabulary.
fn schema_rules() -> BTreeSet<String> {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    let mut rules = BTreeSet::new();

    for field in schema["required"].as_array().unwrap() {
        rules.insert(format!("{}: required", field.as_str().unwrap()));
    }

    for (field, property) in schema["properties"].as_object().unwrap() {
        match declared_types(property).as_slice() {
            [] => {}
            [one] => {
                rules.insert(format!("{field}: type={one}"));
            }
            many => {
                rules.insert(format!("{field}: type in {{{}}}", many.join("|")));
            }
        }
        for keyword in [
            "minLength",
            "maxLength",
            "enum",
            "const",
            "pattern",
            "format",
        ] {
            if property.get(keyword).is_some() {
                rules.insert(format!("{field}: {keyword}"));
            }
        }
    }

    rules
}

/// The schema rule this predicate carries, or `None` when it states a rule the schema does
/// not — the oracle measures the schema's rules, never the converse.
///
/// A `type` clause declares a set of kinds, so it spells its rule the same two ways
/// [`schema_rules`] does: `type=<one>` for a single kind, `type in {a|b}` for a union. A
/// clause covers a union-typed property only by declaring that property's whole union —
/// declaring a subset of it would reject a documented form, so it earns no coverage here
/// and is a rule the shipped floor may not carry in the first place.
fn covered_rule(predicate: &Predicate) -> Option<String> {
    Some(match predicate {
        Predicate::Required { field } => format!("{field}: required"),
        Predicate::MinLen { field, .. } => format!("{field}: minLength"),
        Predicate::MaxLen { field, .. } => format!("{field}: maxLength"),
        Predicate::Enum { field, .. } => format!("{field}: enum"),
        Predicate::Type { field, kinds } => {
            // The schema's own names, ordered as `schema_rules` orders them: the lattice
            // order the clause's set carries is not the schema vocabulary's.
            let named: BTreeSet<&str> = kinds.iter().map(|kind| json_schema_type(*kind)).collect();
            match named.iter().copied().collect::<Vec<&str>>().as_slice() {
                [one] => format!("{field}: type={one}"),
                many => format!("{field}: type in {{{}}}", many.join("|")),
            }
        }
        // The rest state rules the published schema does not: `allowed_chars` holds the
        // kebab-case bar the schema leaves to a `description`, `forbidden_keys` holds the
        // `--strict` experimental-component migration the schema still permits outright,
        // `closed-keys` holds the `--strict` unrecognized-field bar the schema declares no
        // `additionalProperties` for, and the body/selection predicates range over surfaces
        // a manifest has none of. `optional` declares a key the schema's `properties` also
        // names, but a *declared* key is no rule either side gates on — the schema states
        // rules about values, and this oracle measures rules.
        Predicate::Optional { .. }
        | Predicate::Range { .. }
        | Predicate::Deny { .. }
        | Predicate::ForbiddenKeys { .. }
        | Predicate::ClosedKeys
        | Predicate::AllowedChars { .. }
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
        | Predicate::GlobValid { .. }
        | Predicate::MentionReachable { .. }
        | Predicate::FormatPlacesEdges => return None,
    })
}

/// The shipped floor's coverage, read from the built-in lock's own clause rows rather than
/// a hand-written mirror of them.
fn covered_rules() -> BTreeSet<String> {
    temper::builtin::contract("plugin-manifest")
        .expect("plugin-manifest ships an embedded floor")
        .clauses
        .iter()
        .filter_map(|clause| covered_rule(&clause.predicate))
        .collect()
}

#[test]
fn the_lag_behind_the_published_schema_is_the_named_expected_set() {
    let lag: BTreeSet<String> = schema_rules()
        .difference(&covered_rules())
        .cloned()
        .collect();
    let expected: BTreeSet<String> = EXPECTED_LAG
        .iter()
        .map(|rule| (*rule).to_string())
        .collect();

    let closed: Vec<&String> = expected.difference(&lag).collect();
    let opened: Vec<&String> = lag.difference(&expected).collect();
    assert!(
        closed.is_empty() && opened.is_empty(),
        "the shipped clause coverage moved — re-state EXPECTED_LAG.\n\
         closed by a clause (drop from the list): {closed:?}\n\
         newly uncovered (add to the list): {opened:?}",
    );
}

#[test]
fn the_shipped_clauses_carry_the_rules_the_schema_and_the_vocabulary_agree_on() {
    // The other side of the diff: the lag is a real subtraction from a real intersection,
    // not the whole schema going uncovered because the read is wired wrong. Six of the
    // nine are the component-path unions, each covered by declaring the property's whole
    // documented set — the property the `type` widening bought.
    let covered: BTreeSet<String> = schema_rules()
        .intersection(&covered_rules())
        .cloned()
        .collect();

    assert_eq!(
        covered.iter().map(String::as_str).collect::<Vec<&str>>(),
        vec![
            "agents: type in {array|string}",
            "commands: type in {array|object|string}",
            "hooks: type in {array|object|string}",
            "keywords: type=array",
            "lspServers: type in {array|object|string}",
            "mcpServers: type in {array|object|string}",
            "name: minLength",
            "name: required",
            "skills: type in {array|string}",
        ],
    );
}

#[test]
fn the_published_schema_states_no_closed_key_rule_so_the_strict_bar_is_outside_this_diff() {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();

    // The `--strict` hold the contract header names — unrecognized top-level fields — is
    // measured by nothing here: the published schema declares no `additionalProperties`,
    // so an undeclared key validates clean against it. The `closed-keys` widening closes
    // that hold against the *docs*, and this oracle will not move when it lands.
    assert_eq!(schema.get("additionalProperties"), None);
    assert_eq!(schema["type"], "object");
}
