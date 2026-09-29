# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: 0e602ab0 — the parse fold verified on disk, suite green (788).
- Residue swept through: 0e602ab0 — same window; one residue entry filed.
- Posture swept through: src/drift.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs, src/compose.rs;
  open — src/drift.rs, src/glob.rs.
- This tick: reconciled 380a926a..0e602ab0 — the parse fold holds, and the
  seam it reshaped carries five `pub`s no consumer earns.
- Queue: 9 pending — 2 open, 3 deferred, 4 parked. Pickable: 1 —
  COMPOSE-DRIFT-ZERO-CONSUMER-VISIBILITY-NARROW rests on no fork;
  INSTALL-LIFTS stays fork-held on `(hook-member-identity)`. Open forks: 19.
  Friction: 2 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep at src/drift.rs, once the wave hands back
