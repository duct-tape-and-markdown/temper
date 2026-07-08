# Plan state

- Spec derived through: f87cc0c
- Audited through: 9d31a8b
- Residue swept through: a88cb9e
- This tick: Ship audit 95689ae..HEAD (9d31a8b). Three commits in range
  (3a2bed1, a88cb9e, 9d31a8b) — all three are plan's own prior commits,
  touching only `.flume/plan/*`; none touch src/tests/sdk. Spot-verified on
  disk (not just git log): tests/cli.rs:65 still carries its own local
  `write_harness`, confirming TEST-FIXTURE-HELPERS-CONSOLIDATE genuinely
  hasn't shipped, so its blockedBy chain (recordwriters, lockunitbuilders)
  correctly still holds. No pending entry dropped; cursor advances to HEAD.
- Queue: TMPDIR-HELPER-CONSOLIDATE(mainrs), TMPDIR-HELPER-CONSOLIDATE
  (kindrs), FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE, TEST-FIXTURE-HELPERS-
  CONSOLIDATE all open and pairwise disjoint, pickable in parallel —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers, still unshipped) — TEST-HELPER-DUPES-CONSOLIDATE
  (lockunitbuilders) (blockedBy recordwriters) — PACKAGING-CHANNELS
  (parked, condition unchanged).

Plan continues: yes — residue sweep still trails HEAD by one unswept commit
(9d31a8b, itself `.flume/plan/*`-only) — next tick's quiet-or-sweep pass.
