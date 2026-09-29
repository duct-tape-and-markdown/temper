# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 28f74ca7 — the two folds shipped, verified on disk.
- Residue swept through: 28f74ca7 — swept with the audit, same window.
- Posture swept through: src/compose.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs; open —
  src/compose.rs, src/drift.rs, src/glob.rs.
- This tick: reconciled 1f6478a2..28f74ca7 — both folds verified, and the
  guard fixtures' third half (the lock) filed as the window's residue.
- Queue: 9 pending — 2 open, 3 deferred, 4 parked. Pickable: 1 — the other
  open entry is fork-held on `(hook-member-identity)`. Open forks: 18.
  Friction: 1 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the lock-fixture fold ships, then the rotation
resumes at src/compose.rs
