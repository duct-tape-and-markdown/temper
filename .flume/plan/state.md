# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: f1f627a8 — the three-build `when`/guard window verified on
  disk; no entry's work shipped inside it.
- Residue swept through: f1f627a8 — same window, nothing fileable.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`.
  `src/contract.rs` next.
- This tick: reconcile 6d4ecaa3..HEAD — three builds verified, seven gates
  re-tested, no entry filed or dropped.
- Queue: 7 pending — 0 open, 0 blockedBy, 4 parked, 3 deferred. Pickable: 0.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — posture sweep resumes at `src/contract.rs` (open
rotation, no pickable entries).
