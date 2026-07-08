# Plan state

- Spec derived through: f87cc0c
- Audited through: b749f80
- Residue swept through: b749f80
- This tick: Ship audit (job 3) was quiet first — the one commit past the
  old cursor (04b3f20, b749f80's own prior tick) touches only
  `.flume/plan/state.md`, no src/tests/sdk; cursor advances to HEAD. Residue
  sweep (job 4) then ran: dispatched a fresh sweep of src/, tests/, sdk/ for
  duplication not already covered by the 7 queued entries. Confirmed no new
  test-scaffolding dupes. Found and verified 2 new classes: (1) a `\`->`/`
  path-normalize mechanic (`drift::to_lock_path`) reimplemented inline at 4
  more sites (main.rs:918, coverage_note.rs:196, document.rs:327,
  install.rs:676/678) — filed PATH-SEP-NORMALIZE-CONSOLIDATE for the 2 sites
  disjoint from open work (document.rs, coverage_note.rs); the main.rs and
  install.rs sites are deferred, entangled with open TMPDIR-HELPER-
  CONSOLIDATE(mainrs)/FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE's file scope
  per disjoint-or-serialize. (2) `sdk/src/declarations.ts`+`emit.ts`'s
  lexicographic string-comparator `(a,b)=>(a<b?-1:a>b?1:0)` reimplemented
  inline 7 times — filed SDK-SORT-COMPARATOR-CONSOLIDATE, disjoint (no
  other entry touches sdk/). One candidate rejected: `builtins.ts`'s 3 doc
  comments citing deleted `packages/*.PACKAGE.md` paths are comment
  staleness, riding whichever entry next opens that file per the rule — not
  a standalone entry. Cursor advances to HEAD.
- Queue: TMPDIR-HELPER-CONSOLIDATE(mainrs), TMPDIR-HELPER-CONSOLIDATE
  (kindrs), FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE, TEST-FIXTURE-HELPERS-
  CONSOLIDATE, PATH-SEP-NORMALIZE-CONSOLIDATE, SDK-SORT-COMPARATOR-
  CONSOLIDATE all open and pairwise disjoint, pickable in parallel —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers) — TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders)
  (blockedBy recordwriters) — PACKAGING-CHANNELS (parked, condition
  unchanged).

Plan continues: yes — every input (inbox, spec delta, ship audit, residue
sweep) is now current; next tick is the quiet closing pass (job 5).
