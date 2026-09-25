# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 94344db6 — cad9ccd0..HEAD reconciled this tick.
- Residue swept through: 94344db6 — same window, swept with the audit.
- Posture swept through: sdk/src/declarations.ts next — mid-rotation.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs,
  sdk/src/builtins.ts; open — sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled cad9ccd0..HEAD — the widened hook event set shipped
  as claimed, its one sibling allowlist walked and clean, nothing filed.
- Queue: 11 pending — 2 open, 1 blocked, 3 deferred, 5 parked. Pickable: 1
  (LOCK-ROUND-TRIP-PIN; INSTALL-LIFTS is open but fork-held). Open forks: 18.
  Friction: 1 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep at sdk/src/declarations.ts,
with LOCK-ROUND-TRIP-PIN-READS-THE-GUARD-FIELD pickable now
