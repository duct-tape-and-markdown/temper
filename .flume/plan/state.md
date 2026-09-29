# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 1f6478a2 — copied forward: no `src/`, `tests/` or `sdk/`
  commit past it.
- Residue swept through: 1f6478a2 — copied forward with the audit cursor.
- Posture swept through: src/compose.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs; open —
  src/compose.rs, src/drift.rs, src/glob.rs.
- This tick: swept tests/it/install.rs — filed two folds (the writers'
  constants hand-spelled 31 times; one guard-payload builder spelled 29 ways).
- Queue: 10 pending — 2 open, 1 blockedBy, 3 deferred, 4 parked. Pickable: 1
  — the other open entry is fork-held on `(hook-member-identity)`. Open
  forks: 18. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the pins fold ships, then the rotation resumes
at src/compose.rs
