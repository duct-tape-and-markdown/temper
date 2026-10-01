# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: b253d418 — both graph.rs cost-hoist folds verified on disk,
  their entries already removed by the ship commits.
- Residue swept through: b253d418 — same window, no residue.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/graph.rs`, `src/glob.rs`.
- This tick: reconciled 3167c3db..b253d418 — both folds and both re-taken count
  pins verified green on disk; all 8 remaining gates re-tested, none moved.
- Queue: 8 pending — 4 parked, 4 deferred. Pickable: 0. Open forks: 1.
  Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the posture sweep is mid-rotation with no pickable entry
behind it, so plan drives the next frontier module itself.
