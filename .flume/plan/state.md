# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 79e0079 — unchanged, no new src/tests/sdk commits since.
- Residue swept through: 79e0079 — unchanged, same reason.
- Posture swept through: full src/ list covered; sdk/src/assembly.ts now covered too — mid-rotation. Phrase delta 8911c38 arms the whole sweep domain (src/, sdk/src/, tests/); rest of sdk/src/ (builtins.ts onward, tree order) + tests/ frontier remains; sdk/src/builtins.ts is the tree-order candidate next.
- This tick: POSTURE SWEEP, sdk/src/assembly.ts neighborhood (immediate imports kind.ts/contract.ts read for context, not covered). Finding: the generic `EnforcementMode` doc comment (35-43) names the Claude-Code-specific `PreToolUse` hook — a provider fact leaked outside the provider face (architecture.md, "The provider face is data"); assembly.ts imports no provider-face module, so this is an upward edge invisible to the import graph. Filed ASSEMBLY-ENFORCEMENT-MODE-PROVIDER-FACT-LEAK, gate open.
- Queue: 3 pending, 1 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unaffected, HEAD unchanged since their last re-check). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — the only remaining live job is the posture sweep (sdk/src/ + tests/ frontier still open) and a pickable entry now exists (ASSEMBLY-ENFORCEMENT-MODE-PROVIDER-FACT-LEAK); ready work ships first, sweep resumes after.
