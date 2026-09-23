# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: dfa667d7 — the FRESH ship window (81ee251b..HEAD) is
  reconciled: five entries re-cited against the moved line pointers, no
  entry dropped (the shipped one left the queue at d03760fe).
- Residue swept through: dfa667d7 — same window; the retired rule ids are
  gone from `src/`, `tests/` and `sdk/`, and the one structural gap the
  window grew is filed as CLAUSE-FINDING-ONE-CONSTRUCTOR.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: post-ship reconciliation of the FRESH window — both motions.
- Queue: 16 pending — 3 open, 5 blockedBy, 3 deferred, 5 parked. Pickable: 2
  (disjoint). Open forks: 15. Friction: 1 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: yes — the posture sweep's rotation, `tests/hook_kind.rs`
next on the frozen frontier.
