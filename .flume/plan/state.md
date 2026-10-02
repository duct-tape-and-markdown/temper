# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: f20a9248 — the ignore-walk consolidation, one commit.
- Residue swept through: f20a9248 — same window, nothing fileable.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/graph.rs`, `src/glob.rs`,
  `src/read.rs`, `src/kind.rs`, `src/builtin.rs`, `src/builtin_kind.rs`,
  `src/builtin_lock.rs`, `src/json_manifest.rs`, `src/extract.rs`,
  `src/frontmatter.rs`, `src/toml_document.rs`, `src/gate.rs`,
  `src/check.rs`, `src/coverage.rs`, `src/coverage_note.rs`, `src/dial.rs`,
  `src/display.rs`, `sdk/src/emit.ts`, `sdk/src/declarations.ts`,
  `sdk/src/prose.ts`, `sdk/src/needs.ts`, `sdk/src/assembly.ts`,
  `sdk/src/builtins.ts`, `sdk/src/claude-code.ts`.
- This tick: reconciled 5421c983..f20a9248 — IGNORE-WALK-POLICY-HAS-ONE-HOME
  verified shipped, every gate re-tested, no residue.
- Queue: 8 pending — 4 parked, 4 deferred. Pickable: 0. Open forks: 1.
  Friction: 3. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the posture rotation is open with no pickable entry,
so the sweep drives itself (the whole `tests/` tree and ~20 src/sdk modules
are still in the frontier).
