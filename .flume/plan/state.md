# Plan state

- Spec derived through: 2d66fc9 — unchanged. One intervening specs/ commit
  (64828d9, architecture.md's Invariants section) sits past this cursor,
  unrouted; not this tick's job (inbox outranked it).
- Audited through: 64828d9 — advanced from 870c52d. Window's src-touching
  commits (404b73a, 278ae4c) verified on disk: EXTRACT-FOUNDATION-BOUNDARY-
  RESTORE shipped (extract.rs carries no crate-internal `use`, only doc-
  comment mentions; drift.rs/json_manifest.rs hold the moved lifters and
  grammar at documented visibility) and INSTALL-PROJECTION-MATCH-
  CONSOLIDATE shipped (install.rs's `path_matches` is the one shared
  suffix-match helper, `matches_projection` its only caller). Both entries
  were already removed from the queue by a8dd112 (chore); this tick's
  audit confirmed the removal was earned, not premature.
- Residue swept through: 64828d9 — same window. No new residue: both
  commits are themselves the consolidation, re-checked (no duplicate
  path-matching fn survives in install.rs; extract.rs's only sibling
  references are doc-comment mentions, not `use` imports).
- Posture swept through: 1973522 — unchanged, not this tick's job.
- This tick: INBOX. Two notes drained.
  (1) RULED — unparked all four sweep-parked cohesion/placement entries
  per the standing delegation: IMPORT-ROLLUP-WRITER-PLACEMENT flips open
  (move to drift.rs, no codemap edit needed — drift already owns "the
  lock" and is the sole caller). READ-CONTEXT-MEMBER-CITER-GRAIN flips
  open, narrowed to consolidating the shared selection predicate only
  (print stays per-grain — ruled more indirection than the ~30 shared
  lines earn). READ-VERB-STRAND-COHESION flips open, narrowed to
  extracting only the telemetry field strand (`field`/`event_label`,
  1464-1545, zero shared machinery) to a new `src/telemetry.rs`; the four
  Species-dispatched traversals stay under read.rs's one-verb framing
  (real cohesion, ruled to stay). MAIN-THIN-DISPATCH-COHESION is derived
  into a chain per the ruling ("derive the chain, and a function whose
  home is genuinely ambiguous parks that one question, never the
  campaign"): MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT (open — six row→model
  lifters move beside the lock-row types they interpret) chains ahead of
  MAIN-CORPUS-ASSEMBLY-TO-COMPOSE (blockedBy it — five corpus-assembly
  fns move to compose.rs's "member composition" home) for main.rs
  shared-file safety. Verified on disk before deriving: the seven
  admissibility/collision-diagnostic fns don't cluster by any visible
  engine.rs/graph.rs seam (only one of the seven calls `engine::`
  anything), and gate/explain's own shape (a possible new pipeline-facing
  entry point) is a distinct shape call — both genuinely ambiguous, so
  MAIN-JUDGE-VERB-HOME-RULING parks them as one standalone question,
  blocking neither chained entry.
  (2) NOTE — the architecture-debt-list friction capture was already
  drained by 64828d9 itself (the human specs+flume commit that filed
  these two inbox notes); confirmed the file is gone from
  `.flume/friction/`. Nothing left to route for this note beyond
  acknowledging it.
  Necessary side-fix (gate compliance, not a separate job): three entries
  — DRIFT-SOURCE-DEP-PARSE-HOIST, KIND-ZERO-CONSUMER-EXPORTS-PRUNE
  (both blockedBy EXTRACT-FOUNDATION-BOUNDARY-RESTORE), INSTALL-GUARD-
  MANIFEST-MESSAGE-PRUNE (blockedBy INSTALL-PROJECTION-MATCH-CONSOLIDATE)
  — carried `blockedBy` references to tags a8dd112 had already deleted
  from the queue, a dangling reference the "entry references resolve"
  gate would have reverted this commit over regardless of job choice. Re-
  verified all three premises true on disk (grep-confirmed zero outside
  consumers/single-parse claims still hold), refreshed drifted line
  cites, flipped all three to open. Also corrected open-questions.md's
  drift.rs `RawLockRow` orphan record, which still named the now-shipped
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE as a live rider.
- Queue: 37 pending — unchanged count (3 unparked in place, 1 parked
  entry split into 2 chained open + 1 parked, net zero). 7 pickable OPEN
  (DRIFT-SOURCE-DEP-PARSE-HOIST, INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE,
  KIND-ZERO-CONSUMER-EXPORTS-PRUNE, IMPORT-ROLLUP-WRITER-PLACEMENT,
  READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION,
  MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT), 27 chained blockedBy (all
  resolving to live tags, re-verified), 3 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  MAIN-JUDGE-VERB-HOME-RULING). Open forks unchanged: (multi-harness-
  projection), (lazy-grounds), neither touched. Refactor captures: 0
  live. Friction: 0 live (the one capture on record was already drained
  by 64828d9, ahead of this tick). Inbox: 0 notes.

Plan continues: yes — the spec delta (64828d9, architecture.md's
Invariants section, past the 2d66fc9 cursor) is unrouted and outranks
posture sweep; next tick derives it.
