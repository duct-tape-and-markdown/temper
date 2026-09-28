# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 70afd6e0 — copied forward: no `src/`, `tests/` or `sdk/`
  commit past it.
- Residue swept through: 70afd6e0 — copied forward with the audit cursor.
- Posture swept through: tests/it/emit.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts; open — tests/it/emit.rs,
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: swept the sdk/src/declarations.ts neighborhood — filed the
  one-home fold for `placementKey`, routed its third encoder to
  `(hook-member-identity)` with the SDK-synthesis evidence.
- Queue: 9 pending — 2 open, 3 deferred, 4 parked. Pickable: 1
  (PLACEMENT-KEY-FOLDS-ONTO-THE-ADDRESS-GRAMMAR; the other open entry rests
  on `(hook-member-identity)`). Open forks: 18. Friction: 1 (human channel,
  premise falsified at HEAD). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep resumes at tests/it/emit.rs
once the wave hands back
