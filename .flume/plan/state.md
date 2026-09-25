# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 53c9f544 — unchanged, copied forward: inbox outranked the
  window.
- Residue swept through: bab35c14 — unchanged, copied forward; the hold lifted
  at 02b89d7d, so the sweep is live next tick.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: drained the inbox — the rotted gate command widens
  `(represented-harness-gate-upgrade)`; no entry, the fork's candidates stay
  incompatible.
- Queue: 9 pending — 1 open, 0 blocked, 3 deferred, 5 parked. Pickable: 0
  (INSTALL-LIFTS is open but fork-held). Open forks: 18. Friction: 1 (human
  channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation of 53c9f544..HEAD (three code
commits), with the residue sweep's hold lifted
