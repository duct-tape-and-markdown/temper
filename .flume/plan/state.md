# Plan state

- Spec derived through: f87cc0c
- Audited through: 425968d
- Residue swept through: 425968d
- This tick: Residue sweep ef72da3..425968d (job 4). Verified 24f9bbc/ad9f4af
  landed clean — import.rs fully switched to `graph::normalize_path`, no
  leftover `normalize`; builtin.rs/main.rs both wrap
  `compose::clause_from_row`, no reimplementation left behind; `normalize_lf`
  (drift.rs) and `normalize_root` (coverage_note.rs) are distinct jobs, not
  residue. Broader sweep (src/, tests/, sdk/) surfaced one new class: a
  byte-identical closing-`---`-delimiter scan loop duplicated in
  `frontmatter.rs::split_frontmatter` and `install.rs::frontmatter_inner` —
  each function's own doc comment admits it mirrors the other's detection.
  Filed FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE, blockedBy
  TEST-SCAFFOLDING-CONSOLIDATE (shares install.rs+frontmatter.rs with it and
  transitively with PLURAL-HELPER-CONSOLIDATE via their in-src-test-tmpdir
  edits — kept disjoint per the fan-out rule). Considered and ruled out:
  same-named `skill_kind()` test fixtures in frontmatter.rs/coverage_note.rs
  build materially different `CustomKind` shapes for different test
  purposes — not a real duplicate. No hand-rolled reimplementation of a
  sanctioned-crate mechanic found; no duplicate top-level symbols in
  sdk/src or sdk/test. `cargo check` clean.
- Queue: PLURAL-HELPER-CONSOLIDATE (open, pickable) —
  TEST-SCAFFOLDING-CONSOLIDATE (blockedBy plural-helper) —
  FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE (blockedBy test-scaffolding) —
  TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy test-scaffolding) —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers) — TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders)
  (blockedBy recordwriters) — PACKAGING-CHANNELS (parked, condition
  unchanged).

Plan continues: yes — quiet pass (job 5) is next: every input (inbox, spec
delta, ship audit, residue sweep) is now current as of this tick.
