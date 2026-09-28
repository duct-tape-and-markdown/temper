# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 7e4d7849 — the node-species fold verified on disk.
- Residue swept through: 7e4d7849 — swept with the audit cursor.
- Posture swept through: tests/it/emit.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts; open — tests/it/emit.rs,
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 71b89c5e..7e4d7849 — the species fold shipped as
  scoped; one consumer still names a dangling leaf a member, one entry.
- Queue: 9 pending — 2 open, 3 deferred, 4 parked. Pickable: 1
  (MENTION-DANGLING-LEAF-NAMES-THE-LEAF-GRAIN; the other open entry rests on
  `(hook-member-identity)`). Open forks: 18. Friction: 1 (human channel,
  premise falsified at HEAD). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep resumes at tests/it/emit.rs
once the wave hands back
