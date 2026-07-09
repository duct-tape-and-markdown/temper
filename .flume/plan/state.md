# Plan state

- Spec derived through: a53eee4
- Audited through: a4bb450
- Residue swept through: e6d0311
- This tick: Ship audit c93eeed..a4bb450 (job 3). Verified on disk that
  NESTED-MEMBER-LOCK-ROW shipped as described: `src/drift.rs` carries the
  new `NestedMemberRow` struct plus `Declarations::nested_members`
  (`[[declaration.nested_member]]`), wired through
  `Declarations::write_into`/`declarations_from_doc`; `sdk/src/declarations.ts`
  emits the family via `nestedMemberRows(harness)`;
  `tests/lock_declaration_rows.rs` gained a passing round-trip test for it.
  Confirmed the commit's own claim — purely additive, `CustomKind::fold_members`
  untouched, fence rendering untouched. pending.json already had no
  NESTED-MEMBER-LOCK-ROW entry (build's ship commit a4bb450 removed it).
  RETIRE-FOLD-MEMBERS's `blockedBy NESTED-MEMBER-LOCK-ROW` condition is now
  met: re-verified every cited file/symbol still resolves unchanged
  (src/kind.rs `fold_members`/`overlay_templates`, src/builtin_kind.rs
  `features()`, src/extract.rs `parse_embedded_info`/`parse_embedded_member`,
  src/main.rs's four callers, and both named tests in
  tests/lock_declaration_rows.rs) and flipped its gate to open, sharpening
  the file descriptions to cite the concrete `NestedMemberRow`/
  `Declarations::nested_members` source now that it exists. PACKAGING-CHANNELS
  re-verified unchanged (no `.github/workflows/` or `package.json` commits
  since c93eeed) — reason re-stamped to a4bb450.
- Queue: RETIRE-FOLD-MEMBERS (open, pickable) → PACKAGING-CHANNELS (parked,
  unchanged). Disjoint: the two share no path.

Plan continues: yes — jobs 1-3 are now quiet, but `Residue swept through`
(e6d0311) still trails HEAD (a4bb450), and this tick's own audited commits
(e082d04, a4bb450) touched src/tests/sdk past that cursor, so job 4 (residue
sweep) is live next tick.
