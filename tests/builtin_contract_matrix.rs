//! Insta snapshot of the built-in kind × clause matrix.
//!
//! Every built-in kind row against every clause key its default contract carries,
//! cell = severity (error/advisory/blank), so an asymmetry like one rule having
//! `mention_reachable` and its paired skill lacking it shows as a reviewed snapshot
//! diff at ship time instead of a silent gap.

use std::collections::{BTreeMap, BTreeSet};

use temper::builtin;
use temper::builtin_lock;
use temper::contract::Severity;

#[test]
fn builtin_contract_matrix() {
    let declarations = builtin_lock::declarations();
    let contracts = builtin::contracts();

    // Collect all kinds, sorted by name.
    let mut kinds: Vec<&str> = declarations
        .kinds
        .iter()
        .map(|row| row.name.as_str())
        .collect();
    kinds.sort_unstable();

    // Collect all unique clause predicates across all contracts, sorted by name.
    let mut all_predicates = BTreeSet::new();
    for contract in contracts.values() {
        for clause in &contract.clauses {
            all_predicates.insert(clause.predicate.key());
        }
    }
    let predicates: Vec<&str> = all_predicates.into_iter().collect();

    // Build the matrix: kind → predicate → severity.
    // For each kind, collect its clauses keyed by predicate key.
    let mut matrix: BTreeMap<&str, BTreeMap<&str, &str>> = BTreeMap::new();
    for kind in &kinds {
        let mut row = BTreeMap::new();
        if let Some(contract) = contracts.get(*kind) {
            for clause in &contract.clauses {
                let severity_str = match clause.severity {
                    Severity::Required => "error",
                    Severity::Advisory => "advisory",
                };
                row.insert(clause.predicate.key(), severity_str);
            }
        }
        matrix.insert(kind, row);
    }

    // Render as a formatted table.
    let mut output = String::new();

    // Header row: kind | predicate | predicate | ...
    output.push_str("kind");
    for predicate in &predicates {
        output.push_str(" | ");
        output.push_str(predicate);
    }
    output.push('\n');

    // Separator row
    output.push_str("---");
    for _ in &predicates {
        output.push_str(" | ---");
    }
    output.push('\n');

    // Data rows
    for kind in &kinds {
        output.push_str(kind);
        if let Some(row) = matrix.get(kind) {
            for predicate in &predicates {
                output.push_str(" | ");
                if let Some(severity) = row.get(predicate) {
                    output.push_str(severity);
                }
            }
        }
        output.push('\n');
    }

    insta::assert_snapshot!("builtin_contract_matrix", output);
}
