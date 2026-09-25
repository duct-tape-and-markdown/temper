# Plan state

- Spec derived through: 11d9efac — unchanged, copied forward: no `specs/`
  commit past it.
- Audited through: 67841a93 — 94344db6..HEAD reconciled this tick.
- Residue swept through: 67841a93 — same window, swept with the audit.
- Posture swept through: sdk/src/declarations.ts next — mid-rotation.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs,
  sdk/src/builtins.ts; open — sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: reconciled 94344db6..HEAD — both builds shipped as claimed;
  filed the gap 0264b33f surfaced and did not take.
- Queue: 10 pending — 2 open, 0 blocked, 3 deferred, 5 parked. Pickable: 1
  (HOOK-HANDLER-TYPE-REQUIRED-CLAUSE; INSTALL-LIFTS is open but fork-held).
  Open forks: 18. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the posture sweep at sdk/src/declarations.ts,
with HOOK-HANDLER-TYPE-REQUIRED-CLAUSE pickable now
