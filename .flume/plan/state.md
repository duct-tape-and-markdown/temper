# Plan state

- Spec derived through: 53df138 — its only specs/ diff is the
  normalize_path Invariants amendment, and that content is now fully
  routed (see this tick): nothing past this commit remains un-derived.
- Audited through: f713a08
- Residue swept through: f713a08
- Posture swept through: 4e9d87a — formats done this tick (found
  JSON-MANIFEST-READ-DECODE-CONSOLIDATE and
  TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE, both opened as blockedBy
  chains behind the one currently-open entry each shares a file with).
  This rotation pass has now covered pipeline (4baa5c4, quiet), judges
  (f3980b9), provider (ab4c07d, quiet), foundation (08b5a27), model
  (1c5b0a9), and formats (this tick) — verbs is the one subsystem not
  yet ticked this pass. `git log cfa50fb..HEAD -- src/main.rs
  src/install.rs src/bundle.rs src/lib.rs src/test_support.rs` (cfa50fb
  is verbs' own last sweep) is empty, so verbs is likely quiet, but it
  still needs its tick before this pass can close and the cursor
  advance.
- This tick: INBOX. Two human rulings (John's 07-18 delegation,
  routed via inbox, observed at 07a9c04) resolved both design-call
  parks from b80fec2/f3980b9:
  (1) NORMALIZE-PATH-SUBSYSTEM-PLACEMENT — architecture.md's
  Invariants section now names normalize_path's move (graph.rs →
  address.rs) as the fourth ratified debt edge (53df138). Unparked to
  blockedBy GRAPH-WORLD-ZERO-CONSUMER-PRUNE — the last entry sharing
  graph.rs and (transitively, via the DRIFT-SOURCE-DEP-PARSE-HOIST /
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE chain) drift.rs; import.rs's
  overlap with the separately-queued, already-open
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE is unexpressed by the
  single-tag schema but that entry is queue-front and ships first
  regardless. Reworded every file description from conditional
  ("if the ruling ships") to definite; line citations re-verified
  unmoved since f3980b9 (only address.rs shifted 2 lines in the
  window, unrelated).
  (2) GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE — the ruling names the
  shared extractor's semantics: declared_globs' trim/filter, both
  judges (reachability, glob-valid) consuming one filtered set; no new
  blank-entry finding class. Unparked to blockedBy
  NORMALIZE-PATH-SUBSYSTEM-PLACEMENT (shares graph.rs); engine.rs's
  overlap with the separately-queued, already-open
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH is likewise unexpressed but
  that entry ships first regardless. Both entries' acceptance/tests
  reworded to state the ruled behavior instead of asking build to
  invent it. Drained both inbox lines (now empty).
- Queue: 29 pending, 6 pickable OPEN (unchanged set:
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH, DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE, DRIFT-LOCK-ROW-WALK-
  CONSOLIDATION — pairwise disjoint on files, unchanged), 21 chained
  blockedBy (19 prior + the 2 unparked this tick), 2 parked on human
  action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — reasons
  unchanged, untouched this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — verbs, the one subsystem this rotation pass
has not yet ticked, is live next tick (likely a quiet verdict, which
would close the pass and advance the cursor).
