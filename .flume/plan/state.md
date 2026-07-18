# Plan state

- Spec derived through: 506c34c — unchanged, not this tick's job.
- Audited through: 3561798 — advanced. Window 64828d9..3561798's only
  src/tests/sdk-touching commits (2df42a0, 9c4a32f, ecf0b44) verified on
  disk against their shipped-entry claims: DRIFT-SOURCE-DEP-PARSE-HOIST
  (gate()/explain() now hold one shared `lock_doc`), BUNDLE-ERROR-ZERO-
  CONSUMER-PRUNE (`BundleError` private), KIND-ZERO-CONSUMER-EXPORTS-
  PRUNE (three dead methods removed) — all three already dropped from
  pending.json by build. The queue-side effect wasn't: six pending
  entries still cited a shipped tag as `blockedBy` (five caught by
  grepping the queue; KIND-ENTRY-SHAPE-DATA-DECLARE's cite to
  KIND-ZERO-CONSUMER-EXPORTS-PRUNE was missed by the shipping tick and
  caught here). Each re-tested: GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE,
  BUNDLE-MANIFEST-PATH-GOVERNS-DERIVE, and CONTRACT-DECLARED-KEYS-
  EXHAUSTIVE-MATCH had no functional dependency, only stale file-safety
  serialization — line cites re-verified against the shipped diffs'
  shifts and corrected. DRIFT-CONFIG-STALE-LOCK-PARSE-HOIST and
  KIND-ENTRY-SHAPE-DATA-DECLARE had real shape drift to reconcile:
  the shipped hoist renamed `read_lock_document` to
  `read_lock_document_for_emit` and added a new fallible `read_lock_document`
  under the old name, and renamed `import_edges_from_lock` to
  `import_edges_from_lock_with_doc` — both entries' descriptions
  rewritten against current disk state, not just relabeled.
- Residue swept through: 3561798 — advanced, same window. DRIFT-SOURCE-
  DEP-PARSE-HOIST's own commit body named `source_deps()`/
  `source_dep_stale()` kept "for backward compatibility"; traced their
  only reachable production path (`import_edges_from_lock_with_doc`'s
  `doc: Option<&DocumentMut>` None branch) and found it dead — both call
  sites always pass `Some`. Filed MAIN-IMPORT-EDGES-DOC-PARAM-COLLAPSE.
  `layout_imports`/`includes`/`layout_import_stale`/`include_stale` keep
  real integration-test consumers (tests/layout_prose_import.rs,
  tests/prose_include.rs) — not residue, not filed.
- Posture swept through: 2d1c5a6 — full rotation closed at 6f862e7
  (verbs, last tick); unchanged, not this tick's job. Re-arms next tick:
  this window's three build commits touched src/ (main.rs, bundle.rs,
  kind.rs) past this cursor.
- This tick: POST-SHIP RECONCILIATION (audit + sweep, see cursors
  above). Unblocking the six stale-blocker entries surfaced file
  collisions in the resulting open set — two pre-existing (READ-CONTEXT-
  MEMBER-CITER-GRAIN/READ-VERB-STRAND-COHESION both open over read.rs;
  IMPORT-ROLLUP-WRITER-PLACEMENT/MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT both
  open over drift.rs — the second is the exact gap last tick's state.md
  flagged as unresolved) and three new from unblocking (GRAPH-RESOLVED-
  EDGE-WALK-CONSOLIDATE vs read.rs, DRIFT-CONFIG-STALE-LOCK-PARSE-HOIST
  vs drift.rs, KIND-ENTRY-SHAPE-DATA-DECLARE vs drift.rs and, separately,
  builtin_kind.rs). All re-serialized behind whichever colliding entry
  was already open (least disruption); the new MAIN-IMPORT-EDGES-DOC-
  PARAM-COLLAPSE finding chained behind MAIN-GUARD-DECLARATIONS-DOUBLE-
  READ-HOIST for the same reason. Full open set re-verified pairwise
  file-disjoint after every change (script-checked, not eyeballed).
- Queue: 47 pending (+1 this tick: MAIN-IMPORT-EDGES-DOC-PARAM-COLLAPSE).
  8 pickable OPEN (INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE, BUNDLE-MANIFEST-
  PATH-GOVERNS-DERIVE, CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH, MAIN-
  GUARD-DECLARATIONS-DOUBLE-READ-HOIST, IMPORT-ROLLUP-WRITER-PLACEMENT,
  READ-CONTEXT-MEMBER-CITER-GRAIN, TAP-PAYLOAD-SCHEMA-SPLIT, COVERAGE-
  KNOWN-SURFACES-RELOCATE — verified pairwise file-disjoint), 36 chained
  blockedBy, 3 parked on human action. Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched this tick.
  Refactor captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep re-arms a fresh cycle (opens at
foundation): 2df42a0/9c4a32f/ecf0b44 touched src/ past
Posture swept through: 2d1c5a6, and no job above is live (inbox empty,
no spec delta past 506c34c, post-ship window reconciled through
3561798 this tick).
