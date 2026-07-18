# Plan state

- Spec derived through: 4adb1fb
- Audited through: 1f6afe5
- Residue swept through: 1f6afe5
- Posture swept through: 04cbd6d rotation closed; re-verifying subsystems
  touched by 1f6afe5's ship before resuming the new cycle at foundation —
  pipeline and judges re-verified this tick (pipeline quiet; judges filed
  work); provider next, then foundation opens
- This tick: INBOX — job 1 (refactor-captures held live content; no inbox
  lines). Both live captures re-verified unmoved at HEAD (`git log
  4baa5c4..HEAD -- src/` empty, the `observed at` sha both captures
  shared) and drained into pending entries citing engineering.md/
  architecture.md, then deleted:
  plan-graph-engine-glob-extraction-duplication.md → filed
  GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE (graph.rs's declared_globs vs
  engine.rs's field_globs, a real trim/filter drift between the two
  glob-extraction implementations); plan-normalize-path-subsystem-
  placement.md → filed NORMALIZE-PATH-SUBSYSTEM-PLACEMENT
  (graph.rs's normalize_path is foundation-shaped leaf vocabulary
  consumed upward by pipeline modules, homed in judges). Both filed
  `parked`, not `open`: each capture's own text named a human ruling as
  the blocker before mechanical work could scope — the glob entry
  because unifying the two extractors changes glob-valid's observable
  findings (a product call the trim/filter choice makes, not an
  implementation-shape one); the normalize_path entry because
  architecture.md's Invariants section names only 0040's three debt
  edges, and filing a move outright would assert a fourth the page
  hasn't ratified (architecture.md is human-ratified, never derived).
  Parked (not `open` + `dependsOnForks`) since no open-questions fork
  gives either question a cleaner home than the parked reason itself,
  and parked entries sit outside the disjoint-or-serialized rule that
  binds `open` entries — so neither collides with
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH (open, shares engine.rs) or
  the graph.rs chain (GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE /
  GRAPH-WORLD-ZERO-CONSUMER-PRUNE).
- Queue: 28 pending (was 26), 8 pickable OPEN
  (TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE,
  ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE,
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE,
  FRONTMATTER-COMPANIONS-ZERO-CONSUMER-PRUNE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH, DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION), 16 chained blockedBy, 4 parked on
  human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE,
  NORMALIZE-PATH-SUBSYSTEM-PLACEMENT). Every shared-file pair among
  `open` entries serialized per pending-entry.md (verified: no two
  simultaneously-open entries share a path — the two new parked entries
  touch graph.rs/engine.rs but are not `open`). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged, neither entry
  above used dependsOnForks. Refactor captures: none live
  (.flume/refactor/ holds only README.md). Inbox empty.

Plan continues: yes — job 4 remains live: provider (builtin_kind.rs) was
touched in the same post-04cbd6d window and still needs its own
re-verification before the new rotation cycle can open at foundation.
