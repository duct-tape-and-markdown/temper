# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 6d4ecaa3 — copied forward; the three-build window past it
  is unreconciled and is next tick's job.
- Residue swept through: 6d4ecaa3 — copied forward; same window.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`.
  `src/contract.rs` next.
- This tick: drain the refactor channel — the closed-keys two-homes capture
  died at fcd3813d; deleted, no entry filed.
- Queue: 7 pending — 0 open, 0 blockedBy, 4 parked, 3 deferred. Pickable: 0.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — reconcile 6d4ecaa3..HEAD (audit + residue sweep over the
three shipped `when`/guard builds).
