## Surface

`HOOK-CONTRACT-CLAUSES-THE-HANDLER-SCHEMA` cannot ship inside its fence: shipping
the `hook` default contract's five `when` guards turns
`tests/lock_declaration_rows.rs:2785` red, and that file is not in the entry's
`files[]`.

The assertion is
`the_embedded_lock_clauses_match_todays_hand_written_floors_per_kind`'s hook arm,
comparing `lock_quads("hook")` against `floor_quads("hook")`:

```
left:  [..., ("when", Some("type"), None, "required"), x5]   # lock_quads
right: [..., ("when", None,         None, "required"), x5]   # floor_quads
```

Not a round-trip loss — a projection gap in the test's own helper. A `when`
row's `field` column is its **guard's** field (`src/contract.rs`
`when_label_field`, and `src/engine.rs:1571` reads `guard.target()` to decide
one), but `Predicate::When` exposes no `target()` of its own, so
`floor_quads` (`tests/lock_declaration_rows.rs:2429`) reduces every guard to
`None`. The helper already carries one such exception arm — `MentionReachable`,
the two-field predicate `target` cannot name — and `when` is the second shape
needing it. The gap was invisible until now because the arm's only exercised
kinds are skill/rule/memory/command/agent/hook, and `mcp-server` and
`marketplace` — the two floors already carrying `when` clauses — are not among
them.

## Observed at

e43d1436 (HEAD when observed). Every other target of `cargo test
--no-fail-fast` is green with the entry's work applied, including the
regenerated `src/builtin_lock.toml`, the contract matrix, and both gauntlet
snapshots; `pnpm --dir sdk test` is green too.

## Suggested consolidation

Add `tests/lock_declaration_rows.rs` to the entry's `files[]`, and give
`floor_quads` a `Predicate::When { guard, .. } => (guard.target().map(…), None)`
arm beside the `MentionReachable` one — the same read `lock_quads` takes off the
column and `engine.rs` takes off the variant. The hook arm's comment at
`:2781` ("a single `enum` clause over the lifecycle event") goes stale with it.
Worth extending the same test to `mcp-server` and `marketplace` while there, so
the guard projection is pinned by the kinds that have always had one.
