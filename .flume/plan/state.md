# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: 30f55b49 — the window's two `build:` commits verified on
  disk (833b9a9d, fc7848b7); both shipped entries already off the queue.
- Residue swept through: 30f55b49 — same window, one ride and one ripple
  filed; no standalone residue entry.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled dfa667d7..HEAD — three entries re-cited past the
  window's line shifts, one real ripple folded, the stale `clause_label`
  sentence routed to its ride.
- Queue: 17 pending — 5 open, 4 blockedBy, 3 deferred, 5 parked. Pickable: 4
  (disjoint). Open forks: 15. Friction: 1 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: after-build — the only live input left is the posture
rotation (sdk/src/builtins.ts next), and four pickable entries ship first.
