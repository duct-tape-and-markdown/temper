# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 4e3344e9 — the three-fold telemetry chain verified on disk.
- Residue swept through: 4e3344e9 — same window, clean.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`, `src/install.rs`, `src/engine.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/graph.rs`, `src/glob.rs`,
  `src/read.rs`, `src/kind.rs`, `src/builtin.rs`, `src/builtin_kind.rs`,
  `src/builtin_lock.rs`, `src/json_manifest.rs`, `src/extract.rs`,
  `src/frontmatter.rs`, `src/toml_document.rs`, `src/gate.rs`,
  `src/check.rs`, `src/coverage.rs`, `src/coverage_note.rs`, `src/dial.rs`,
  `src/display.rs`, `src/admissibility.rs`, `src/telemetry.rs`, `src/tap.rs`,
  `src/path.rs`, `src/address.rs`, `src/member_address.rs`,
  `sdk/src/emit.ts`, `sdk/src/declarations.ts`, `sdk/src/prose.ts`,
  `sdk/src/needs.ts`, `sdk/src/assembly.ts`, `sdk/src/builtins.ts`,
  `sdk/src/claude-code.ts`.
- This tick: swept the addressing-leaves neighborhood (`path`, `address`,
  `member_address`) — two folds filed, `member_address` clean.
- Queue: 10 pending — 2 open, 4 parked, 4 deferred. Pickable: 2. Open
  forks: 1. Friction: 3. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture rotation stays open with two
pickable entries, so the wave ships first and the sweep takes the next
frontier neighborhood when it hands back.
