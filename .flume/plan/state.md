# Plan state

- Spec derived through: 13455d4a — unchanged, copied forward (not this tick's job).
- Audited through: 09a79fef — advanced from 9b967326.
- Residue swept through: 09a79fef — advanced from 9b967326.
- Posture swept through: 22f8064c — unchanged, mid-rotation continues. This tick's window (22f8064c..HEAD, code delta) frontier unchanged from last tick's derivation: src/read.rs (covered), src/telemetry.rs (covered), tests/read_verbs.rs (covered), src/tap.rs, tests/tap.rs, src/admissibility.rs (covered), src/gate.rs (covered), tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs, src/graph.rs, tests/graph.rs, src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: post-ship reconciliation over 9b967326..09a79fef (b38e6df0, 94cad5f1, 09a79fef) — audited all three on disk against their entries (clean, no residue), dropped COVERAGE-EMBEDDED-COUNT-MARKER (shipped, verified) from pending; full detail in commit body.
- Queue: 3 pending — 0 open, 1 parked, 2 deferred. Open forks: 3, unchanged. Friction: 0. Amendments: 0. Inbox: 0. Refactor: 0.

Plan continues: yes — the posture sweep is now the only live input (0 pickable open entries), so plan drives the next neighborhood itself rather than waiting on a build wave.
