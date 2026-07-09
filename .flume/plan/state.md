# Plan state

- Spec derived through: f87cc0c
- Audited through: 69ba0fe
- Residue swept through: e45d424
- This tick: Inbox (job 1) — routed T18 (`.flume/inbox.md`, observed 3ff0dd1,
  unblocked). Re-verified the mechanism on disk (no code changed since
  3ff0dd1): `from_kind_fact_row` lifts a lock row's `templates` correctly
  (kind.rs:225-247, its own passing test); `gate`'s built-in loop and
  `builtin_features_by_kind` both build every built-in kind straight off
  `builtin_kind::definitions()` (main.rs:577-593, 890-903), never
  `from_kind_fact_row`, and the only overlay applied — `effective_governs`
  (main.rs:769) — carries `governs` alone; `row_relocates_builtin`'s own doc
  comment (main.rs:993-995) already names a declared `templates` a
  legitimate relocation extension, but nothing consumes it before
  extraction. Filed as BUILTIN-KIND-TEMPLATES-OVERLAY (open, per
  `specs/model/representation.md` "kind" — ownership-not-privilege: a
  built-in's templates overlay is the same mechanism a custom kind already
  gets via `from_kind_fact_row`). Drained the inbox line. No fork needed —
  the gap and fix shape are both concrete. open-questions.md untouched (no
  new fork, no code-touching entry hit an accepted-debt file this tick).
- Queue: BUILTIN-KIND-TEMPLATES-OVERLAY (open, pickable — src/main.rs +
  tests/lock_declaration_rows.rs), PACKAGING-CHANNELS (parked, package.json +
  a new release.yml). Disjoint — no shared file.

Plan continues: no — inbox is now drained and every other input is still
current from last tick (spec delta empty, ship-audit and residue-sweep
cursors both trail no new src/tests/sdk commits). Hand off — build can pick
BUILTIN-KIND-TEMPLATES-OVERLAY.
