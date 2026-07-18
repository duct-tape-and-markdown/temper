# Plan state

- Spec derived through: 64828d9 — unchanged, not this tick's job.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: model next (mid-rotation) — foundation read and
  swept this tick.
- This tick: POSTURE SWEEP. Inbox and spec delta both drained (empty);
  audit/residue cursors current (no src/sdk/tests commits past 64828d9).
  Posture cursor (1973522) was behind a forward window that touched
  src/ (278ae4c, 404b73a) — a fresh rotation cycle, roster from
  architecture.md's codemap (foundation, model, formats, pipeline,
  judges, provider, verbs). `foundation` is first and touched
  (404b73a moved code out of extract.rs), so it is this tick's one
  read-and-swept subsystem: all six modules (check, tap, hash, address,
  json_splice, extract) read in full against every engineering.md
  section and architecture.md's foundation invariant. One finding: the
  404b73a extraction left `src/extract.rs`'s doc comment for the
  now-relocated `manifest_members` orphaned mid-function — its closing
  sentence truncates and runs straight into `number_kind`'s unrelated
  doc comment three lines below — plus a companion stale intra-doc cite
  at `src/json_manifest.rs:335` (`[extract::manifest_members]`, now
  self-referential since the function lives in that file). Both are
  comment/citation staleness, the one sweep-lens exception that never
  files a standalone entry (open-questions.md, "One stale cite,
  ride-only") — routed into that ledger's live-orphan list (now six),
  not pending.json; pending.json untouched this tick. Everything else
  read clean: no cohesion split, no dead plumbing, no zero-consumer
  export, no `_`-arm over a shared concept, no stored-derived-state, in
  any of the six modules. `model` is also touched by 404b73a (kind.rs)
  but the one-touched-subsystem-per-tick bound holds it to next tick,
  not bulk-skipped (it's touched, not quiet).
- Queue: 37 pending — unchanged, this tick touched no entries. 7
  pickable OPEN (DRIFT-SOURCE-DEP-PARSE-HOIST, INSTALL-GUARD-MANIFEST-
  MESSAGE-PRUNE, KIND-ZERO-CONSUMER-EXPORTS-PRUNE, IMPORT-ROLLUP-WRITER-
  PLACEMENT, READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION,
  MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT), 27 chained blockedBy, 3 parked on
  human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  MAIN-JUDGE-VERB-HOME-RULING). Open forks unchanged: (multi-harness-
  projection), (lazy-grounds), neither touched. Refactor captures: 0
  live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture-sweep rotation is mid-cycle (`model`
next) and `model`'s own file (kind.rs) is already known-touched by
404b73a, so next tick's job is live without a fresh forward-window
check.
