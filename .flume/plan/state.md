# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: e253fd44 — the visibility narrow verified on disk, suite green (788).
- Residue swept through: e253fd44 — same window; no residue entry (the narrow
  satisfies its own posture; its one side effect is a sanctioned advisory).
- Posture swept through: src/drift.rs next — mid-rotation. Frozen
  frontier (armed at 7d695577): covered — src/read.rs, src/telemetry.rs,
  tests/it/read_verbs.rs, src/admissibility.rs, src/gate.rs, src/graph.rs,
  tests/it/graph.rs, src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs,
  sdk/src/builtins.ts, sdk/src/declarations.ts, tests/it/emit.rs,
  src/install.rs, src/placement.rs, tests/it/install.rs, src/compose.rs;
  open — src/drift.rs, src/glob.rs.
- This tick: reconciled 0e602ab0..e253fd44 — the narrow holds, no residue, every gate re-tested.
- Queue: 8 pending — 1 open, 3 deferred, 4 parked. Pickable: 0 — INSTALL-LIFTS
  is the one open entry, fork-held on `(hook-member-identity)`. Open forks: 19.
  Friction: 2 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the posture sweep at src/drift.rs, the queue's only source of new work
