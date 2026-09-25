# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 0c56cf0a — the seven-commit window verified on disk: five
  `build:` surfaces present, both `release:` commits at 0.0.20.
- Residue swept through: bab35c14 — unchanged, copied forward (sweep held);
  the window's two stated deferrals wait for it.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: audited bab35c14..HEAD — nothing dropped, every gate re-tested
  and still true, three moved cites re-stamped.
- Queue: 15 pending — 4 open, 3 blocked, 3 deferred, 5 parked. Pickable: 3
  (disjoint; INSTALL-LIFTS is open but fork-held). Open forks: 18. Friction: 1
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: no
