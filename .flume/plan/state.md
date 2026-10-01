# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: dcec6f98 — the three-entry SDK wave verified on disk.
- Residue swept through: dcec6f98 — same window, one entry filed.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/graph.rs`, `src/glob.rs`,
  `src/read.rs`, `src/kind.rs`, `sdk/src/emit.ts`, `sdk/src/declarations.ts`,
  `sdk/src/prose.ts`, `sdk/src/needs.ts`, `sdk/src/assembly.ts`.
- This tick: reconciled 8bf743df..dcec6f98 — all three entries shipped as
  written, every gate re-tested, one sweep entry filed off the read half of
  the `when` seam (a self-referential guard aborts `check`).
- Queue: 9 pending — 4 parked, 4 deferred, 1 open. Pickable: 1. Open forks: 1.
  Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the rotation stays open with the rest of the
domain unswept, and the filed entry is pickable now.
