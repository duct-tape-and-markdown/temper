# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: bc41586 — unchanged, not this tick's job.
- Residue swept through: bc41586 — unchanged, not this tick's job.
- Posture swept through: full src/ list covered; sdk/src/assembly.ts and now sdk/src/builtins.ts covered too — mid-rotation. Phrase delta 8911c38 still arms the whole sweep domain (src/, sdk/src/, tests/); rest of sdk/src/ (claude-code.ts onward, tree order) + tests/ frontier remains; sdk/src/claude-code.ts is the tree-order candidate next.
- This tick: POSTURE SWEEP, neighborhood sdk/src/builtins.ts (read whole, 1662 lines, plus its immediate imports kind.ts/contract.ts/prose.ts for signature shape only). Found: `SETTINGS_MANIFEST` (388, minted by 5adcc4d to hold the one settings.json fact) is used at exactly 1 of 6 sites carrying that literal in this same file — `hook`'s own `locus.glob` (399), `installedPlugin`'s `locus.glob`+`collectionAddress.manifest` (486, 490), and `knownMarketplace`'s same pair (561, 565) still hardcode `"settings.json"`. Filed BUILTINS-SETTINGS-MANIFEST-LITERAL-DEDUP (per engineering.md, "A shared concept is one type"), open/pickable. No other violation found in the neighborhood — union completeness (MarketplaceSource's 4 forms all `when`-gated), export consumption (every `*DefaultContract` re-exported through claude-code.ts), and cohesion (module scope matches its own doc header) all check out on disk.
- Queue: 3 pending, 1 open (BUILTINS-SETTINGS-MANIFEST-LITERAL-DEDUP), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unaffected). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — a pickable entry now exists (BUILTINS-SETTINGS-MANIFEST-LITERAL-DEDUP); the posture rotation stays open behind it (sdk/src/claude-code.ts next neighborhood) and resumes once the wave hands back.
