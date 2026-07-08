# Plan state

- Spec derived through: f87cc0c
- Audited through: 220e9cd
- Residue swept through: 38c01c5
- This tick: Inbox job. The harness's refactor-capture probe errored
  (shell-quoting exec-failed), but `.flume/refactor/` held one live capture
  the probe missed: `build-main-backing-tmpdir.md` (observed at 96e947d) —
  src/main.rs's one test hand-rolls a temp dir (`env::temp_dir()` +
  `process::id()` + manual `remove_dir_all`) instead of the `tempfile`-backed
  `tmpdir(label)` helper TEST-SCAFFOLDING-CONSOLIDATE gave every other
  in-src `#[cfg(test)]` module. Re-verified live at HEAD (src/main.rs:1141
  unchanged since 96e947d); filed TMPDIR-HELPER-CONSOLIDATE(mainrs), open
  and disjoint (touches only src/main.rs, no other entry does); deleted the
  capture.
- Queue: TMPDIR-HELPER-CONSOLIDATE(mainrs) (open, pickable) —
  FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE (blockedBy test-scaffolding,
  shipped-but-unaudited) — TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy
  test-scaffolding, shipped-but-unaudited) — TEST-HELPER-DUPES-CONSOLIDATE
  (recordwriters) (blockedBy test-fixture-helpers) —
  TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders) (blockedBy recordwriters)
  — PACKAGING-CHANNELS (parked, condition unchanged).

Plan continues: yes — ship audit is live next: 1894af9 and 7329585 (both
`build:` commits shipping TEST-SCAFFOLDING-CONSOLIDATE and
PLURAL-HELPER-CONSOLIDATE(reporter)) landed past `Audited through: 220e9cd`
and touch src/tests; the two blockedBy-TEST-SCAFFOLDING-CONSOLIDATE entries
above need their gate condition re-verified against shipped disk state.
