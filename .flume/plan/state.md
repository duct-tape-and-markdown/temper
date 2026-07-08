# Plan state

- Spec derived through: f87cc0c
- Audited through: 95689ae
- Residue swept through: 38c01c5
- This tick: Ship audit. Verified on disk that both commits past the prior
  `Audited through` (220e9cd) shipped as claimed: 1894af9 consolidated the
  tmpdir/fixture test scaffolding into `tests/common` (TEST-SCAFFOLDING-
  CONSOLIDATE) and 7329585 switched reporter.rs's plural suffix to
  `display::plural` (PLURAL-HELPER-CONSOLIDATE(reporter), no residue —
  grepped for the old ternary, none left). Neither has a stale pending entry
  (both were already absent from pending.json). Re-verified the two entries
  gated `blockedBy TEST-SCAFFOLDING-CONSOLIDATE`: the blocker shipped, so
  both unblock to `open`. FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE's citations
  (src/frontmatter.rs, src/install.rs) were untouched by the tmpdir ship and
  needed no correction. TEST-FIXTURE-HELPERS-CONSOLIDATE's citations span
  13 of the exact test files the tmpdir ship rewrote — every cited line
  shifted (-15 in most files, -17 in tests/install.rs and tests/emit.rs);
  re-derived each via `rg` against current HEAD and rewrote the entry whole.
  Confirmed all three now-open entries (TMPDIR-HELPER-CONSOLIDATE(mainrs),
  FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE, TEST-FIXTURE-HELPERS-CONSOLIDATE)
  are pairwise disjoint on `files.edit[].path`.
- Queue: TMPDIR-HELPER-CONSOLIDATE(mainrs), FRONTMATTER-DELIMITER-SCAN-
  CONSOLIDATE, TEST-FIXTURE-HELPERS-CONSOLIDATE all open and pickable in
  parallel — TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers, still unshipped) — TEST-HELPER-DUPES-CONSOLIDATE
  (lockunitbuilders) (blockedBy recordwriters) — PACKAGING-CHANNELS (parked,
  condition unchanged).

Plan continues: yes — residue sweep is live next: `Residue swept through:
38c01c5` trails HEAD by five commits (425968d..95689ae), none yet swept for
corpus-vs-code residue beyond what this tick and its predecessors already
filed.
