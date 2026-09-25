# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 715f55ae — three build commits verified on disk
  (87fe7447, 00411872, c1d4daa0); every gate reason re-tested.
- Residue swept through: 715f55ae — the hold lifted at 02b89d7d, so the
  window bab35c14..HEAD swept this tick; one gap filed.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 53c9f544..HEAD (audit) and bab35c14..HEAD (sweep) —
  filed SKILL-PATHS-FIELD-DOC-NAMES-THE-FILE-TOOL, re-cut INSTALL-LIFTS.
- Queue: 10 pending — 2 open, 0 blocked, 3 deferred, 5 parked. Pickable: 1
  (INSTALL-LIFTS is open but fork-held). Open forks: 18. Friction: 1 (human
  channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture rotation resumes at
sdk/src/builtins.ts, which the pickable entry rewrites first
