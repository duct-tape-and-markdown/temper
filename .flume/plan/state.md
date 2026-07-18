# Plan state

- Spec derived through: 64828d9 — advanced from 2d66fc9. The one
  intervening commit (architecture.md's Invariants section) is wording-
  only, no new specs/decisions/ file, so the Consequences-checklist
  discipline doesn't apply — it's a plain reconciliation slice, routed by
  verification. Three edges move from "declared debt, entries queued" to
  "history, shipped": reverified on disk this tick, not taken on the
  commit body's word. `drift → install`: drift.rs's only `install::` hit
  is a rustdoc link (`[`install::matches_projection`]`), not a `use` —
  dissolved. `frontmatter → builtin_kind`: zero `builtin_kind` references
  anywhere in src/frontmatter.rs (test-only, matching the ruling) —
  dissolved. `extract`'s upward imports: extract.rs carries zero
  `crate::install`/`crate::drift` imports, only two `crate::address::
  FieldPath` uses — an intra-foundation leaf reference, not an upward
  edge — dissolved. The foundation invariant's reword ("nothing outside
  foundation," not "nothing internal") licenses exactly that extract→
  address edge as an intra-foundation absence, never a violation; grepped
  pending.json for any entry resting on the old "nothing internal"
  wording — none found. The one edge the delta still names as live
  (`normalize_path`, graph.rs → address.rs) was already queued
  (NORMALIZE-PATH-SUBSYSTEM-PLACEMENT), already citing this same amended
  section ahead of the spec landing — reverified still `blockedBy
  GRAPH-WORLD-ZERO-CONSUMER-PRUNE`, cites intact, nothing to change.
  Every slice of this delta resolves to either a verified-already-shipped
  claim or an already-queued entry — fully routed, zero new pending
  entries, pending.json untouched this tick.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: 1973522 — unchanged, not this tick's job.
- This tick: SPEC DELTA. 64828d9 routed — see cursor line above for the
  full verification; no entries filed or rewritten.
- Queue: 37 pending — unchanged, this tick touched no entries. 7
  pickable OPEN (DRIFT-SOURCE-DEP-PARSE-HOIST, INSTALL-GUARD-MANIFEST-
  MESSAGE-PRUNE, KIND-ZERO-CONSUMER-EXPORTS-PRUNE, IMPORT-ROLLUP-WRITER-
  PLACEMENT, READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION,
  MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT), 27 chained blockedBy, 3 parked on
  human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  MAIN-JUDGE-VERB-HOME-RULING). Open forks unchanged: (multi-harness-
  projection), (lazy-grounds), neither touched. Refactor captures: 0
  live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — commits past `Posture swept through:` (1973522)
touched src/ (278ae4c, 404b73a); posture sweep's forward window is
non-empty and outranks nothing above it now that inbox and spec delta
are both drained — next tick's job.
