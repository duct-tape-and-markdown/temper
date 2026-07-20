//! The embedded built-in floor projection.
//!
//! Every embedded kind's floor [`Contract`] is a lossless projection of the
//! embedded built-in lock's clause rows (`crate::builtin_lock::declarations`),
//! grouped by kind label — never a hand-written mirror. The lock itself is
//! `@dtmd/temper/claude-code`'s own emit; this module only lifts its `ClauseRow`s
//! back into the typed [`Contract`] algebra the gate already runs on.

use std::collections::BTreeMap;

use crate::builtin_lock;
use crate::compose;
use crate::contract::Contract;

/// The floor [`Contract`] for `kind` — every embedded clause row naming it, in
/// declaration order, projected into typed clauses. A floor is an exported clause
/// array: the constructed contract's own
/// `guidance` stays `None` — every clause's guidance already rides its row.
fn contract_for_kind(kind: &str) -> Contract {
    compose::default_contract_from_rows(&builtin_lock::declarations().clauses, kind).expect(
        "the embedded built-in lock declares only required/advisory severities and \
             this projection's supported predicates, each carrying its required argument",
    )
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
