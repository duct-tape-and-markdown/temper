# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: a54f3c3 — window bc41586..a54f3c3 (2ebf5ac touching sdk/src/builtins.ts) audited on disk: `SETTINGS_MANIFEST` now used at all 6 sites (399, 403, 486, 490, 561, 565); zero raw `"settings.json"` code literals remain in the file (prose cites at 366/391-392/456/477-478/507/522/551-552/1182/1256 untouched, correctly). `pnpm --dir sdk test` green, 137/137.
- Residue swept through: a54f3c3 — same window, no new residue. Cross-language `"settings.json"` literals surviving in `src/builtin_lock.toml`, `src/builtin_kind.rs`, `src/install.rs`, `src/coverage_note.rs`, `src/drift.rs` predate this window (untouched by 2ebf5ac) — out of this tick's scope, not filed.
- Posture swept through: full src/ list covered; sdk/src/assembly.ts and sdk/src/builtins.ts covered too — mid-rotation, unchanged this tick. Phrase delta 8911c38 still arms the whole sweep domain (src/, sdk/src/, tests/); rest of sdk/src/ (claude-code.ts onward, tree order) + tests/ frontier remains; sdk/src/claude-code.ts is the tree-order candidate next.
- This tick: POST-SHIP RECONCILIATION, window bc41586..a54f3c3. BUILTINS-SETTINGS-MANIFEST-LITERAL-DEDUP shipped (2ebf5ac build, a54f3c3 chore) and closed correctly on disk; audit + sweep both clean.
- Queue: 2 pending, 0 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — both re-checked, parked conditions still hold, unaffected by this window). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture rotation is the only live job left and the queue holds no pickable entry (both parked); plan drives the sweep to sdk/src/claude-code.ts next tick.
