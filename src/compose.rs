//! The harness assembly's domain types — [`Requirement`], [`Edge`], [`EnforcementMode`]
//! — and the lock-row lift [`default_contract_from_rows`], which builds a kind's whole
//! default contract from the clause rows naming it. A requirement's
//! set-/edge-scope demands ride ordinary [`contract::Clause`] values;
//! their predicate payloads ([`contract::EdgeBound`] and
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
/// the root member, never a stance temper bakes in.
/// Defaults to [`Warn`](EnforcementMode::Warn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnforcementMode {
    /// Allows the call and records the finding out-of-band only — the next report,
    /// never the live session. The newly expressible tier; unreachable until an
    /// author declares it.
    Note,
    /// Allows the call and surfaces the finding in-band, into the live context. The
    /// default: enforcement mode is author-declared per placement, never assumed.
    #[default]
    Warn,
    /// Denies the call.
    Block,
}

/// A declared **edge relationship** — a kind capability declared on the owning kind's
/// members. The owning kind is the edge *source*
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
    /// The non-empty set of artifact kinds the reference may resolve into — the edge
    /// *targets*. A one-element set resolves a bare address within its one kind; a
    /// multi-element set resolves only the kind-qualified `kind:name` address, whose
    /// kind names which member of the set. Every declared kind must be one `temper`
    /// models, else that element's routes can never resolve (a graph-admissibility
    /// concern, [`crate::graph`]).
    pub to: Vec<String>,
}

/// A named **requirement** — the harness's named obligation, declared in the
/// assembly's `[requirement.<name>]`. **Every facet is optional
/// except the name.** Fill is by the artifact's opt-in `satisfies` alone — there is
/// no name-`match` selector.
///
/// `temper` **carries `prose` verbatim, never interprets it** — it is authored
/// intent the surface carries, never a thing the engine judges. The decidable
/// shadow is what `check` gates: [`crate::coverage`] over the `satisfies` edges,
/// [`crate::roster`]/[`crate::graph`] over the **satisfier set** (the artifacts of
/// its `kind` that opt in via `satisfies`).
#[derive(Debug, Clone, PartialEq)]
pub struct Requirement {
    /// The requirement's name.
    pub name: String,
    /// The authored *intent*, stated in meaning not predicates. Carried verbatim and
    /// **never interpreted**.
    pub prose: Option<String>,
    /// The requirement's declared satisfier kind. Unlike the old name-`match`
    /// selector, this never narrows *which* opt-in artifacts are candidates —
    /// [`crate::roster`]/[`crate::graph`] draw the satisfier set kind-blind from
    /// every modeled kind, the opt-in `satisfies` join the sole filter.
    /// Present, it instead
    /// *sources* the shipped each-grain "every satisfier is kind K" clause
    /// [`crate::engine::judge`] evaluates over that kind-blind set — a satisfier of
    /// a different kind is a finding, never a silent exclusion. Absent ⇒
    /// **kind-blind**: any artifact that opts in fills it, and no narrowing clause
    /// attaches at all.
    pub kind: Option<String>,
    /// Whether an unfilled requirement is a gate-blocking violation. Absent ⇒ `false`
    /// (`temper` never fabricates a gate the author did not declare
    /// "Declared, never mined"). Never cardinality — posture and the set-scope `count` clause in
    /// [`clauses`](Requirement::clauses) are different kinds of thing.
    pub required: bool,
    /// The requirement's set-/edge-scope demands — ordinary [`contract::Clause`]
    /// values whose predicates range over the satisfier set and its graph
    /// neighborhood. Each carries its own severity/guidance/cite; empty ⇒ no set-scope
    /// demand at all. `count`/`unique`/`membership` are checked in
    /// [`crate::roster`]; `degree` ranges over the *edge* graph, so it is checked in
    /// [`crate::graph`] instead.
    pub clauses: Vec<contract::Clause>,
    /// An optional external verifier for the behavioral remainder (`verified_by`).
    /// Stored verbatim; whether it *resolves* is an admissibility check.
    pub verified_by: Option<String>,
}

