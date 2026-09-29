# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: b04aef62 — both ships verified on disk, every gate reason re-tested.
- Residue swept through: b04aef62 — same window, test-only; no residue filed.
- Posture swept through: src/compose.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs; open —
  src/compose.rs, src/drift.rs, src/glob.rs.
- This tick: reconciled 535eda23..b04aef62 — both ships hold, and the
  hand-lock class's by-behavior census closes on the last filed slice.
- Queue: 10 pending — 3 open, 0 blockedBy, 3 deferred, 4 parked. Pickable: 2
  — DIAL-IS-RESOLVED-ONCE-PER-RUN and the last lock-fixture slice, file-disjoint;
  INSTALL-LIFTS stays fork-held on `(hook-member-identity)`. Open forks: 18.
  Friction: 1 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep resumes at src/compose.rs
