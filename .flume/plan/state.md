# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: abb9750f — copied forward: no `src/`/`tests/`/`sdk/` commit past it.
- Residue swept through: abb9750f — same window; no residue filed.
- Posture swept through: src/drift.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs, src/compose.rs;
  open — src/drift.rs, src/glob.rs.
- This tick: swept src/compose.rs — the directive backing-set walk is the one
  unpinned whole-input hoist; filed its count pin and one fork on its shape.
- Queue: 9 pending — 2 open, 0 blockedBy, 3 deferred, 4 parked. Pickable: 1 —
  DIRECTIVE-BACKING-WALK-IS-PINNED-BY-COUNT rests on no fork; INSTALL-LIFTS
  stays fork-held on `(hook-member-identity)`. Open forks: 19. Friction: 2
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep at src/drift.rs, once the wave hands back
