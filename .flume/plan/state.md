# Plan state

- Spec derived through: fcf719f4 — unchanged, copied forward; the spec delta
  is drained.
- Audited through: 9993fc0d — unchanged, copied forward; the window past it
  is no longer empty (ten `build:`/`chore(flume):` commits touching `src/`,
  `tests/`, `sdk/`), so reconciliation is the next tick's job.
- Residue swept through: 9993fc0d — unchanged, copied forward; same window.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: refactor — drained the fence capture into SETTINGS-KIND-SHIPS(builtin)
  (three cases re-verified on disk, two files added), capture deleted.
- Queue: 20 pending — 1 open, 11 blockedBy, 3 deferred, 5 parked. Pickable: 1.
  Open forks: 12. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: yes — post-ship reconciliation over 9993fc0d..HEAD, the audit
and residue window this tick's inbox job pre-empted.
