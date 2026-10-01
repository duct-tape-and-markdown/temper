# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 9ce49208 — all three read.rs chain findings verified on
  disk, their entries already removed by the ship commits.
- Residue swept through: 9ce49208 — same window, no residue.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/graph.rs`, `src/glob.rs`,
  `src/read.rs`, `src/kind.rs`.
- This tick: posture sweep covered `src/kind.rs` — one finding filed, the
  `<name>(<args>)` label grammar split three times in one file.
- Queue: 9 pending — 1 open, 4 parked, 4 deferred. Pickable: 1.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the rotation is open with the rest of the domain
unswept, and the filed entry is pickable now, so the wave ships first and the
sweep resumes when it hands back.
