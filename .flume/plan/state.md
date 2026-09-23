# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: 30f55b49 — unchanged; the window past it is eight `build:`
  commits plus the 0.0.19 release pair, unreconciled.
- Residue swept through: 30f55b49 — unchanged, copied forward.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: drained the inbox — three notes, all re-verified on disk, into
  three open entries plus the `(unique-over-a-list)` fork.
- Queue: 12 pending — 4 open, 0 blockedBy, 3 deferred, 5 parked. Pickable: 3
  (disjoint). Open forks: 16. Friction: 1 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation over 30f55b49..HEAD, the
window the inbox job displaced.