/// A kind's whole default [`Contract`], built directly from the clause rows naming it
/// in the committed lock — the one lift both a custom kind (which carries no embedded
/// default: its committed rows **are** its contract) and a built-in kind whose lock
/// declares rows run through. A built-in kind the lock names no row for falls back to
/// its embedded default instead ([`crate::builtin::contract`]); either way rows-or-
/// default, never a severity-flip layer over the embedded default. This is the same
/// lift [`crate::builtin::contract`] runs over the *embedded* lock's own rows, run here
/// over the committed lock's.
///
/// # Errors
///
/// A row naming a predicate outside the closed vocabulary, or missing an argument its
/// predicate requires, is a [`ClauseRowError`] — the lock is tool-written, never
/// hand-patched (`specs/model/pipeline.md`), so a row the closed vocabulary cannot
/// admit is a corrupt lock rejected loud, never a clause silently dropped.
pub fn default_contract_from_rows(
    clauses: &[ClauseRow],
    kind: &str,
) -> Result<Contract, ClauseRowError> {
    let clauses = clauses
        .iter()
        .filter(|row| row.kind.as_deref() == Some(kind))
        .map(clause_from_row)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Contract {
        name: kind.to_string(),
        clauses,
        guidance: None,
    })
}

/// A contract with the clause rows of the invocation's joined locks naming `kind`
/// appended to it — the host's own contract, hardened.
///
/// The joined rows are lifted by the same [`clause_from_row`] the host's are: whatever
/// lock carried a row, it composes into the host corpus's selection for the kind it
/// names, and nothing downstream can tell the two apart. Appending is the whole
/// operation, and it is what bounds a layer to hardening: a joined row never replaces,
/// reorders, or deletes a host clause, so the host's reviewed contract still judges
/// every member it judged before. A joined row that would weaken a host clause simply
/// reports beside the one it cannot displace — visible, and inert on the verdict.
///
/// # Errors
///
/// As [`default_contract_from_rows`]: a joined row the closed vocabulary cannot admit
/// is a corrupt lock, refused rather than dropped.
pub fn with_joined_clauses(
    mut contract: Contract,
    joined: &[ClauseRow],
    kind: &str,
) -> Result<Contract, ClauseRowError> {
    for row in joined
        .iter()
        .filter(|row| row.kind.as_deref() == Some(kind))
    {
        contract.clauses.push(clause_from_row(row)?);
    }
    Ok(contract)
}

/// A lock clause row the closed predicate vocabulary cannot admit — an unknown
/// predicate or one missing its required argument, or an out-of-vocabulary severity.
/// Surfaced as a load error rather than a silent skip: the lock is tool-written, so a
/// row the SDK could not have emitted is corruption, not a tolerable hand-edit
/// (`specs/model/contract.md`, "clause": an unknown predicate is rejected at load).
#[derive(Debug, Clone, thiserror::Error, miette::Diagnostic)]
pub enum ClauseRowError {
    /// The row names a predicate outside the closed vocabulary, or omits an argument
    /// its predicate requires — either way no clause can be built.
    #[error(
        "lock clause row names predicate `{predicate}`, which is not in the closed \
         vocabulary or is missing a required argument"
    )]
    Predicate {
        /// The offending row's predicate key.
        predicate: String,
    },
    /// The row's severity label is outside the closed `required`/`advisory` vocabulary.
    #[error(
        "lock clause row for predicate `{predicate}` declares severity `{severity}`, \
         outside the closed `required`/`advisory` vocabulary"
    )]
    Severity {
        /// The offending row's predicate key.
        predicate: String,
        /// The unrecognized severity label.
        severity: String,
    },
    /// The row carries no address. Emit stamps one onto every row it writes, so a row
    /// without one never came from an emit.
    #[error(
        "lock clause row for predicate `{predicate}` carries no `label` — every emitted \
         clause row is stamped with its address, so a row without one is not a row emit wrote"
    )]
    Label {
        /// The offending row's predicate key.
        predicate: String,
    },
}

