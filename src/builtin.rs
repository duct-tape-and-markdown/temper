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
use crate::contract::{Charset, Clause, Contract, Predicate, Severity};
use crate::drift::{CharsetRow, ClauseRow};

/// Lift one embedded clause row's `charset` column into the typed [`Charset`] —
/// `None` when a range spec is not the `<lo>-<hi>` shape `emit` always writes.
fn charset_from_row(row: &CharsetRow) -> Option<Charset> {
    let mut ranges = Vec::with_capacity(row.ranges.len());
    for spec in &row.ranges {
        match spec.chars().collect::<Vec<char>>().as_slice() {
            [lo, '-', hi] => ranges.push((*lo, *hi)),
            _ => return None,
        }
    }
    let chars = row.chars.as_deref().unwrap_or_default().chars().collect();
    Some(Charset { ranges, chars })
}

/// Lift one clause row's predicate — the full argument payload
/// (`bound`/`charset`/`keys`/`values`) alongside `field` — into the typed
/// [`Predicate`]. `None` for a predicate key or argument shape this projection
/// carries no column for. `pub(crate)` so [`crate::compose::default_contract_from_rows`] reuses
/// the identical lift over a custom kind's own committed-lock rows, never a second
/// copy of the predicate vocabulary.
pub(crate) fn predicate_from_row(row: &ClauseRow) -> Option<Predicate> {
    Some(match row.predicate.as_str() {
        "required" => Predicate::Required {
            field: row.field.clone()?,
        },
        "optional" => Predicate::Optional {
            field: row.field.clone()?,
        },
        "min_len" => Predicate::MinLen {
            field: row.field.clone()?,
            min: row.bound?.min?,
        },
        "max_len" => Predicate::MaxLen {
            field: row.field.clone()?,
            max: row.bound?.max?,
        },
        "max_lines" => Predicate::MaxLines {
            max: row.bound?.max?,
        },
        "allowed_chars" => Predicate::AllowedChars {
            field: row.field.clone()?,
            charset: charset_from_row(row.charset.as_ref()?)?,
        },
        "forbidden_keys" => Predicate::ForbiddenKeys {
            keys: row.keys.clone()?,
        },
        "deny" => Predicate::Deny {
            field: row.field.clone()?,
            values: row.values.clone()?,
        },
        "name-matches-dir" => Predicate::NameMatchesDir,
        "unique-name" => Predicate::UniqueName,
        _ => return None,
    })
}

/// Lift one embedded clause row into its typed [`Clause`] — predicate, severity,
/// guidance, and cite, the clause's full four channels.
/// The embedded lock is this crate's own emit, never hand-edited
/// (`crate::builtin_lock`), so a row this projection cannot lift is a build-time
/// bug, not a runtime condition — the same invariant `builtin_lock::declarations`
/// leans on for the embedded bytes themselves.
fn clause_from_row(row: &ClauseRow) -> Clause {
    Clause {
        severity: compose::severity_from_label(&row.severity)
            .expect("the embedded built-in lock declares only required/advisory severities"),
        predicate: predicate_from_row(row).expect(
            "the embedded built-in lock's rows encode only this projection's supported \
             predicates, each carrying its required argument",
        ),
        guidance: row.guidance.clone(),
        source: row.cite.clone(),
    }
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

/// The shipped each-grain clause a typed requirement's `kind` facet sources —
/// "every satisfier is kind K" at `required` severity. The mechanism — the predicate shape and its `required`
/// severity — ships fixed with the requirement facet; only `kind` is
/// per-requirement author data, so [`crate::roster::check`] calls this to
/// synthesize the clause fresh from [`compose::Requirement::kind`] every run
/// rather than storing it on the requirement.
#[must_use]
pub fn kind_narrowing_clause(kind: &str) -> Clause {
    Clause {
        severity: Severity::Required,
        predicate: Predicate::Kind {
            kind: kind.to_string(),
        },
        guidance: None,
        source: None,
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
