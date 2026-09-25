# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 53c9f544 — the window's three code commits verified on
  disk; no entry drops.
- Residue swept through: bab35c14 — unchanged, copied forward (sweep held);
  the window's two stated deferrals wait for it.
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: audited 0c56cf0a..HEAD — GUARD-SHELL-EDGE rescoped over its
  shipped parent, one friction capture for the two unlanded follow-ups.
- Queue: 12 pending — 4 open, 0 blocked, 3 deferred, 5 parked. Pickable: 3
  (disjoint; INSTALL-LIFTS is open but fork-held). Open forks: 18. Friction: 2
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: no — audit at HEAD, delta and inbox empty, both sweeps held
