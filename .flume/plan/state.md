# Plan state

- Spec derived through: b645125 — unchanged, not this tick's job.
- Audited through: fee46ef — advanced from f130ebc. Window f130ebc..fee46ef reconciled: one code-touching commit, 3533e0b (build: align SDK telemetry vocabulary to ratified event names).
- Residue swept through: fee46ef — advanced, same window; no residue found.
- Posture swept through: 77c0790 — unchanged; re-armed for next tick: 3533e0b (in this window) touched sdk/src/contract.ts and sdk/src/declarations.ts, both posture-eligible.
- This tick: POST-SHIP RECONCILIATION. Audited 3533e0b live on disk: sdk/src/contract.ts's telemetry() doc and declarations.ts's TELEMETRY_EVENT_HOOKS now name SkillInvoked/ToolUse, matching src/tap.rs::documented_event_names() (the canonical TapEvent variants) exactly; contract.test.ts/emit.test.ts updated in lockstep. Swept for residue: `rg` across src/tests/sdk/specs finds no remaining telemetry-doc-vocabulary use of the retired Skill/PostToolUse spelling — every surviving "PostToolUse"/"Skill" hit is the real Claude Code wire hook name (matcher values, builtin_lock.toml, .claude/settings.json), a distinct concept, correctly untouched. Metrics glanced (36 turns, clean merge, no revert) — unremarkable sizing. Both parked entries' blockers re-verified untouched by the window: graph.rs/tests/graph.rs (IMPORT-HOP-CAP-CITE) and .github/, Cargo.toml 0.1.0 (PACKAGING-CHANNELS-REMAINDER) — sdk/package.json bumped to 0.0.10 in 9d0868b but that's the SDK's own version, not the crate/tag lockstep the park names.
- Queue: 3 pending, 1 open, 0 blockedBy, 2 parked. Refactor: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: after-build — the only remaining live job is the posture sweep (re-armed above), and one pickable entry exists (BUILTIN-LOCK-ROUND-TRIP-ALL-KIND-ROWS, open): ready work ships first, the sweep resumes when the wave hands back.
