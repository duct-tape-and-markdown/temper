# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: cad9ccd0 — unchanged, copied forward: 53545c8c is past
  it and unreconciled.
- Residue swept through: cad9ccd0 — unchanged, copied forward.
- Posture swept through: sdk/src/declarations.ts next — mid-rotation.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs,
  sdk/src/builtins.ts; open — sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: drained the one refactor capture — the `when` guard projection
  gap filed as its own entry, the hook entry serialized behind it.
- Queue: 11 pending — 2 open, 1 blocked, 3 deferred, 5 parked. Pickable: 1
  (LOCK-ROUND-TRIP-PIN; INSTALL-LIFTS is open but fork-held). Open forks: 18.
  Friction: 1 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation of cad9ccd0..HEAD
(53545c8c's widened hook event set)
