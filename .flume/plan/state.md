# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: eabe7d46 — copied forward; no `src/`/`tests/`/`sdk/` commit since.
- Residue swept through: eabe7d46 — copied forward, same window.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`. `src/graph.rs` next.
- This tick: posture sweep covers compose.rs — two entries filed, the read's
  vestigial kind clone and the directive-member two-loop build.
- Queue: 10 pending — 1 open, 1 blockedBy, 4 parked, 4 deferred. Pickable: 1.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — one pickable entry ships first; the posture
sweep takes `src/graph.rs` when the wave hands back.
