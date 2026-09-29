# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 80c2b0ea — copied forward: HEAD moved only by the plan commit.
- Residue swept through: 80c2b0ea — copied forward with the audit cursor.
- Posture swept through: tests/it/install.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs; open — tests/it/install.rs,
  src/compose.rs, src/drift.rs, src/glob.rs.
- This tick: swept src/install.rs with src/placement.rs — filed three folds
  over the guard's in-band message surface and the banner round-trip test.
- Queue: 11 pending — 3 open, 1 blockedBy, 3 deferred, 4 parked. Pickable: 2
  — GUARD-BINDING-LIMIT-FOLDS-INTO-ONE-HOME and
  BANNER-ROUND-TRIP-TEST-READS-THE-WRITERS-CONSTANT, disjoint. Open forks: 18.
  Friction: 1 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture rotation resumes at tests/it/install.rs
