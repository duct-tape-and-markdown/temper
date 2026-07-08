# Plan state

- Spec derived through: f87cc0c
- Audited through: 63f3c61
- Residue swept through: 63f3c61
- This tick: Residue sweep (job 4). No earlier job was live: spec delta is
  empty past f87cc0c; the only commit past Audited through (63f3c61) is
  e24b9dc, a plan-only commit touching none of src/tests/sdk, so nothing new
  to ship-audit. Filed IN-SRC-TMPDIR-HELPER-CONSOLIDATE(driftcoveragenote):
  the candidate the last tick's ship audit flagged but deferred —
  src/drift.rs:1790 and src/coverage_note.rs:416 each still carry their own
  byte-identical `fn tmpdir(label)`, left unmerged by a9942d4
  (IN-SRC-TMPDIR-HELPER-CONSOLIDATE, 8-of-10) because drift.rs was then
  entangled with the open PATH-SEP-NORMALIZE-CONSOLIDATE. That shipped at
  366e35a; re-verified both copies still present and byte-identical to
  `src/test_support::tmpdir` at HEAD, and that no other `fn tmpdir(label`
  copy survives outside test_support.rs and tests/common/mod.rs (the
  separate integration-test-tier home, a different job). New entry is
  disjoint from all three standing entries — distinct files, no shared path.
- Queue: IN-SRC-TMPDIR-HELPER-CONSOLIDATE(driftcoveragenote) open —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) open — TEST-HELPER-DUPES-
  CONSOLIDATE(lockunitbuilders) blockedBy recordwriters — PACKAGING-CHANNELS
  parked (condition re-verified unchanged: no `.github/workflows/
  release.yml`, root package.json still the private flume manifest).

Plan continues: no — every input is current (inbox empty, spec delta empty,
ship audit has nothing new past a plan-only commit, residue swept through
HEAD). Three entries are open/pickable (one newly filed, one blockedBy
chain); one is parked on human action. Build takes over.
