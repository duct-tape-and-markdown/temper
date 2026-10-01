# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: b253d418 — both graph.rs cost-hoist folds verified on disk,
  their entries already removed by the ship commits.
- Residue swept through: b253d418 — same window, no residue.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/graph.rs`, `src/glob.rs`,
  `src/read.rs`.
- This tick: posture sweep covered `src/read.rs` — three findings filed as a
  serialized chain (the satisfier set's second selector, the duplicate grain
  classifier, the count pin's unearned `pub`).
- Queue: 11 pending — 4 parked, 4 deferred, 1 open, 2 blockedBy. Pickable: 1.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the rotation is open with the rest of the domain
unswept, and the read.rs chain's head is pickable now, so ready work ships
first and the sweep resumes when the wave hands back.
