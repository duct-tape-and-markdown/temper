# Plan state

- Spec derived through: 39a4833
- Audited through: 525111a
- Residue swept through: 525111a
- This tick: INBOX. Drained the one note (mention-discovery deferral, observed
  955dc30) into pending MENTION-ROUTE-RESOLVE-AT-CHECK, cite pipeline.md
  "Emit". Gap re-verified on disk at HEAD 7a3632f (window 955dc30..HEAD is one
  docs(example) commit touching no engine code): `resolved_mention_edges`
  (graph.rs:882) folds every lock mention in as already-resolved; `graph::check`
  (816) route-resolves only declared `Edge`s; `read.rs:303` extends the resolved
  set with every mention unconditionally — so a deferred mention (drift.rs:2472
  MentionRow doc: "rides the lock for check to resolve against the discovered
  corpus") is route-verified nowhere. Field-verified scenario (base-harness
  verify-summary mentions source:main, src/main.js moved away → check+explain
  stay green) reproduced by the docs(example) commit that inboxed it. Clean cite
  → pending, not open-questions; SDK/lock half already shipped, only the engine
  check/explain verdict is missing. Entry disjoint from parked PACKAGING.
- Queue: 1 pickable — MENTION-ROUTE-RESOLVE-AT-CHECK (open, edits
  graph.rs/main.rs/read.rs). PACKAGING-CHANNELS-REMAINDER parked (human release
  actions).

Plan continues: no — inbox drained; spec delta empty (39a4833 last spec commit,
fully derived); no src/tests/sdk commit past 525111a to reconcile (7a3632f
touched examples/ + docs/ only). No live plan input remains, and
MENTION-ROUTE-RESOLVE-AT-CHECK is pickable, so build takes over. NB the
SessionStart reporter still shows the `.temper` dogfood gate red
(friction-capture-procedure, pending-entry-discipline unfilled) — harness
territory, a `chore(harness)` fix outside plan's writable paths.
