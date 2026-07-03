# Plan state

- **Phase:** reconcile. HEAD 2ef5c4d.
- **Last shipped:** LOCK-FRESHNESS-FACTS (`build` fa8067d / `chore` 2ef5c4d) —
  the lock's baselines renamed to two freshness facts. Verified on disk: the
  lock writes/reads `source_hash`+`emit_hash` (import.rs:747-748/983-984), drift
  reads `emit_hash`, and a legacy-shape lock is read as a baseline (drift.rs:407,
  577) — the self-migration the inbox flagged. The prior afterMerge revert did
  not recur.
- **This tick:** drained the inbox note (retry succeeded — LEGACY-READ never
  fired; the revert-visibility hazard recorded as accepted debt). Reconciled the
  queue: LOCK-FRESHNESS-FACTS gone from pending, so EMIT-VERB's dangling
  `blockedBy` flips to `open` — the new chain head. Verified EMIT-VERB unbuilt on
  disk (Command::Apply / ApplyOutcome::Conflicted / three-state merge all stand).
- **Pickable now:** **EMIT-VERB** (`open`). Serialized behind it:
  MANIFEST-EMIT → MANIFEST-GATE-READ → INIT-ONRAMP → EMIT-OWNED-PLACEMENTS (each
  `blockedBy` the prior). Deferred/parked: EXTRACTION-VOCAB-GAPS, AGENT-KIND (no
  consumer), PACKAGING-CHANNELS (creds).
- **What's next:** build ships EMIT-VERB; the next reconcile flips MANIFEST-EMIT
  `open`. Human still owes ask (a) (SDK scaffolding) before any altitude-rung
  entry can be filed.

Plan continues: no — queue reconciled, EMIT-VERB is a pickable head, inbox
drained. Hand to build; re-planning would re-emit the same queue.
