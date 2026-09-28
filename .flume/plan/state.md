# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 70afd6e0 — 909f3c94..HEAD reconciled this tick.
- Residue swept through: 70afd6e0 — same window, swept with the audit.
- Posture swept through: sdk/src/declarations.ts next — mid-rotation.
  Frozen frontier (armed at 7d695577), paths re-spelled for a7c8c9b5's fold:
  covered — src/read.rs, src/telemetry.rs, tests/it/read_verbs.rs,
  src/admissibility.rs, src/gate.rs, src/graph.rs, tests/it/graph.rs,
  src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs, sdk/src/builtins.ts;
  open — sdk/src/declarations.ts, tests/it/emit.rs, src/install.rs,
  tests/it/install.rs, src/compose.rs, src/drift.rs, src/glob.rs,
  src/placement.rs.
- This tick: reconciled 909f3c94..HEAD — the 59→1 test-target fold shipped
  clean; re-cited the three entries and three fork cites it path-rotted.
- Queue: 8 pending — 1 open, 3 deferred, 4 parked. Pickable: 0 (the one
  open entry rests on `(hook-member-identity)`). Open forks: 18. Friction:
  1 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the posture sweep at sdk/src/declarations.ts, with no
pickable entry for build to take
