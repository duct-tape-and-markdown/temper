# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 0939127c — both folds verified on disk this tick.
- Residue swept through: 0939127c — swept with the audit motion, same window.
- Posture swept through: src/install.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs; open —
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 8d0d2fd2..0939127c — the in-src crate holds four
  more clause-fixture spellings, two addressing what no lock stamps.
- Queue: 9 pending — 2 open, 3 deferred, 4 parked. Pickable: 1 (the new
  in-src fold; INSTALL-LIFTS-A-REGISTRATION-MEMBER still rests on
  `(hook-member-identity)`). Open forks: 18. Friction: 1 (human channel).
  Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture rotation resumes at src/install.rs
