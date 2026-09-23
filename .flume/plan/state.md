# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: dfa667d7 — unchanged; the window past it holds one plan
  commit and no `src/`, `tests/` or `sdk/` change.
- Residue swept through: dfa667d7 — same, copied forward.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: posture sweep of the tests/hook_kind.rs neighborhood — two
  violations verified on disk, filed and serialized.
- Queue: 18 pending — 4 open, 6 blockedBy, 3 deferred, 5 parked. Pickable: 3
  (disjoint). Open forks: 15. Friction: 1 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep's rotation, `sdk/src/builtins.ts`
next on the frozen frontier.
