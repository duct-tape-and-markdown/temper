# Plan state

- Spec derived through: 64828d9 — unchanged, not this tick's job.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: formats next (mid-rotation) — model read and
  swept this tick.
- This tick: POSTURE SWEEP. Inbox and spec delta both drained (empty);
  audit/residue cursors current (no src/sdk/tests commits past 64828d9).
  Rotation continues the cycle 4d1c261 opened (roster from
  architecture.md's codemap: foundation, model, formats, pipeline,
  judges, provider, verbs; foundation swept last tick). `model` was
  already known-touched by 404b73a (kind.rs), so it is this tick's one
  read-and-swept subsystem: all five modules (kind, contract, compose,
  schema, roster) read in full against every engineering.md section and
  architecture.md's model invariant ("kind`/`contract` compile without
  knowing Claude Code exists"). Read clean: no cohesion split, no dead
  plumbing beyond what's already queued, no `_`-arm over a shared
  concept beyond what's already queued, no stored-derived-state, no
  stale intra-doc cite (kind.rs's `extract::` references were checked
  against 404b73a's moved-function set — `manifest_members` and its
  siblings — and kind.rs cites none of them; every `extract::` link it
  carries still resolves). Zero-consumer export check (`rg` for each
  candidate's call sites) confirmed `Format::label`, `ExtentUnit::name`,
  `Shape::name`/`pattern`/`demand`/`admits`, `Charset::allows`,
  `EdgeBound::admits`, and `Predicate::target`/`documented_field` all
  have real cross-module consumers; the three genuine zero-consumer kind.rs
  fns (`Commitment::label`, `Content::label`, `CustomKind::qualified_name`)
  and the `declared_keys` `_`-arm were already queued
  (KIND-ZERO-CONSUMER-EXPORTS-PRUNE, CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH)
  before this tick, so nothing new files. `formats` (json_manifest.rs) is
  also touched by 404b73a — the one-touched-subsystem-per-tick bound
  holds it to next tick, not bulk-skipped (it's touched, not quiet);
  `pipeline` (drift.rs), `provider` (builtin_kind.rs), and `verbs`
  (main.rs, install.rs) are touched too, but formats is next in roster
  order regardless. `judges` (engine/graph/dial/coverage/coverage_note/
  display/reporter) is the one untouched subsystem in this window.
- Queue: 37 pending — unchanged, this tick touched no entries. 7
  pickable OPEN (DRIFT-SOURCE-DEP-PARSE-HOIST, INSTALL-GUARD-MANIFEST-
  MESSAGE-PRUNE, KIND-ZERO-CONSUMER-EXPORTS-PRUNE, IMPORT-ROLLUP-WRITER-
  PLACEMENT, READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION,
  MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT), 27 chained blockedBy, 3 parked on
  human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  MAIN-JUDGE-VERB-HOME-RULING). Open forks unchanged: (multi-harness-
  projection), (lazy-grounds), neither touched. Refactor captures: 0
  live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture-sweep rotation is mid-cycle (`formats`
next) and `formats`'s own file (json_manifest.rs) is already
known-touched by 404b73a, so next tick's job is live without a fresh
forward-window check.
