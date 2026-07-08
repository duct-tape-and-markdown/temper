# Plan state

- Spec derived through: f87cc0c
- Audited through: 37a92f0
- Residue swept through: da9245c
- This tick: Residue sweep (job 4). Inbox empty, no refactor captures, spec
  delta empty, and `git log 37a92f0..HEAD -- src/ tests/ sdk/` empty — jobs
  1-3 all quiet, so this tick swept src/tests/sdk against the corpus for
  vocabulary the corpus no longer sanctions and second implementations of
  one job, beyond the six already-filed consolidation entries. An Explore
  agent's findings were independently re-verified line-by-line (not taken on
  faith): (1) three doc-comment cites to the deleted `packages/*/PACKAGE.md`
  tree survive in sdk/src/builtins.ts:308,348,385 — this is comment/citation
  staleness, the one exception job 4 carves out ("rides whichever entry
  next opens that file, never a standalone entry"); docs/ledger.md already
  independently tracks it as parked human follow-up hygiene, so nothing
  filed. (2) Five more copy-pasted integration-test builders confirmed live
  across the same file set TEST-SCAFFOLDING-CONSOLIDATE already touches —
  write_harness/write_skill (byte-identical body under two names, 6 sites),
  CheckRun+check_in (3 exact + 2 near-dup shapes), author_satisfies (two
  arities, 2 sites each), and tree_bytes (triplicated AND a hand-rolled walk
  where the sanctioned `walkdir` crate — already a direct Cargo dependency —
  fits). Filed as TEST-FIXTURE-HELPERS-CONSOLIDATE, `blockedBy:
  TEST-SCAFFOLDING-CONSOLIDATE` since every file it touches is already in
  that entry's edit list — two opens on the same files would conflict at
  merge. No other duplicate helpers, hand-rolled glob/regex/serialization,
  or retired vocabulary found beyond what's already filed. Cursor advances
  to HEAD (da9245c).
- Queue: WINDOWS-VENDOR-SYMLINK-JUNCTION (open) — GLOB-ENGINE-CONSOLIDATE
  (open) — CLAUSE-FROM-ROW-CONSOLIDATE (open) — SDK-SEAM-ENCODE-CONSOLIDATE
  (open) — PATH-NORMALIZER-CONSOLIDATE (blockedBy glob) —
  PLURAL-HELPER-CONSOLIDATE (blockedBy glob) — TEST-SCAFFOLDING-CONSOLIDATE
  (blockedBy windows-vendor) — TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy
  test-scaffolding) — PACKAGING-CHANNELS (parked, condition unchanged).

Plan continues: no — inbox, spec delta, ship audit, and residue sweep are
all current; four entries are `open` and pickable now, so build takes over.
