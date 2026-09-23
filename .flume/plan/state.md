# Plan state

- Spec derived through: fcf719f4 — unchanged, copied forward; the spec delta
  is drained.
- Audited through: 9993fc0d — unchanged, copied forward; the window past it
  (31f7cc81, 4e03d810, 5a15926d) touches no `src/`, `tests/`, or `sdk/` path.
- Residue swept through: 9993fc0d — unchanged, copied forward; same empty
  window.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577; the list is re-carried here from
  e78223af, which ab4a3ead compressed away): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs (this tick); open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: posture sweep — tests/tap.rs neighborhood; one finding filed
  (TAP-RECORD-FIXTURE-ONE-HOME), both gate-revert bills discharged unscoped.
- Queue: 28 pending — 1 open, 19 blockedBy, 3 deferred, 5 parked. Pickable: 1.
  Open forks: 12. Friction: 0. Amendments: 0. Refactor: 0. Inbox: 1 (on trunk
  at 5a15926d, landed after this worktree branched — next tick's job 1).

Plan continues: yes — the trunk inbox note is an undrained queue-shaping input
this worktree could not reach; the open posture rotation resumes behind it.
