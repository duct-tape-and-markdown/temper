# Plan state

- Spec derived through: aa389f08 — advanced; 0059 routed, all four
  Consequences bullets accounted for in this tick's commit body.
- Audited through: bab35c14 — unchanged, copied forward; no commit past it
  touches `src/`, `tests/` or `sdk/`.
- Residue swept through: bab35c14 — unchanged, copied forward with the audit.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: derived 0059 — one open entry filed, housed in tests/cli.rs to
  stay disjoint, and the `(guard-locus-binding-clause-gated)` record deleted.
- Queue: 14 pending — 5 open, 1 blockedBy, 3 deferred, 5 parked. Pickable: 4
  (disjoint). Open forks: 15. Friction: 1 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep, mid-rotation at
sdk/src/builtins.ts, resumes once the wave hands back.
