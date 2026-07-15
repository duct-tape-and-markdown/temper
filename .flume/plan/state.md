# Plan state

- Spec derived through: f67303c
- Audited through: c0bbf3b
- Residue swept through: c0bbf3b
- This tick: reconciled the 9223917..HEAD ship window — two build commits,
  CHECK-ARG-HALF-GATE (4256274) + GLOB-VALIDITY-PREDICATE (46b8cd1); no plan
  tick had advanced the audit cursor past the GROWN-FIELD ship. Both are the
  only window commits touching src/sdk/tests (the rest is docs/example/flume).
  AUDIT: both shipped, already removed from pending by build; no queued
  entry's work is done → no drops. CHECK-ARG shipped → re-tested the two
  entries that named it as blocker: LAYOUT-OVERLAY-CHECK-GAP (blockedBy →
  **open**, no fork — now the one pickable entry) and EMIT-INTO-REROOT-REAP
  (blockedBy → **open**, still held by `(lock-upgrade-migration-posture)` via
  dependsOnForks). SWEEP: CHECK-ARG's removal of `empty_assembly_incoherence`
  + its tests is clean subtraction — `grep` finds zero leftover refs;
  GLOB-VALIDITY reuses `kind::compile_glob` (engine.rs:375 — no second glob
  engine, spec-faithful to 0022) — no new residue entry. The window reworked
  main.rs (CHECK-ARG restructure) and opened builtins.ts / contract.rs /
  session_start.rs: refreshed the drifted main.rs citations in LAYOUT-OVERLAY
  (896-919→862, 1024→990), SATISFIES (1051/1054→1017/1020), EMIT-INTO
  (39/319→134,308-316), and the two open-questions riders those files carry
  (builtins.ts packages/ cites 558/598/635→565/611/648; contract.rs straggler
  459→475; session_start `+++` fixtures re-verified unmoved at 128/133/146).
- Queue: LAYOUT-OVERLAY-CHECK-GAP (open, no fork) is the one pickable entry;
  SATISFIES-LABEL-QUALIFY + EMIT-INTO-REROOT-REAP (both open but
  dependsOnForks `(lock-upgrade-migration-posture)`), LOCK-SPELLING-REAP
  (blockedBy SATISFIES + same fork), PACKAGING-CHANNELS-REMAINDER (parked) all
  held. Pickable set is the single main.rs edit — disjoint.

Plan continues: no — window reconciled, both cursors to HEAD (c0bbf3b); inbox
empty, spec cursor f67303c is specs/ HEAD (no delta). No input below
reconciliation is live. LAYOUT-OVERLAY-CHECK-GAP is pickable — build takes over.
