# Plan state

- Spec derived through: 069811e6 — unchanged, copied forward; the delta past
  it is empty.
- Audited through: b89a4372 — the six ship commits in 02608819..HEAD verified
  on disk; all six entries were already dropped by their `chore(flume):` ships.
- Residue swept through: b89a4372 — same window; the splice machinery
  842884f4 retired is gone from `src/` and `tests/`, no second writer left.
- Posture swept through: tests/hook_kind.rs next — mid-rotation continues.
  Frozen frontier (armed at 7d695577): covered — src/read.rs,
  src/telemetry.rs, tests/read_verbs.rs, src/admissibility.rs, src/gate.rs,
  src/graph.rs, tests/graph.rs, src/tap.rs, tests/tap.rs; open —
  tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts,
  tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs,
  src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: reconciled 02608819..HEAD — install.rs's rewrite re-cited into
  INSTALL-LIFTS-A-REGISTRATION-MEMBER, build's own deferral registered as
  `(represented-harness-gate-upgrade)`, `(post-tool-use-placement)` re-measured.
- Queue: 15 pending — 3 open, 4 blockedBy, 3 deferred, 5 parked. Pickable: 2.
  Open forks: 15. Friction: 1 (human channel). Amendments: 0. Refactor: 0.
  Inbox: 0.

Plan continues: after-build — the posture rotation resumes at
tests/hook_kind.rs once the two pickable entries ship.
