//! The shipped `dial` kind's read side: the severities this machine reads clauses at.
//!
//! `.temper/dial.toml` names clauses by the compiled address every finding already
//! prints, and declares a severity per entry. Severity is the only verb the schema has
//! — [`crate::builtin_kind`] declares the kind, `dialDefaultContract` closes its key set
//! — so deletion is unspellable and a dialed clause still reports.
//!
//! Two bounds hold here rather than in the schema, because neither is a shape.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value as JsonValue;

use crate::check::Diagnostic;
use crate::compose::{self, EnforcementMode};
use crate::contract::{Clause, Severity};
use crate::extract::Features;

/// The kind's bare name — temper's own, and the owner segment of every label it may not
/// name ([`Dial::refusals`]).
pub const KIND: &str = "dial";

/// The document the kind governs, under the workspace root.
pub const DOCUMENT: &str = "dial.toml";

/// The top-level key the entries sit under: `[[clause]]`.
const ENTRIES_KEY: &str = "clause";

/// The rule id of a finding about a dial *entry* — never a clause label, so it is
/// undialable, which is the point: an entry that says nothing must not be able to
/// silence the report that says so.
const ENTRY_RULE: &str = "dial.entry";

/// This machine's declared severities, keyed by the clause address each entry names.
///
/// An entry the schema would reject never lands here — a missing or non-string label, a
/// severity outside the closed two — because failing to apply is the safe direction and
/// the kind's own contract is what reports it. Silence is impossible: the entry survives
/// into [`Dial::refusals`] as a label nothing matched.
#[derive(Debug, Clone, Default)]
pub struct Dial {
    entries: BTreeMap<String, Severity>,
}

impl Dial {
    /// This machine's dial, read off the `dial` kind's extracted members.
    ///
    /// More than one dial document cannot arise — the kind governs one glob with no
    /// wildcard — but the read folds every member rather than assuming it, and a label
    /// two members share resolves to the last one folded.
    #[must_use]
    pub fn from_features(members: &[Features]) -> Self {
        let mut entries = BTreeMap::new();
        for member in members {
            let Some(JsonValue::Array(rows)) = member.fields.get(ENTRIES_KEY) else {
                continue;
            };
            for row in rows {
                let (Some(JsonValue::String(label)), Some(JsonValue::String(severity))) =
                    (row.get("label"), row.get("severity"))
                else {
                    continue;
                };
                if let Some(severity) = compose::severity_from_label(severity) {
                    entries.insert(label.clone(), severity);
                }
            }
        }
        Self { entries }
    }

    /// Whether this machine dials nothing — the common case, and the one every caller
    /// below is a no-op for.
    #[must_use]
    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Re-read every clause in `clauses` this dial names, at the severity it declares —
    /// returning the addresses it actually reached.
    ///
    /// Hardening (advisory → required) binds under every mode. Softening is the reviewed
    /// half: `mode` [`Block`](EnforcementMode::Block) leaves the authored severity
    /// standing, so the machine can only ever be stricter than the shared gate, never
    /// laxer. A clause the dial re-declares at the severity it already carries is neither,
    /// and applies silently.
    ///
    /// Idempotent, and called over overlapping clause sets on purpose: a set-grain clause
    /// reaches the judge through a selection assembled out of contracts this has already
    /// run over.
    pub fn apply(&self, mode: EnforcementMode, clauses: &mut [Clause]) -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        for clause in clauses.iter_mut() {
            let Some(&severity) = self.entries.get(&clause.label) else {
                continue;
            };
            reached.insert(clause.label.clone());
            let softens = clause.severity == Severity::Required && severity == Severity::Advisory;
            if softens && mode == EnforcementMode::Block {
                continue;
            }
            clause.severity = severity;
        }
        reached
    }

    /// Every entry that dialed nothing, as a finding — the two ways a dial can be wrong
    /// about the gate rather than malformed against its own schema.
    ///
    /// Both are errors, and both are strictly *stricter* than the shared gate: a machine
    /// that fails its own check over a stale dial has still never softened anything. The
    /// alternative — an entry that quietly names nothing — is the outcome the address's
    /// legibility exists to prevent.
    #[must_use]
    pub fn refusals(&self, reached: &BTreeSet<String>) -> Vec<Diagnostic> {
        let own_prefix = format!("{KIND}.");
        self.entries
            .keys()
            .filter(|label| !reached.contains(label.as_str()))
            .map(|label| {
                let message = if label.starts_with(&own_prefix) {
                    format!(
                        "dial entry `{label}` names the dial's own contract, which no dial \
                         may re-read: that contract is the envelope this document is \
                         checked against"
                    )
                } else {
                    format!(
                        "dial entry `{label}` names no clause this harness carries — spell \
                         the address a finding printed as its `rule` id, or drop the entry"
                    )
                };
                Diagnostic::error(ENTRY_RULE, DOCUMENT, message)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One dial member carrying `entries` under the entries key, and nothing else — the
    /// only feature this module's read looks at.
    fn dial_member(entries: Option<JsonValue>) -> Features {
        Features {
            id: "workstation".to_string(),
            fields: entries
                .into_iter()
                .map(|value| (ENTRIES_KEY.to_string(), value))
                .collect(),
            body_lines: 0,
            rendered_lines: Some(0),
            rendered_chars: Some(0),
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

    #[test]
    fn an_entry_outside_the_severity_vocabulary_never_lands() {
        // Failing to apply is the safe direction, and the kind's own `enum` clause is
        // what reports it — the read never invents a third severity's meaning.
        let dial = Dial::from_features(&[dial_member(Some(serde_json::json!([
            { "label": "skill.extent", "severity": "off" },
            { "label": "rule.required.description", "severity": "advisory" },
        ])))]);

        assert_eq!(
            dial.entries.keys().collect::<Vec<_>>(),
            vec!["rule.required.description"]
        );
    }

    #[test]
    fn an_entry_missing_either_half_never_lands() {
        let dial = Dial::from_features(&[dial_member(Some(serde_json::json!([
            { "label": "skill.extent" },
            { "severity": "advisory" },
            { "label": ["skill.extent"], "severity": "advisory" },
        ])))]);

        assert!(dial.is_empty());
    }

    #[test]
    fn a_dial_declaring_no_entries_at_all_is_empty() {
        assert!(Dial::from_features(&[dial_member(None)]).is_empty());
    }
}
