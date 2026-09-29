# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 2f95f0a1 — the lock-fixture fold shipped, verified on disk.
- Residue swept through: 2f95f0a1 — swept with the audit, same window.
- Posture swept through: src/compose.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs; open —
  src/compose.rs, src/drift.rs, src/glob.rs.
- This tick: reconciled 28f74ca7..2f95f0a1 — `common::GuardLock` verified as
  the guard suites' one lock home, and the class's next two slices filed.
- Queue: 10 pending — 2 open, 1 blockedBy, 3 deferred, 4 parked. Pickable: 1
  — INSTALL-LIFTS is fork-held on `(hook-member-identity)` and the
  session-start slice waits on the cli.rs one. Open forks: 18. Friction: 1
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the clause-append fold ships, then the
rotation resumes at src/compose.rs
