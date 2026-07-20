# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: a54f3c3 — window bc41586..a54f3c3 (2ebf5ac touching sdk/src/builtins.ts) audited on disk: `SETTINGS_MANIFEST` now used at all 6 sites (399, 403, 486, 490, 561, 565); zero raw `"settings.json"` code literals remain in the file (prose cites at 366/391-392/456/477-478/507/522/551-552/1182/1256 untouched, correctly). `pnpm --dir sdk test` green, 137/137.
- Residue swept through: a54f3c3 — same window, no new residue. Cross-language `"settings.json"` literals surviving in `src/builtin_lock.toml`, `src/builtin_kind.rs`, `src/install.rs`, `src/coverage_note.rs`, `src/drift.rs` predate this window (untouched by 2ebf5ac) — out of this tick's scope, not filed.
- Posture swept through: full src/ list covered; sdk/src/assembly.ts, sdk/src/builtins.ts, and sdk/src/claude-code.ts covered too — mid-rotation. Phrase delta 8911c38 still arms the whole sweep domain (src/, sdk/src/, tests/); rest of sdk/src/ (contract.ts onward, tree order) + tests/ frontier remains; sdk/src/contract.ts is the tree-order candidate next.
- This tick: POSTURE SWEEP, neighborhood sdk/src/claude-code.ts (read whole, 59 lines) plus immediate imports builtins.ts/prose.ts (export-surface check). Filed BUILTINS-SETTINGS-MANIFEST-EXPORT-PRUNE (zero-consumer export), open/pickable; no other violation in the neighborhood.
- Queue: 3 pending, 1 open (BUILTINS-SETTINGS-MANIFEST-EXPORT-PRUNE), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unaffected, re-checked, still hold). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — a pickable entry now exists (BUILTINS-SETTINGS-MANIFEST-EXPORT-PRUNE); the posture rotation stays open behind it (sdk/src/contract.ts next neighborhood) and resumes once the wave hands back.
