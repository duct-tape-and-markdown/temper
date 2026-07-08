# Plan state

- Spec derived through: f87cc0c
- Audited through: 01b5205
- Residue swept through: f10dd77
- This tick: Ship audit (job 3). Commits past the old cursor (f10dd77):
  8865a7e (prior plan commit, `.flume/plan/*` only) then five build commits
  shipping TMPDIR-HELPER-CONSOLIDATE(mainrs) (9978e0f), TMPDIR-HELPER-
  CONSOLIDATE(kindrs) (c844b2f), FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE
  (89c8b7d), and TEST-FIXTURE-HELPERS-CONSOLIDATE (0735474), landed together
  in 01b5205 which already dropped all four from pending.json. Verified on
  disk (not just the log): kind.rs's tmpdir rename, main.rs's tmpdir(label)
  use, frontmatter::closing_delimiter + its install.rs call sites, and
  tests/common's write_skill/check_in/author_satisfies/tree_bytes all
  present as described. TEST-FIXTURE-HELPERS-CONSOLIDATE shipping resolved
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters)'s blockedBy — its six target
  symbols still exist byte-identical, but 0735474 shifted every cited line
  number across gate_fail_loud.rs/requirement_roster.rs/coverage.rs/
  graph.rs/lock_declaration_rows.rs; rewrote the entry's file citations
  against current line numbers and flipped its gate to open. lockunitbuilders
  stays blockedBy recordwriters (still open, not yet shipped). Re-verified
  PATH-SEP-NORMALIZE-CONSOLIDATE's and SDK-SORT-COMPARATOR-CONSOLIDATE's
  citations untouched by these commits and still accurate on disk.
  PACKAGING-CHANNELS's parked reason re-checked, unrelated and still holds.
- Queue: PATH-SEP-NORMALIZE-CONSOLIDATE, SDK-SORT-COMPARATOR-CONSOLIDATE,
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) all open and pairwise
  disjoint, pickable in parallel — TEST-HELPER-DUPES-CONSOLIDATE
  (lockunitbuilders) (blockedBy recordwriters) — PACKAGING-CHANNELS (parked,
  condition unchanged).

Plan continues: yes — residue sweep trails HEAD (swept through f10dd77;
src/tests/sdk changed since in the five build commits this tick audited) and
is the next live input.
