# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: bab35c14 — advanced; the eight `build:` commits plus the
  0.0.19 release pair are reconciled, all eight verified on disk.
- Residue swept through: bab35c14 — advanced with the audit; the one class
  the window exposed is routed as `(guard-locus-binding-clause-gated)`.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: post-ship reconciliation over 30f55b49..HEAD — audit and sweep
  in one window, one new fork, two stale gates re-cut.
- Queue: 12 pending — 4 open, 0 blockedBy, 3 deferred, 5 parked. Pickable: 3
  (disjoint). Open forks: 17. Friction: 1 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: yes — the posture sweep, mid-rotation at sdk/src/builtins.ts,
is the next live input.
