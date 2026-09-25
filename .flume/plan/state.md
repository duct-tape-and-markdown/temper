# Plan state

- Spec derived through: 11d9efac — both delta commits routed: 0060 into two
  serialized guard entries, 0061 into one, 0062 folded onto the tap entry.
- Audited through: bab35c14 — unchanged, copied forward, and NOT clean: five
  `build:` and two `release:` commits past it touch `src/`/`tests/`/`sdk/`.
- Residue swept through: bab35c14 — unchanged, copied forward (sweep held).
- Posture swept through: sdk/src/builtins.ts next — mid-rotation, unchanged.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs, tests/hook_kind.rs;
  open — sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs,
  src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs,
  src/glob.rs, src/placement.rs.
- This tick: routed the spec delta — 0060/0061/0062 into 3 new entries, 2
  rewrites, and one settled fork deleted.
- Queue: 15 pending — 4 open, 3 blocked, 3 deferred, 5 parked. Pickable: 3
  (disjoint; INSTALL-LIFTS is open but fork-held). Open forks: 18. Friction: 1
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the post-ship audit of bab35c14..HEAD, seven commits
the audit cursor never covered.
