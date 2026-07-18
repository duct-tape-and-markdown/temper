# Plan state

- Spec derived through: 4adb1fb
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: judges done — provider next
- This tick: INBOX (refactor-capture drain). Inbox itself was empty, but one
  live capture sat in `.flume/refactor/` at tick start —
  `plan-graph-resolved-edge-walk-duplication.md` (filed last tick sweeping
  `judges`, observed e2325df) — which outranks the posture sweep's
  `provider` leg in job order. Re-verified before scoping: `git log
  e2325df..HEAD -- src/ sdk/src/ tests/` is empty (only fa9c87f landed,
  touching `.flume/**` alone), and every cited line in `src/graph.rs`
  (`check` 111-129, `resolved_edges` 967-990, `resolved_arcs` 1001-1010,
  `acyclic`'s call at 196, `degree`'s at 252, `mention_reachable`'s at 371)
  and `src/main.rs` (`gate()`'s four call sites at 1131/1147/1154/1162)
  re-read identical on disk — the claimed duplication (`check`'s from-
  scratch copy plus up to three independent `resolved_edges`/
  `resolved_arcs` recomputations per `gate()` invocation) holds unmoved.
  Filed GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE (`engineering.md`, "Cost scale
  is hoisted, and pinned by count"), leaving the one genuine design call —
  whether `resolved_edges` widens to carry the dangling half `check` needs,
  or `check` keeps a thin filter over a richer enumeration — as a judgment
  call for build, named in the commit body (the ImportError-rename
  precedent). Also caught a fresh consumer while scoping: `src/read.rs`'s
  `why` narration (335) calls `graph::resolved_edges` directly outside
  `gate()`'s run — noted as unaffected by the within-gate consolidation but
  flagged to verify if the output-shape decision ripples there. Gate set to
  `blockedBy DRIFT-SOURCE-DEP-PARSE-HOIST`, not just `DISCOVERY-INFALLIBLE-
  RESULT-COLLAPSE`: this entry also touches `src/main.rs`, and
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE/DRIFT-SOURCE-DEP-PARSE-HOIST
  (upstream in the existing DISCOVERY chain) touch it too — serializing
  behind the chain's last main.rs-touching link avoids a future window
  where two open entries race that file (pending-entry.md, "Disjoint, or
  serialized"). Deleted the capture file after draining it.
- Queue: 18 pending — 7 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS, ROSTER-BUILTIN-KIND-
  NARROWING-RELOCATE, DOCUMENT-RETIRED-FENCE-SURFACE-PRUNE,
  READ-EXPLAIN-STRAND-VISIBILITY-NARROW, ENGINE-SELECTOR-LABEL-ZERO-
  CONSUMER-PRUNE, REPORTER-SEVERITY-WORD-CONSOLIDATE; all disjoint
  files), 9 chained blockedBy (DRIFT-LOCK-ROW-WALK-CONSOLIDATION →
  DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-EXTRACTION →
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-
  PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH →
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, DRIFT-SOURCE-DEP-PARSE-HOIST →
  GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures remain; inbox empty.

Plan continues: yes — the posture sweep resumes at its `provider` leg next
tick (job 4, rotation unchanged: `judges` done, `provider` next).
