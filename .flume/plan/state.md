# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: c4f82c5b — the leaf-grain dangling verdict verified on disk.
- Residue swept through: c4f82c5b — swept with the audit cursor, clean.
- Posture swept through: tests/it/emit.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts; open — tests/it/emit.rs,
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 7e4d7849..c4f82c5b — the leaf-grain verdict shipped
  as scoped, the species fold closes, no residue and no new entry.
- Queue: 8 pending — 1 open, 3 deferred, 4 parked. Pickable: 0 (the one open
  entry rests on `(hook-member-identity)`; all four parks re-tested, all
  still true). Open forks: 18. Friction: 1 (human channel, premise falsified
  at HEAD). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the posture sweep at tests/it/emit.rs, with no
pickable entry for build to take
