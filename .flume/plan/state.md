# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 909f3c94 — 67841a93..HEAD reconciled this tick.
- Residue swept through: 909f3c94 — same window, swept with the audit.
- Posture swept through: sdk/src/declarations.ts next — mid-rotation.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs,
  sdk/src/builtins.ts; open — sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 67841a93..HEAD — the `type` presence clause shipped
  clean; unparked the tests/ fold, both its stated conditions now met.
- Queue: 9 pending — 1 open, 1 blocked, 3 deferred, 4 parked. Pickable: 1
  (INTEGRATION-SUITE-ONE-TEST-TARGET, the singleton its park required).
  Open forks: 18. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the posture sweep at sdk/src/declarations.ts,
with INTEGRATION-SUITE-ONE-TEST-TARGET pickable now
