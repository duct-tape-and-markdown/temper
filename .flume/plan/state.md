# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 80c2b0ea — the in-src fold verified on disk this tick.
- Residue swept through: 80c2b0ea — swept with the audit motion, same window.
- Posture swept through: src/install.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs; open —
  src/install.rs, tests/it/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 0939127c..80c2b0ea — the clause-fixture fold chain
  closed; four stale gates re-tested on disk, all still true.
- Queue: 8 pending — 1 open, 3 deferred, 4 parked. Pickable: 0 — every
  entry rests on a human ruling or an open fork. Open forks: 18.
  Friction: 1 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — posture sweep at src/install.rs, the only live input
