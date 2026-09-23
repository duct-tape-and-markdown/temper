## Surface

Two copies of "find the root selection's clause for predicate P", added a
tick apart because the two judges live in different modules:

- `src/engine.rs` — `pub fn root_clause(selections, predicate)`, this tick's,
  read by `src/gate.rs` for the `fresh` clause.
- `src/graph.rs:326` — `fn root_reachable_clause(selections)`, the same walk
  with `Predicate::Reachable` inlined, read by `graph::reachable`.

Both find `Selector::Root` and then the matching clause, and both document
the same "yield the clause, not a bool, so severity + label + guidance all
reach the finding" rationale.

## Observed at

32de1b65 (HEAD when observed) — `src/graph.rs` sat outside
FRESH-CLAUSE-JUDGES-STALENESS's fence, so the unification could not ride
that entry.

## Suggested consolidation

Keep `engine::root_clause` — `engine` owns `Selection`/`Selector` — and make
`graph::root_reachable_clause` a one-line call into it, or delete it and
inline `engine::root_clause(selections, &Predicate::Reachable)` at
`graph::reachable`'s opt-in guard.
