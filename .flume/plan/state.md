# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 6d4ecaa3 — nothing past it touches `src/`, `tests/`, `sdk/`.
- Residue swept through: 6d4ecaa3 — copied forward; the window past it is empty.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`.
  `src/contract.rs` next.
- This tick: posture sweep covers engine.rs — the `when` body's element
  projection decides over features it zeroes; three entries filed.
- Queue: 10 pending — 1 open, 2 blockedBy, 4 parked, 3 deferred. Pickable: 1.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — ready work ships first; the rotation resumes on
`src/contract.rs` when the wave hands back.