/// Lift one clause row into its typed [`contract::Clause`] — its address, predicate,
/// severity, guidance, and cite.
/// `pub` (not `pub(crate)`, same reasoning as [`severity_from_label`]): the `main`
/// binary is a separate crate from this library, so its requirement-nested lift
/// needs this visible across the crate boundary to wrap it, as `crate::builtin`'s
/// embedded-lock lift also does.
///
/// # Errors
///
/// A row naming a predicate outside the closed vocabulary, missing a required
/// argument, carrying no address, or declaring an out-of-vocabulary severity is a
/// [`ClauseRowError`] — rejected loud, never a silently dropped clause (see
/// [`default_contract_from_rows`]).
pub fn clause_from_row(row: &ClauseRow) -> Result<contract::Clause, ClauseRowError> {
    let severity = severity_from_label(&row.severity).ok_or_else(|| ClauseRowError::Severity {
        predicate: row.predicate.clone(),
        severity: row.severity.clone(),
    })?;
    let predicate = contract::predicate_from_row(row).ok_or_else(|| ClauseRowError::Predicate {
        predicate: row.predicate.clone(),
    })?;
    // Lifted verbatim, never re-derived: the label the lock committed is the label
    // every finding prints, so the two cannot drift apart on a grammar change.
    let label = row.label.clone().ok_or_else(|| ClauseRowError::Label {
        predicate: row.predicate.clone(),
    })?;
    Ok(contract::Clause {
        label,
        severity,
        predicate,
        guidance: row.guidance.clone(),
        source: row.cite.clone(),
    })
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

    /// A [`ClauseRow`] at `severity`, every other column defaulted — the base the
    /// reject-loud cases struct-update, overriding only `kind`/`predicate` and any
    /// argument column the case exercises.
    fn clause_row(severity: &str) -> ClauseRow {
        ClauseRow {
            label: Some("fixture.clause".to_string()),
            kind: None,
            predicate: String::new(),
            field: None,
            severity: severity.to_string(),
            guidance: None,
            cite: None,
            count: None,
            target: None,
            degree: None,
            gate: None,
            value_type: None,
            bound: None,
            charset: None,
            keys: None,
            values: None,
            range: None,
            section: None,
        }
    }

    #[test]
    fn default_contract_from_rows_builds_a_custom_kinds_whole_default_contract() {
        // A custom kind has no built-in default to override — its committed rows are its
        // whole default contract, so a matching row contributes a brand new clause rather
        // than only flipping an existing one's severity.
        let rows = vec![
            ClauseRow {
                label: Some("spec.max_lines".to_string()),
                kind: Some("spec".to_string()),
                predicate: "max_lines".to_string(),
                field: None,
                severity: "advisory".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                gate: None,
                value_type: None,
                bound: Some(crate::drift::BoundRow {
                    min: None,
                    max: Some(150),
                }),
                charset: None,
                keys: None,
                values: None,
                range: None,
                section: None,
            },
            ClauseRow {
                label: Some("rule.max_lines".to_string()),
                kind: Some("rule".to_string()),
                predicate: "max_lines".to_string(),
                field: None,
                severity: "required".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                gate: None,
                value_type: None,
                bound: Some(crate::drift::BoundRow {
                    min: None,
                    max: Some(10),
                }),
                charset: None,
                keys: None,
                values: None,
                range: None,
                section: None,
            },
        ];

        let contract = default_contract_from_rows(&rows, "spec").unwrap();
        assert_eq!(contract.name, "spec");
        assert_eq!(
            contract.clauses,
            vec![Clause {
                label: "spec.max_lines".to_string(),
                severity: Severity::Advisory,
                predicate: Predicate::MaxLines { max: 150 },
                guidance: None,
                source: None,
            }]
        );
    }

    #[test]
    fn default_contract_from_rows_rejects_a_row_it_cannot_lift() {
        // The lock is tool-written, never hand-patched: a row the closed vocabulary
        // cannot admit is corruption rejected loud, never a clause silently dropped.
        // An unknown predicate names nothing in the vocabulary.
        let unknown = vec![ClauseRow {
            label: None,
            kind: Some("spec".to_string()),
            predicate: "not_a_predicate".to_string(),
            ..clause_row("advisory")
        }];
        assert!(matches!(
            default_contract_from_rows(&unknown, "spec"),
            Err(ClauseRowError::Predicate { predicate }) if predicate == "not_a_predicate"
        ));

        // A known predicate missing its required argument (`section_contains` with no
        // `section` column) cannot be built either — the same loud rejection.
        let missing_arg = vec![ClauseRow {
            label: None,
            kind: Some("spec".to_string()),
            predicate: "section_contains".to_string(),
            ..clause_row("advisory")
        }];
        assert!(matches!(
            default_contract_from_rows(&missing_arg, "spec"),
            Err(ClauseRowError::Predicate { .. })
        ));

        // A severity outside the closed `required`/`advisory` vocabulary is rejected
        // on the severity channel.
        let bad_severity = vec![ClauseRow {
            label: None,
            kind: Some("spec".to_string()),
            predicate: "max_lines".to_string(),
            bound: Some(crate::drift::BoundRow {
                min: None,
                max: Some(150),
            }),
            ..clause_row("blocking")
        }];
        assert!(matches!(
            default_contract_from_rows(&bad_severity, "spec"),
            Err(ClauseRowError::Severity { severity, .. }) if severity == "blocking"
        ));
    }
}
