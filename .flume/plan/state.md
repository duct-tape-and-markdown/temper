# Plan state

- Spec derived through: f87cc0c
- Audited through: 220e9cd
- Residue swept through: 425968d
- This tick: Ship audit 425968d..220e9cd. Verified PLURAL-HELPER-CONSOLIDATE
  shipped clean (4baac1b: src/display.rs owns `plural(n)`, bundle.rs/
  coverage_note.rs/install.rs all call the shared helper, no duplicate
  remains; 220e9cd removed it from pending). Unblocked
  TEST-SCAFFOLDING-CONSOLIDATE (gate blockedBy->open) and re-verified its
  in-src tmpdir line refs against HEAD: bundle.rs 336->331, coverage_note.rs
  420->415, install.rs 1544->1548 (rippled by the shipped edit); import.rs
  382->372 (stale since the earlier PATH-NORMALIZER-CONSOLIDATE ship, caught
  now). Refreshed FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE's install.rs refs
  too (still blockedBy TEST-SCAFFOLDING-CONSOLIDATE, unshipped). Re-checked
  PACKAGING-CHANNELS' parked condition: still true (no
  .github/workflows/release.yml, package.json still the private flume
  manifest).
- Queue: TEST-SCAFFOLDING-CONSOLIDATE (open, pickable) —
  FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE (blockedBy test-scaffolding) —
  TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy test-scaffolding) —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers) — TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders)
  (blockedBy recordwriters) — PACKAGING-CHANNELS (parked, condition
  unchanged).

Plan continues: yes — residue sweep is next: Residue swept through
(425968d) trails HEAD (220e9cd), and jobs 1-3 are now current.
