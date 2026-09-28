# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 80dbd411 — both folds verified on disk: the include row
  reads through `drift::includes`, the eleven helpers have one home.
- Residue swept through: 80dbd411 — swept with the audit cursor; a body
  comparison found six copies the name-keyed census missed.
- Posture swept through: src/install.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs; open —
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled c4f82c5b..80dbd411 — both folds shipped, and the
  fixture fold's own residue filed as a second pass.
- Queue: 9 pending — 2 open, 3 deferred, 4 parked. Pickable: 1 (the renamed
  fixture copies; INSTALL-LIFTS-A-REGISTRATION-MEMBER still rests on
  `(hook-member-identity)`, all four parks re-tested true on disk). Open
  forks: 18. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the posture sweep resumes at src/install.rs
once the wave hands back
