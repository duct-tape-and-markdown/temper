# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 3fc615ef — the renamed-copy fold shipped and is verified on
  disk this tick.
- Residue swept through: 3fc615ef — swept with the audit motion, same window.
- Posture swept through: src/install.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs; open —
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 80dbd411..3fc615ef — the fold holds on disk, and the
  one-clause judge it left spelled per file became ONE-CLAUSE-JUDGE-CALLS-FOLD-INTO-ONE-HOME.
- Queue: 10 pending — 2 open, 1 blocked, 3 deferred, 4 parked. Pickable: 1
  (the floor fold; the new fold is serialized behind it,
  INSTALL-LIFTS-A-REGISTRATION-MEMBER still rests on `(hook-member-identity)`).
  Open forks: 18. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the posture rotation resumes at src/install.rs
