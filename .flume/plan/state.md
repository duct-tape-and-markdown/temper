# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: b89a4372 — unchanged; 81ee251b..HEAD (the FRESH ship
  window) is unreconciled and is next tick's job.
- Residue swept through: b89a4372 — unchanged, same window.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: drained the one refactor capture into
  ROOT-CLAUSE-LOOKUP-ONE-HOME and serialized the three open entries that
  shared `src/drift.rs` into one chain.
- Queue: 15 pending — 3 open, 4 blockedBy, 3 deferred, 5 parked. Pickable: 2
  (disjoint). Open forks: 15. Friction: 1 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation of the FRESH window
(81ee251b..HEAD), which re-cites the entries whose line pointers that ship
moved.
