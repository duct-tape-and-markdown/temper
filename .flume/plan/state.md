# Plan state

- Spec derived through: fcf719f4 — unchanged, copied forward; the spec delta
  is drained.
- Audited through: fbbe2c6d — advanced; 9993fc0d..HEAD reconciled, eight
  shipped entries verified on disk.
- Residue swept through: fbbe2c6d — advanced; same window, one gap filed and
  one fork registered.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: post-ship reconciliation over 9993fc0d..HEAD — filed
  WHEN-BODY-GUIDANCE-REACHES-ITS-FINDING, registered
  `(guard-body-address-and-severity)`, both cursors advanced.
- Queue: 21 pending — 2 open, 11 blockedBy, 3 deferred, 5 parked. Pickable: 2.
  Open forks: 13. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the posture sweep resumes at tests/hook_kind.rs
once the two pickable entries ship.
