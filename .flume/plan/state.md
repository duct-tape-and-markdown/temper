# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 2f95f0a1 — copied forward; 2f95f0a1..535eda23 is unreconciled.
- Residue swept through: 2f95f0a1 — copied forward, same window as the audit.
- Posture swept through: src/compose.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs; open —
  src/compose.rs, src/drift.rs, src/glob.rs.
- This tick: drained the refactor channel — the hand-lock capture became two
  file-disjoint slices and the capture file is gone.
- Queue: 10 pending — 3 open, 0 blockedBy, 3 deferred, 4 parked. Pickable: 2
  — the two lock-fixture slices; INSTALL-LIFTS stays fork-held on
  `(hook-member-identity)`. Open forks: 18. Friction: 1 (human channel).
  Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — reconcile 2f95f0a1..535eda23, then the rotation
resumes at src/compose.rs
