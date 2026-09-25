# Plan state

- Spec derived through: aa389f08 — unchanged, copied forward; the inbox was
  the live input this tick, so 0060/0061/0062 stay unrouted.
- Audited through: bab35c14 — unchanged, copied forward; no commit past it
  touches `src/`, `tests/` or `sdk/`.
- Residue swept through: bab35c14 — unchanged, copied forward with the audit.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: drained the inbox — 7 notes routed into 3 entries, 4 new forks,
  one amended fork record, and one taste item taken as debt.
- Queue: 12 pending — 4 open, 3 deferred, 5 parked. Pickable: 3 (disjoint;
  INSTALL-LIFTS is open but fork-held). Open forks: 19. Friction: 1
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the spec delta, three unrouted decisions (0060, 0061,
0062) past the cursor.
