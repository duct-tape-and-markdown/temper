# Plan state

- Spec derived through: fcf719f4 — unchanged, copied forward; the spec delta
  is drained.
- Audited through: db387262 — unchanged, copied forward; the inbox job took
  this tick; the window past it is db387262..HEAD, whose one code commit
  (24a2145a, tests/common) is next tick.
- Residue swept through: db387262 — unchanged, same reason.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: drained the build phase's fence capture — re-scoped
  ROOT-DEFAULT-CONTRACT-SHIPS with the two tests that assert the negation of
  the shipped default (both verified on disk at HEAD) plus the second
  gauntlet snapshot, and keyed the capture's flagged consequence as
  `(empty-contract-unspellable)`.
- Queue: 19 pending — 5 open, 6 blockedBy, 3 deferred, 5 parked. Pickable: 5.
  Open forks: 15. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the post-ship window db387262..HEAD is
unreconciled (24a2145a) and the posture rotation is open
(tests/hook_kind.rs next); five pickable entries ship first.
