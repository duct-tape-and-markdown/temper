# Plan state

- Spec derived through: a53eee4
- Audited through: f000b97
- Residue swept through: a4bb450
- This tick: Ship audit (job 3), a4bb450..f000b97 — two commits,
  5ee0b6d (build: RETIRE-FOLD-MEMBERS) and f000b97 (chore: ship, drops the
  entry). Verified on disk: `builtin_kind::features` gained a
  `&declarations.nested_members` param, threaded through all four call
  sites in src/main.rs; `CustomKind::fold_members` /
  `parse_embedded_info` / `parse_embedded_member` retired with no
  fallback (rg confirms zero live callers, one doc-comment mention of
  the retired name in tests/nested_member.rs); `Primitive::Fenced` kept
  as an ordinary raw-block primitive exactly as the commit body says
  (still primed in src/kind.rs, still exercised by
  tests/extract_equivalence.rs). pending.json's RETIRE-FOLD-MEMBERS entry
  was already removed by f000b97 — nothing left to drop. Re-tested
  PACKAGING-CHANNELS' parked reason: none of its cited blockers touched
  by this range, still parked, unchanged. Re-tested the three
  open-questions "Kept on purpose" debts (session_start.rs `+++`
  fixtures, sdk/src/builtins.ts citations, tests/coverage.rs stale
  comment): none of their three files touched by 5ee0b6d, all three
  stand unchanged.
- Queue: PACKAGING-CHANNELS (parked, unchanged) — the only pending
  entry.

Plan continues: yes — residue sweep (job 4): Residue swept through
(a4bb450) trails HEAD (f000b97).
