# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: abb9750f — both ships verified on disk, suite re-run green.
- Residue swept through: abb9750f — same window; no residue filed.
- Posture swept through: src/compose.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs; open —
  src/compose.rs, src/drift.rs, src/glob.rs.
- This tick: reconciled b04aef62..abb9750f — both ships hold, the hand-lock
  class is empty, and one friction capture filed on a worktree test trap.
- Queue: 8 pending — 1 open, 0 blockedBy, 3 deferred, 4 parked. Pickable: 0 —
  INSTALL-LIFTS is the one open entry and stays fork-held on
  `(hook-member-identity)`. Open forks: 18. Friction: 2 (human channel).
  Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the posture sweep at src/compose.rs, with no pickable entry
