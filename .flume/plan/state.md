# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 8d0d2fd2 — the labelled-clause fold is verified on disk this tick.
- Residue swept through: 8d0d2fd2 — swept with the audit motion, same window.
- Posture swept through: src/install.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs; open —
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled ff6e6072..8d0d2fd2 — the fold fixed one crate's
  labeller and left the in-src twin plus the Selection host unfolded.
- Queue: 10 pending — 3 open, 3 deferred, 4 parked. Pickable: 2 (both new
  folds; INSTALL-LIFTS-A-REGISTRATION-MEMBER still rests on
  `(hook-member-identity)`). Open forks: 18. Friction: 1 (human channel).
  Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture rotation resumes at src/install.rs
