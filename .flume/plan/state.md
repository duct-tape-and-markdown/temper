# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: df1012f8 — the backing-set count pin, verified on disk and green.
- Residue swept through: df1012f8 — same window; one residue filed.
- Posture swept through: src/drift.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs, src/compose.rs;
  open — src/drift.rs, src/glob.rs.
- This tick: reconciled abb9750f..df1012f8 — the pin holds, and the same class
  has one live instance left: the guard's per-tool-call locus walk.
- Queue: 9 pending — 2 open, 0 blockedBy, 3 deferred, 4 parked. Pickable: 1 —
  GUARD-LOCUS-WALK-IS-PINNED-BY-COUNT rests on no fork; INSTALL-LIFTS stays
  fork-held on `(hook-member-identity)`. Open forks: 19. Friction: 2
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep at src/drift.rs, once the wave hands back
