# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: 02608819 — advanced: db387262..HEAD reconciled, one code
  commit (24a2145a, the tap fixture's move into tests/common) verified on
  disk; nothing dropped, every gate re-tested and still true.
- Residue swept through: 02608819 — advanced: the same window, code against
  corpus, is clean — the one commit discharges a duplicate surface rather
  than leaving one.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: reconciled db387262..HEAD — both motions clean, no entry filed,
  dropped or re-gated; both cursors advanced.
- Queue: 21 pending — 5 open, 8 blockedBy, 3 deferred, 5 parked. Pickable: 5.
  Open forks: 14. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the posture rotation (tests/hook_kind.rs next)
is the only live job left and five pickable entries ship first.
