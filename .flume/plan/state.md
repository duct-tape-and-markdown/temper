# Plan state

- **Phase:** reconcile. HEAD 8197053.
- **Last shipped:** the memory-tree flip (`chore` 6954475); the scripted-altitude
  corpus re-cut landed as intent (`specs:` 32ea84d). Code has not migrated to it.
- **This tick:** drained the inbox ruling — John delegated ask (b) of
  `(scripted-altitude-reconcile)` to plan. Mapped the floor-migration
  entanglement (an Explore sweep + disk check: no `config.stale`, lock still
  `import_hash`/`last_applied`, `import`/`apply`/`re-add` intact, two-projectors
  live) and FILED the wave as a strict serialized `blockedBy` chain:
  **READD-RETIRE → LOCK-FRESHNESS-FACTS → EMIT-VERB → MANIFEST-EMIT →
  MANIFEST-GATE-READ → INIT-ONRAMP → EMIT-OWNED-PLACEMENTS** (all pure Rust).
  Every delta touches `drift.rs`/`main.rs` and shares the lock shape, so nothing
  is parallel-safe — the head (READD-RETIRE) is `open`, the rest each `blockedBy`
  the prior; a ship unblocks the next on the following reconcile. Updated the fork
  (ask (b) discharged; ask (a) — the TS SDK — stays parked, altitude rung only).
- **Pickable now:** **READD-RETIRE** (chain head). Deferred/parked below it:
  EXTRACTION-VOCAB-GAPS, AGENT-KIND (no consumer), PACKAGING-CHANNELS (release
  creds). The floor chain's own tail entries are `blockedBy`-held until their
  upstream ships.
- **What's next:** build ships READD-RETIRE, then each successive plan reconcile
  flips the next chain link `open`. Human still owes ask (a) (SDK scaffolding)
  before any altitude-rung entry can be filed.

Plan continues: no — the inbox is drained, the floor wave is filed as a
buildable serialized chain with a pickable head, and the residual entries are
reconciled and human-gated. Hand to build; re-planning would re-emit the same
queue.
