# Plan state

- Spec derived through: a53eee4
- Audited through: a4bb450
- Residue swept through: a4bb450
- This tick: Residue sweep (job 4), e6d0311..a4bb450. The only src/tests/sdk
  change in range is e082d04 (NESTED-MEMBER-LOCK-ROW): src/drift.rs,
  sdk/src/declarations.ts, sdk/src/emit.ts, sdk/test/emit.test.ts,
  tests/lock_declaration_rows.rs — purely additive per its own commit body,
  confirmed on disk (the new `NestedMemberRow` to/from-table pair follows
  the same per-shape pattern already used by `CountBoundRow` et al., no
  duplicate generic map helper introduced); already fully accounted for by
  the open RETIRE-FOLD-MEMBERS entry, nothing new to file. Re-verified all
  three standing accepted debts in open-questions.md — `kinds/`/`packages/`
  retirement's two fixtures (tests/session_start.rs `+++` fixtures;
  sdk/src/builtins.ts:308,348,385 PACKAGE.md citations) and the
  `overlay_builtin_kind` stale comment (tests/coverage.rs:336-338) — all
  three untouched by e082d04, restamped to a4bb450.
- Queue: RETIRE-FOLD-MEMBERS (open, pickable) → PACKAGING-CHANNELS (parked,
  unchanged). Disjoint: the two share no path.

Plan continues: no — inbox empty, spec delta empty, ship audit and residue
sweep both caught up to HEAD a4bb450; handing off to build.
