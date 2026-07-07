//! The harness assembly's domain types — [`Requirement`], [`Edge`], [`EnforcementMode`]
//! — and [`effective`], which composes the lock's per-clause severity overrides onto
//! the embedded by-kind floor (`specs/model/pipeline.md`). A requirement's
//! set-/edge-scope demands ride ordinary [`contract::Clause`] values
//! (`specs/model/contract.md`); their predicate payloads ([`contract::EdgeBound`] and
//! friends) live in [`crate::contract`], not here.
//!
//! There is no reader in this module: every value here is populated from the lock's
//! declaration rows (`crate::drift::Declarations`), the sole producer since `emit`
//! compiles the SDK program. These are the shared shapes the gate lifts lock rows
//! into and [`crate::roster`]/[`crate::graph`]/[`crate::coverage`] range over —
//! the manifest era's reader (`TEMPER-TOML-ZERO`) retired with this file's parser.

use crate::contract::{self, Contract};
use crate::drift::ClauseRow;

/// The harness's declared **enforcement mode** — how firmly the guard binds a tool
/// call, split by where the finding goes: a closed vocabulary the author declares on
/// the root member (`specs/model/representation.md`, "The root member";
/// `specs/decisions/0005-mode-on-root-member.md`), never a stance temper bakes in.
/// Defaults to [`Warn`](EnforcementMode::Warn) (`specs/distribution.md`, "The
/// placements and their enforcement modes"; `specs/decisions/
/// 0006-guard-mode-vocabulary.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnforcementMode {
    /// Allows the call and records the finding out-of-band only — the next report,
    /// never the live session. The newly expressible tier; unreachable until an
    /// author declares it.
    Note,
    /// Allows the call and surfaces the finding in-band, into the live context. The
    /// default: enforcement mode is author-declared per placement, never assumed
    /// (`specs/intent.md` invariant 5).
    #[default]
    Warn,
    /// Denies the call.
    Block,
}

/// A declared **edge relationship** — a kind capability declared on the owning kind's
/// members (`specs/model/representation.md`). The owning kind is the edge *source*
/// (the implicit `from`); the relationship names its reference `field` and the target
/// `to` kind. [`crate::graph`] reads the field off each source artifact into edges,
/// then flags any route that resolves to no artifact of the target kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// The reference field read off each source artifact's frontmatter (via the
    /// `extra` catch-all). Its scalar value (or each element of a list value) names
    /// the target artifact.
    pub field: String,
    /// The artifact kind that owns the reference field — the edge *source*. A `from`
    /// naming an unmodeled kind yields no source artifacts, so the edge is inert.
    pub from: String,
    /// The artifact kind the reference resolves to — the edge *target*. The target
    /// kind must be one `temper` models, else no route can resolve (a
    /// graph-admissibility concern, [`crate::graph`]).
    pub to: String,
}

/// A named **requirement** — the harness's named obligation, declared in the
/// assembly's `[requirement.<name>]` (or lifted from a member's own published
/// obligation, `specs/model/contract.md`). **Every facet is optional
/// except the name.** Fill is by the artifact's opt-in `satisfies` alone — there is
/// no name-`match` selector.
///
/// `temper` **never interprets `means`** — it is authored intent the surface carries,
/// never a thing the engine judges (`specs/model/contract.md`, "requirement"). The decidable shadow is
/// what `check` gates: [`crate::coverage`] over the `satisfies` edges,
/// [`crate::roster`]/[`crate::graph`] over the **satisfier set** (the artifacts of
/// its `kind` that opt in via `satisfies`).
#[derive(Debug, Clone, PartialEq)]
pub struct Requirement {
    /// The requirement's name.
    pub name: String,
    /// The authored *intent*, stated in meaning not predicates. Carried verbatim and
    /// **never interpreted** (`specs/model/contract.md`, "requirement").
    pub means: Option<String>,
    /// The requirement's declared satisfier kind. Unlike the old name-`match`
    /// selector, this never narrows *which* opt-in artifacts are candidates —
    /// [`crate::roster`]/[`crate::graph`] draw the satisfier set kind-blind from
    /// every modeled kind, the opt-in `satisfies` join the sole filter
    /// (`specs/model/contract.md`, "selection": narrowing a selection is an
    /// each-grain clause over it, never a second selector). Present, it instead
    /// *sources* the shipped each-grain "every satisfier is kind K" clause
    /// [`crate::roster::check`] evaluates over that kind-blind set — a satisfier of
    /// a different kind is a finding, never a silent exclusion. Absent ⇒
    /// **kind-blind**: any artifact that opts in fills it, and no narrowing clause
    /// attaches at all.
    pub kind: Option<String>,
    /// Whether an unfilled requirement is a gate-blocking violation. Absent ⇒ `false`
    /// (`temper` never fabricates a gate the author did not declare — `specs/intent.md`,
    /// "Declared, never mined"). Never cardinality — posture and the set-scope `count` clause in
    /// [`clauses`](Requirement::clauses) are different kinds of thing.
    pub required: bool,
    /// The requirement's set-/edge-scope demands — ordinary [`contract::Clause`]
    /// values whose predicates range over the satisfier set and its graph
    /// neighborhood (`count`/`unique`/`membership`/`degree`,
    /// `specs/model/contract.md`, "Decision: set-scope demands are
    /// clauses"). Each carries its own severity/guidance/cite; empty ⇒ no set-scope
    /// demand at all. `count`/`unique`/`membership` are checked in
    /// [`crate::roster`]; `degree` ranges over the *edge* graph, so it is checked in
    /// [`crate::graph`] instead.
    pub clauses: Vec<contract::Clause>,
    /// An optional external verifier for the behavioral remainder (`verified_by`).
    /// Stored verbatim; whether it *resolves* is an admissibility check.
    pub verified_by: Option<String>,
}

