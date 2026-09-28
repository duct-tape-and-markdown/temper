# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: c4f82c5b — the leaf-grain dangling verdict verified on disk.
- Residue swept through: c4f82c5b — swept with the audit cursor, clean.
- Posture swept through: src/install.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs; open —
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: swept tests/it/emit.rs — two duplicate-surface findings filed,
  a hand-walked lock read and the suite's 28 copied fixture definitions.
- Queue: 10 pending — 2 open, 1 blockedBy, 3 deferred, 4 parked. Pickable: 1
  (the include-row read; INSTALL-LIFTS-A-REGISTRATION-MEMBER still rests on
  `(hook-member-identity)`, all four parks still true). Open forks: 18.
  Friction: 1 (human channel, premise falsified at HEAD). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep resumes at src/install.rs
once the wave hands back
