# Plan state

- Spec derived through: caf29fa — unchanged; no specs/ commits past it.
- Audited through: 4ab6fe2 — advanced from a00e14a; DECLARATIONS-TELEMETRY-HOOK-PROVIDER-FACE-MOVE (19258b7/a6db2b5) verified shipped clean on disk (TELEMETRY_EVENT_HOOKS re-grepped: zero references left in declarations.ts, both live uses in builtins.ts correct).
- Residue swept through: 4ab6fe2 — advanced from a00e14a; one finding, both handled this tick (see "This tick" below).
- Posture swept through: sdk/src/declarations.ts (+ its immediate imports assembly.ts/kind.ts/contract.ts/prose.ts/builtins.ts) covered; sdk/src/dial.ts next in rotation. Unchanged this tick.
- This tick: POST-SHIP RECONCILIATION over a00e14a..HEAD. Audit: shipped move verified clean, no pending entry to drop. Sweep: the move's own line-shift (-24 in declarations.ts) had gone unreflected in DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE's cited lines (873/908/915/924 → corrected to 849/884/891/900) — rescoped in place. A second finding, the move's stale cite at sdk/src/contract.ts:338 ("declarations.ts" for TELEMETRY_EVENT_HOOKS, now builtins.ts), is comment/cite staleness — per the ride-only rule, filed as the open-questions record's eleventh orphan, not a standalone entry. Both cursors advance to 4ab6fe2.
- Queue: 3 pending — 1 open (DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE, lines rescoped this tick), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks: 2 (multi-harness-projection, lazy-grounds). Friction: 0. Amendments: 0. Inbox: 0 notes.

Plan continues: after-build — the only remaining live job is the dial.ts posture rotation, and DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE is pickable now.
