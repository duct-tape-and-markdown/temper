# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: f1f627a8 — copied forward; the window past it is plan commits
  only, no `src/`/`tests/`/`sdk/` work.
- Residue swept through: f1f627a8 — copied forward, same empty window.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`. `src/compose.rs` next.
- This tick: posture sweep covers `src/contract.rs` — two entries filed, the
  shape-regex table's partial copy and `extent`'s unauthorable whole grain.
- Queue: 9 pending — 1 open, 0 blockedBy, 4 parked, 4 deferred. Pickable: 1.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — SHAPE-REGEX-TABLE-ANSWERS-EVERY-SHAPE is
pickable; the posture sweep resumes at `src/compose.rs` when the wave hands
back.
