# Plan state

- Spec derived through: 862f0b60 — routed this tick: a two-site path
  correction in `engineering.md`, no intent change, nothing derivable.
- Audited through: 70afd6e0 — copied forward: no `src/`, `tests/` or `sdk/`
  commit past it.
- Residue swept through: 70afd6e0 — copied forward with the audit cursor.
- Posture swept through: sdk/src/declarations.ts next — mid-rotation.
  Frozen frontier (armed at 7d695577), paths re-spelled for a7c8c9b5's fold:
  covered — src/read.rs, src/telemetry.rs, tests/it/read_verbs.rs,
  src/admissibility.rs, src/gate.rs, src/graph.rs, tests/it/graph.rs,
  src/tap.rs, tests/it/tap.rs, tests/it/hook_kind.rs, sdk/src/builtins.ts;
  open — sdk/src/declarations.ts, tests/it/emit.rs, src/install.rs,
  tests/it/install.rs, src/compose.rs, src/drift.rs, src/glob.rs,
  src/placement.rs.
- This tick: routed the 862f0b60 spec delta — pure path rot, no entry, no
  fork; both corrected paths verified on disk.
- Queue: 8 pending — 1 open, 3 deferred, 4 parked. Pickable: 0 (the one
  open entry rests on `(hook-member-identity)`). Open forks: 18. Friction:
  1 (human channel, premise falsified at HEAD). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: yes — the posture sweep at sdk/src/declarations.ts, with no
pickable entry for build to take
