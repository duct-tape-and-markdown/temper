# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: d4ec7da — window 2a6e488..d4ec7da reconciled.
- Residue swept through: d4ec7da — same window.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs, src/compose.rs, src/contract.rs, src/coverage.rs, src/coverage_note.rs, src/dial.rs, src/display.rs, src/document.rs, src/drift.rs, src/engine.rs, src/extract.rs, src/frontmatter.rs, src/gate.rs, src/glob.rs covered, mid-rotation. src/graph.rs is the tree-order candidate next — unchanged, not this tick's job.
- This tick: POSTURE SWEEP, src/glob.rs (neighborhood = the module alone; foundation vocabulary, no crate-internal imports). Verified on disk: `rg "globset::"` across src/ hits only glob.rs — `compile_glob` is genuinely the crate's one glob-compilation surface, no parallel matcher; its five callers (kind.rs, coverage_note.rs, import.rs, graph.rs, engine.rs's `GlobValid` predicate) all route through it. `glob_compile_count` has a real consumer (tests/check_cost.rs's count-pin). No dead plumbing, no embedded provider knowledge (module is provider-agnostic per its own header), no cohesion violation. Quiet — no entry filed, rotation continues to src/graph.rs.
- Queue: 2 pending, 0 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched this tick). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — no pickable entry exists (both parked), so the open posture rotation (next neighborhood src/graph.rs) is next tick's live job.
