## Surface

`ROOT-DEFAULT-CONTRACT-SHIPS` cannot reach green inside its fence. Shipping
`rootDefaultContract` puts one kind-less clause row (`root.reachable`, advisory)
into `src/builtin_lock.toml`, and two existing tests assert the negation of
exactly that — both outside the entry's `files[]`:

- `src/builtin_lock.rs:41` —
  `the_embedded_lock_parses_into_kind_facts_and_floor_clauses_only` walks every
  embedded clause row and asserts `matches!(clause.kind.as_deref(), Some("agent"
  | … | "memory"))` (:94-:112). The root's row carries `kind = None` by
  construction — that absence *is* the root declaration — so the assertion fails
  on the regenerated lock. Fix: admit the kind-less row, pinned as the root's
  (exactly one, label `root.reachable`) rather than widened to "any `None`".

- `tests/check_cost.rs:924` —
  `gate_reachability_closure_runs_once_per_invocation_and_only_when_a_root_clause_binds`.
  Its first half writes `Declarations::default()` as the lock and asserts the
  closure walks zero times. With the default shipped, `compose::root_contract`'s
  rows-or-default fallback composes `builtin::root_contract()`, which now binds
  `reachable`, so the closure walks once. The claim survives, restated: what opts
  in is the *clause*, not the root selection's existence. Fix: seed a root row of
  some other predicate (a `count` bound) instead of an empty `Declarations` —
  the same edit `tests/root_contract.rs`'s sibling case takes, in fence.

Both are one-hunk test edits; no `src/` behavior changes with them.

The rest of the entry reached green locally and is otherwise fence-clean —
`pnpm --dir sdk test` (193), `cargo clippy --all-targets -D warnings`,
`cargo doc`, and every other `cargo test --no-fail-fast` target. Two entry
`files[]` rows turned out to need no edit: `tests/emit.rs` (regenerating
`examples/base-harness/.temper/lock.toml` was the whole of it) and
`tests/acceptance.rs` + `tests/snapshots/acceptance__rules_check_diagnostics.snap`
(no fixture member under `tests/fixtures/rules/` has a dead registration, so the
root clause adds no finding there). The gauntlet did gain one —
`tests/snapshots/gauntlet__projection_tree.snap` moves too, beyond the
`gauntlet__check_diagnostics.snap` the entry names; `tests/snapshots/**` already
covers it.

One consequence worth a look when the entry is re-scoped, not a blocker: with
the default shipped, an authored `contract: []` lowers zero kind-less rows, which
`compose::root_contract` reads as "no rows" and answers with the embedded
default — so "this harness declares no root contract" is unspellable. That is
rows-or-default behaving as it does per kind, so it may be intended; flagging it
because the root is the one member for which the empty contract is a plausible
authored intent.

## Observed at

6921adbe (HEAD when observed).

## Suggested consolidation

Re-scope `ROOT-DEFAULT-CONTRACT-SHIPS` with `src/builtin_lock.rs` and
`tests/check_cost.rs` added to `files.edit`. No structural consolidation is
owed — this is fence coverage, not debt.
