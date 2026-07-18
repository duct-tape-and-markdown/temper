//! The embedded built-in floor projection.
//!
//! Each built-in kind's floor [`Contract`] (`agent`, `command`, `skill`, `rule`,
//! `memory`) is a lossless
//! projection of the embedded built-in lock's clause rows
//! (`crate::builtin_lock::declarations`), grouped by kind label — never a
//! hand-written mirror. The lock
//! itself is `@dtmd/temper/claude-code`'s own emit; this module only lifts its
//! `ClauseRow`s back into the typed [`Contract`] algebra the gate already runs on.

use std::collections::BTreeMap;

use crate::builtin_lock;
use crate::compose;
use crate::contract::{Clause, Contract};
use crate::drift::ClauseRow;

/// Lift one embedded clause row into its typed [`Clause`] — predicate, severity,
/// guidance, and cite, the clause's full four channels, via the shared
/// [`compose::clause_from_row`] lift.
/// The embedded lock is this crate's own emit, never hand-edited
/// (`crate::builtin_lock`), so a row the shared lift cannot lift is a build-time
/// bug, not a runtime condition — the same invariant `builtin_lock::declarations`
/// leans on for the embedded bytes themselves.
fn clause_from_row(row: &ClauseRow) -> Clause {
    compose::clause_from_row(row).expect(
        "the embedded built-in lock declares only required/advisory severities and \
         this projection's supported predicates, each carrying its required argument",
    )
}

/// The floor [`Contract`] for `kind` — every embedded clause row naming it, in
/// declaration order, projected into typed clauses. A floor is an exported clause
/// array: the constructed contract's own
/// `guidance` stays `None` — every clause's guidance already rides its row.
fn contract_for_kind(kind: &str) -> Contract {
    let clauses = builtin_lock::declarations()
        .clauses
        .iter()
        .filter(|row| row.kind.as_deref() == Some(kind))
        .map(clause_from_row)
        .collect();
    Contract {
        name: kind.to_string(),
        clauses,
        guidance: None,
    }
}

/// The embedded built-in floor bound to a kind's bare row label, or `None` if no
/// embedded kind of that name ships one.
#[must_use]
pub fn contract(kind: &str) -> Option<Contract> {
    builtin_lock::declarations()
        .kinds
        .iter()
        .any(|row| row.name == kind)
        .then(|| contract_for_kind(kind))
}

/// Every embedded built-in kind's floor, keyed by its bare row label — the
/// compiled default program's floor roster.
#[must_use]
pub fn contracts() -> BTreeMap<String, Contract> {
    builtin_lock::declarations()
        .kinds
        .iter()
        .map(|row| (row.name.clone(), contract_for_kind(&row.name)))
        .collect()
}
