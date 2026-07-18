# Plan state

- Spec derived through: 4adb1fb
- Audited through: 1f6afe5
- Residue swept through: 1f6afe5
- Posture swept through: 04cbd6d — rotation closed (foundation, model,
  formats, pipeline, judges, provider, verbs all swept this cycle);
  foundation next cycle
- This tick: POST-SHIP RECONCILIATION. Jobs 1-2 quiet: inbox and
  refactor-captures empty; `git log 4adb1fb..HEAD -- specs/` empty (no
  spec delta). Job 3 was live: `git log 2f44341..HEAD -- src/ sdk/src/
  tests/` returned eabd046/956811a/4c3b268/84197e5 (shipped in 1f6afe5) —
  READ-EXPLAIN-STRAND-VISIBILITY-NARROW, ENGINE-SELECTOR-LABEL-ZERO-
  CONSUMER-PRUNE, REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-
  QUALIFIED-ZERO-CONSUMER-PRUNE, all already removed from pending.json by
  the ship. Audit: read all four diffs against their entries' acceptance
  criteria — each is signature/consolidation-only exactly as scoped, no
  behavior change, no residue. Re-tested the one stale gate the window
  created: BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE's `blockedBy
  BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE` pointed at a tag that no
  longer exists. Verified on disk (`qualified`/its test gone,
  `definition`/`definitions` still `Result`-wrapped, all 13 test call
  sites re-counted since the prune shifted every line past 509) and
  refreshed its builtin_kind.rs citations to current line numbers — every
  other cited file (bundle.rs, import.rs, install.rs, main.rs, 18
  tests/*.rs) unmoved since scoping (0dd9892). Unblocking straight to
  `open` would have collided on src/kind.rs with KIND-MEMBER-DOCUMENT-
  ZERO-CONSUMER-PRUNE (already open, same file) — serialized behind it
  instead (pending-entry.md, "Disjoint, or serialized"), no functional
  dependency, first-open-wins. Sweep: same window, no corpus-named
  retirement, no living demolished symbol, no second implementation
  introduced — the four ships are exactly the subtraction their entries
  specified.
- Queue: 22 pending — 6 pickable OPEN (TAP-LOG-FILENAME-ZERO-CONSUMER-
  PRUNE, ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE,
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE,
  FRONTMATTER-COMPANIONS-ZERO-CONSUMER-PRUNE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; zero path collisions, verified this
  tick), 14 chained blockedBy (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE →
  now behind KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE (kind.rs
  collision); DRIFT-LOCK-ROW-WALK-CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-
  HOIST → PLACEMENT-MODULE-EXTRACTION → {EXTRACT-FOUNDATION-BOUNDARY-
  RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-PRUNE → CONTRACT-DECLARED-KEYS-
  EXHAUSTIVE-MATCH → CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP →
  DRIFT-EMIT-OUTCOME-LABEL-ZERO-CONSUMER-PRUNE, DRIFT-SOURCE-DEP-
  PARSE-HOIST → GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE →
  GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST}, INSTALL-PROJECTION-MATCH-
  CONSOLIDATE → INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE →
  INSTALL-PLACEMENT-KIND-ENUM}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — job 4 is live: `Posture swept through: 04cbd6d`
names a commit behind HEAD whose forward window (`git log 04cbd6d..HEAD
-- src/ sdk/src/ tests/`) touched pipeline (read.rs), judges (engine.rs,
reporter.rs), and provider (builtin_kind.rs) — a touched subsystem
cannot skip forward even after a rotation closed, so next tick reads
those subsystems in full before resuming at foundation.
