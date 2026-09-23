# Plan state

- Spec derived through: fcf719f4 — unchanged, copied forward; the spec delta
  is drained.
- Audited through: 9993fc0d — unchanged, copied forward; the window past it
  (31f7cc81, 4e03d810, 5a15926d, b229ffab) touches no `src/`, `tests/`, or
  `sdk/` path.
- Residue swept through: 9993fc0d — unchanged, copied forward; same empty
  window.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577; the list is re-carried here from
  e78223af, which ab4a3ead compressed away): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: inbox — drained the trunk residue note; the entry it rules on is
  re-verified pre-fix on disk and stands unchanged, no queue field moved.
- Queue: 28 pending — 1 open, 19 blockedBy, 3 deferred, 5 parked. Pickable: 1.
  Open forks: 12. Friction: 0. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — every queue-shaping input is drained and
EXAMPLE-EMIT-GATE-BUILDS-THE-SDK-IT-READS is pickable; the open posture
rotation (tests/hook_kind.rs) resumes when the wave hands back.
