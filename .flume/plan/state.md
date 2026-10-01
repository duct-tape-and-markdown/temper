# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: eabe7d46 — one build commit (bb38e8a4) verified on disk.
- Residue swept through: eabe7d46 — same window, no residue.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`. `src/compose.rs` next.
- This tick: reconcile f1f627a8..eabe7d46 — the shape-regex match verified,
  every queue gate re-tested, no residue.
- Queue: 8 pending — 0 open, 0 blockedBy, 4 parked, 4 deferred. Pickable: 0.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — no entry is pickable, so the posture sweep takes
`src/compose.rs` next tick.
