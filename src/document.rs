//! The `satisfies` clause type and TOML-to-JSON conversion for member frontmatter.
//!
//! The `Satisfies` type models an authored `[satisfies.<requirement>]` opt-in
//! carrying an optional rationale. [`item_to_json`] and [`value_to_json`] convert
//! TOML Items and Values to JSON for member frontmatter parsing, so custom kinds
//! read authored `[clause.*]` values faithfully.

use serde_json::{Map as JsonMap, Value as JsonValue};
use toml_edit::{Item, Value};

/// A `[satisfies.<requirement>]` clause module: the member opts into filling `requirement`, carrying the optional
/// authored `rationale` — the *why*, first-class beside the link rather than
/// delegated and forgotten. Authored on the surface, never
/// imported; the coverage check reads only the requirement name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Satisfies {
    /// The declared requirement this member opts into filling.
    pub requirement: String,
    /// The authored rationale — never a decidable feature, so no check reads it.
    pub rationale: Option<String>,
}

impl Satisfies {
    /// A `satisfies` clause naming `requirement`, with no rationale.
    pub fn new(requirement: impl Into<String>) -> Self {
        Self {
            requirement: requirement.into(),
            rationale: None,
        }
    }
}

/// Convert a header [`Item`] to a [`serde_json::Value`] — the inverse of the
/// built-in adapters' `json_to_toml_value`, so a `[clause.<field>]` `value` lands
/// in a member's frontmatter the same shape a built-in's hand-written parser
/// produces. A JSON-null-unrepresentable item (`Item::None`; a bare TOML `Datetime`
/// is kept as its string form) yields `None`, dropped rather than invented. Recurses
/// through tables and arrays so a nested clause value round-trips.
#[must_use]
pub fn item_to_json(item: &Item) -> Option<JsonValue> {
    match item {
        Item::Value(val) => value_to_json(val),
        Item::Table(table) => {
            let mut map = JsonMap::new();
            for (key, child) in table.iter() {
                if let Some(json) = item_to_json(child) {
                    map.insert(key.to_string(), json);
                }
            }
            Some(JsonValue::Object(map))
        }
        Item::ArrayOfTables(tables) => Some(JsonValue::Array(
            tables
                .iter()
                .map(|t| item_to_json(&Item::Table(t.clone())))
                .collect::<Option<Vec<_>>>()?,
        )),
        Item::None => None,
    }
}

/// Convert a header [`Value`] to a [`serde_json::Value`]; a TOML `Datetime` carries
/// as its string form (JSON has no datetime scalar).
fn value_to_json(val: &Value) -> Option<JsonValue> {
    Some(match val {
        Value::String(s) => JsonValue::from(s.value().clone()),
        Value::Integer(i) => JsonValue::from(*i.value()),
        Value::Float(f) => JsonValue::from(*f.value()),
        Value::Boolean(b) => JsonValue::from(*b.value()),
        Value::Datetime(d) => JsonValue::from(d.value().to_string()),
        Value::Array(array) => JsonValue::Array(array.iter().filter_map(value_to_json).collect()),
        Value::InlineTable(inline) => {
            let mut map = JsonMap::new();
            for (key, child) in inline.iter() {
                if let Some(json) = value_to_json(child) {
                    map.insert(key.to_string(), json);
                }
            }
            JsonValue::Object(map)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_to_json_converts_each_clause_value_kind_faithfully() {
        // item_to_json converts each TOML value kind to JSON faithfully: string,
        // int, bool, array, and inline table. Parse TOML text directly with the
        // `[clause.field]` structure, extracting value items the same way.
        use toml_edit::DocumentMut;
        let toml_text = "[clause.name]\nvalue = \"demo\"\n\
[clause.priority]\nvalue = 7\n\
[clause.enabled]\nvalue = true\n\
[clause.tools]\nvalue = [\"Bash\", \"Read\"]\n\
[clause.meta]\nvalue = { team = \"core\" }\n";

        let doc: DocumentMut = toml_text.parse().unwrap();
        let clause_table = doc.get("clause").and_then(|t| t.as_table()).unwrap();

        // Extract and convert each field's value item.
        let by_field: std::collections::BTreeMap<String, JsonValue> = clause_table
            .iter()
            .filter_map(|(field, module)| {
                module
                    .as_table()
                    .and_then(|m| m.get("value"))
                    .and_then(|value| item_to_json(value).map(|json| (field.to_string(), json)))
            })
            .collect();

        // Verify each value kind converts faithfully.
        assert_eq!(by_field["name"], JsonValue::String("demo".to_string()));
        assert_eq!(by_field["priority"], JsonValue::from(7));
        assert_eq!(by_field["enabled"], JsonValue::Bool(true));
        assert_eq!(by_field["tools"], serde_json::json!(["Bash", "Read"]));
        assert_eq!(by_field["meta"], serde_json::json!({ "team": "core" }));
    }
}
