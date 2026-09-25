# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: cad9ccd0 — one build commit verified on disk (5cc2c1fc);
  every gate reason re-tested.
- Residue swept through: cad9ccd0 — the window's one changed file re-read
  against `specs/builtins.md`; nothing filed.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 715f55ae..HEAD (audit + sweep) — SKILL-PATHS shipped
  as scoped, no new gap, queue unchanged.
- Queue: 9 pending — 1 open, 0 blocked, 3 deferred, 5 parked. Pickable: 0
  (INSTALL-LIFTS is open but fork-held). Open forks: 18. Friction: 1 (human
  channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the posture rotation resumes at sdk/src/builtins.ts,
and no pickable entry stands ahead of it
