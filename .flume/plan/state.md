# Plan state

- Spec derived through: f87cc0c
- Audited through: 220e9cd
- Residue swept through: 38c01c5
- This tick: Residue sweep 425968d..HEAD (38c01c5). Only 4baac1b
  (PLURAL-HELPER-CONSOLIDATE) touched src/tests/sdk in range, already
  ship-audited clean last tick. Grepped for the same suffix-ternary class
  anyway and found src/reporter.rs:100's `verdict()` still inlines
  `if blocking.len() == 1 { "" } else { "s" }` — the shipped entry's file
  list covered bundle.rs/coverage_note.rs/install.rs but never named
  reporter.rs, so this fifth copy survived. Filed
  PLURAL-HELPER-CONSOLIDATE(reporter), open, disjoint from every other
  pending entry's files. No other residue found; grep confirms it's the
  only remaining inline copy in src/.
- Queue: TEST-SCAFFOLDING-CONSOLIDATE (open, pickable) —
  PLURAL-HELPER-CONSOLIDATE(reporter) (open, pickable, disjoint) —
  FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE (blockedBy test-scaffolding) —
  TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy test-scaffolding) —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers) — TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders)
  (blockedBy recordwriters) — PACKAGING-CHANNELS (parked, condition
  unchanged).

Plan continues: yes — quiet closing pass is next: inbox empty, spec delta
empty, ship audit current (220e9cd), residue swept through HEAD.
