# Plan state

- Spec derived through: caf29fa — advanced from 087b90a; both commits routed this tick with zero entries filed (see "This tick" below).
- Audited through: a00e14a — unchanged; 19258b7 (ship DECLARATIONS-TELEMETRY-HOOK-PROVIDER-FACE-MOVE, build a6db2b5) touched sdk/src/declarations.ts + sdk/src/builtins.ts since — a00e14a..HEAD is a live post-ship reconciliation window, next live input.
- Residue swept through: a00e14a — same open window as above; unreconciled.
- Posture swept through: sdk/src/declarations.ts (+ its immediate imports assembly.ts/kind.ts/contract.ts/prose.ts/builtins.ts, read for context per the posture-sweep rule) covered; sdk/src/dial.ts next in rotation (tree order after declarations.ts; the phrase delta at 2e2b32a still owes sdk/src/ the remainder — dial.ts, emit.ts, index.ts, kind.ts, needs.ts, prose.ts — then the tests/ tree). Unchanged this tick.
- This tick: SPEC DELTA. Routed 049ae18 (invariant 8, intent.md) and caf29fa (0035 embedded-locus amendment) — zero entries filed, both verified-already-moot on disk (detail in commit body). Cursor advanced to caf29fa.
- Queue: 3 pending — 1 open (DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE, unblocked since 19258b7 shipped its blocker), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks: 2 (multi-harness-projection, lazy-grounds). Friction: 0. Amendments: 0. Inbox: 0 notes.

Plan continues: yes — post-ship reconciliation over a00e14a..HEAD is the next live input, ahead of the open dial.ts posture rotation.
