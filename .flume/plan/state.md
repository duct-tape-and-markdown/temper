# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 8bf743df — `call_label` verified on disk as the tree's one
  reader of the lock's call syntax; the entry's file was already removed.
- Residue swept through: 8bf743df — same one-commit window, no residue.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/graph.rs`, `src/glob.rs`,
  `src/read.rs`, `src/kind.rs`.
- This tick: reconciled 9ce49208..8bf743df — the kind.rs label fold verified,
  no residue, every pending gate re-tested and still true.
- Queue: 8 pending — 4 parked, 4 deferred. Pickable: 0.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the rotation is open with the rest of the domain unswept
and nothing pickable, so plan drives the next neighborhood itself.