/// The effective contract for `kind`: the embedded `floor` with each clause's
/// severity overridden by a matching row in the lock's declared `clauses`
/// (`specs/model/pipeline.md`, "The lock": the
/// gate's per-kind contract sources its overrides from the lock's `ClauseRow`
/// family, never a manifest `[kind.*]` layer). A row overrides the floor clause
/// sharing its identity (predicate key + targeted field); a row naming no matching
/// floor clause contributes nothing — `effective` only ever flips an existing
/// clause's severity, never declares a wholly new one from a row's own argument
/// columns (`count`/`target`/`degree`). A row whose `severity` is outside the closed
/// vocabulary leaves the floor's own severity untouched, the same tolerant read the
/// rest of the lock takes over hand-editable state.
#[must_use]
pub fn effective(clauses: &[ClauseRow], kind: &str, mut floor: Contract) -> Contract {
    for clause in &mut floor.clauses {
        let key = clause.predicate.key();
        let target = clause.predicate.target();
        let overriding = clauses.iter().find(|row| {
            row.kind.as_deref() == Some(kind)
                && row.predicate == key
                && row.field.as_deref() == target
        });
        if let Some(severity) = overriding.and_then(|row| severity_from_label(&row.severity)) {
            clause.severity = severity;
        }
    }
    floor
}

/// A custom kind's whole floor [`Contract`], built directly from the clause rows
/// naming it in the committed lock (`specs/model/pipeline.md`, "The lock"). Unlike a
/// built-in kind — whose floor is the embedded default, with the lock's own rows only
/// overriding a clause's severity ([`effective`])
/// — a custom kind carries no embedded default: its committed rows **are** its floor,
/// the same lift [`crate::builtin::contract`] runs over the *embedded* lock's own rows,
/// run here over the *project's own*. Tolerant like the rest of a hand-editable lock's
/// readers: a row naming an unsupported predicate, an out-of-vocabulary severity, or one
/// missing its own required argument is skipped rather than trusted.
#[must_use]
pub fn floor_from_rows(clauses: &[ClauseRow], kind: &str) -> Contract {
    Contract {
        name: kind.to_string(),
        clauses: clauses
            .iter()
            .filter(|row| row.kind.as_deref() == Some(kind))
            .filter_map(|row| {
                Some(contract::Clause {
                    severity: severity_from_label(&row.severity)?,
                    predicate: crate::builtin::predicate_from_row(row)?,
                    guidance: row.guidance.clone(),
                    source: row.cite.clone(),
                })
            })
            .collect(),
        guidance: None,
    }
}

