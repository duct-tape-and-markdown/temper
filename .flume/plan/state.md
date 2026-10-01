# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 9f812edd — copied forward; the window past it is plan-only.
- Residue swept through: 9f812edd — copied forward with the audit cursor.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`. `src/engine.rs` next.
- This tick: posture sweep over `src/install.rs` — the guard's payload read
  filed as one fold entry; the module's cohesion observed, not filed.
- Queue: 8 pending — 1 open, 4 parked, 3 deferred. Pickable: 1. Open forks: 1.
  Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — GUARD-DECODES-ITS-PAYLOAD-ONCE is pickable, so
the wave ships it first; the posture rotation resumes on `src/engine.rs` when
build hands back.