/// Parse a lock clause row's `severity` label into the typed [`contract::Severity`]
/// — the closed `required`/`advisory` vocabulary a bare contract's own clauses
/// declare. An out-of-vocabulary label is `None`. `pub` (not `pub(crate)`) so the
/// `main` binary's lift of a requirement's own clause rows reuses the identical
/// parse, never a second copy.
pub fn severity_from_label(label: &str) -> Option<contract::Severity> {
    match label {
        "required" => Some(contract::Severity::Required),
        "advisory" => Some(contract::Severity::Advisory),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::contract::{Clause, Predicate, Severity};

    /// A small skill-shaped floor: a required `max_len` on `name`, a required
    /// `forbidden_keys`, and an advisory `max_lines`. Enough distinct identities to
    /// exercise override-vs-extend.
    fn floor() -> Contract {
        Contract {
            name: "skill.anthropic".to_string(),
            guidance: None,
            clauses: vec![
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::MaxLen {
                        field: "name".to_string(),
                        max: 64,
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::ForbiddenKeys {
                        keys: vec!["globs".to_string()],
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::MaxLines { max: 500 },
                },
            ],
        }
    }

    #[test]
    fn effective_with_no_clause_rows_yields_the_floor_unchanged() {
        assert_eq!(effective(&[], "skill", floor()), floor());
    }

    #[test]
    fn effective_overrides_a_floor_clauses_severity_by_matching_identity() {
        // A row sharing the floor's `forbidden_keys` identity (predicate key, no
        // targeted field) flips its severity in place — the lock's `ClauseRow`
        // family is the sole source `effective` composes from, never a manifest
        // `[kind.*]` layer.
        let row = ClauseRow {
            kind: Some("skill".to_string()),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "advisory".to_string(),
            guidance: None,
            cite: None,
            count: None,
            target: None,
            degree: None,
            bound: None,
            charset: None,
            keys: None,
            values: None,
        };
        let contract = effective(&[row], "skill", floor());
        assert_eq!(contract.clauses.len(), floor().clauses.len());
        assert_eq!(contract.clauses[1].severity, Severity::Advisory);
    }

    #[test]
    fn effective_ignores_a_row_naming_a_different_kind() {
        let row = ClauseRow {
            kind: Some("rule".to_string()),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "advisory".to_string(),
            guidance: None,
            cite: None,
            count: None,
            target: None,
            degree: None,
            bound: None,
            charset: None,
            keys: None,
            values: None,
        };
        assert_eq!(effective(&[row], "skill", floor()), floor());
    }

    #[test]
    fn effective_ignores_a_row_with_no_matching_floor_clause() {
        // The row names a predicate/field pair the floor doesn't carry —
        // `effective` never reconstructs a wholly new clause from a row's own
        // argument columns, so an unmatched row contributes nothing.
        let row = ClauseRow {
            kind: Some("skill".to_string()),
            predicate: "min_len".to_string(),
            field: Some("name".to_string()),
            severity: "required".to_string(),
            guidance: None,
            cite: None,
            count: None,
            target: None,
            degree: None,
            bound: None,
            charset: None,
            keys: None,
            values: None,
        };
        assert_eq!(effective(&[row], "skill", floor()), floor());
    }

    #[test]
    fn floor_from_rows_builds_a_custom_kinds_whole_floor() {
        // Unlike `effective`, a custom kind has no embedded default to override — its
        // committed rows are its whole floor, so a matching row contributes a brand new
        // clause rather than only flipping an existing one's severity.
        let rows = vec![
            ClauseRow {
                kind: Some("spec".to_string()),
                predicate: "max_lines".to_string(),
                field: None,
                severity: "advisory".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                bound: Some(crate::drift::BoundRow {
                    min: None,
                    max: Some(150),
                }),
                charset: None,
                keys: None,
                values: None,
            },
            ClauseRow {
                kind: Some("rule".to_string()),
                predicate: "max_lines".to_string(),
                field: None,
                severity: "required".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                bound: Some(crate::drift::BoundRow {
                    min: None,
                    max: Some(10),
                }),
                charset: None,
                keys: None,
                values: None,
            },
        ];

        let contract = floor_from_rows(&rows, "spec");
        assert_eq!(contract.name, "spec");
        assert_eq!(
            contract.clauses,
            vec![Clause {
                severity: Severity::Advisory,
                predicate: Predicate::MaxLines { max: 150 },
                guidance: None,
                source: None,
            }]
        );
    }

    #[test]
    fn floor_from_rows_skips_a_row_it_cannot_lift() {
        // An unsupported predicate and an out-of-vocabulary severity both degrade to
        // absent, the tolerant read the rest of a hand-editable lock takes.
        let rows = vec![
            ClauseRow {
                kind: Some("spec".to_string()),
                predicate: "section_contains".to_string(),
                field: None,
                severity: "advisory".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                bound: None,
                charset: None,
                keys: None,
                values: None,
            },
            ClauseRow {
                kind: Some("spec".to_string()),
                predicate: "max_lines".to_string(),
                field: None,
                severity: "blocking".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                bound: Some(crate::drift::BoundRow {
                    min: None,
                    max: Some(150),
                }),
                charset: None,
                keys: None,
                values: None,
            },
        ];

        assert!(floor_from_rows(&rows, "spec").clauses.is_empty());
    }

    #[test]
    fn effective_ignores_a_row_with_an_out_of_vocabulary_severity() {
        let row = ClauseRow {
            kind: Some("skill".to_string()),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "blocking".to_string(),
            guidance: None,
            cite: None,
            count: None,
            target: None,
            degree: None,
            bound: None,
            charset: None,
            keys: None,
            values: None,
        };
        assert_eq!(effective(&[row], "skill", floor()), floor());
    }
}
