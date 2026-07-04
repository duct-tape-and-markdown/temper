# Plan state

- **Phase:** reconcile. HEAD f185d2f.
- **Last shipped:** MANIFEST-GATE-READ (`build` 70eb831 / `chore` f185d2f) — the
  committed `temper.toml` manifest's `[[member]]` feature tables are now the
  gate's corpus (`main.rs:700`, `manifest_corpus`, keyed by bare kind). Verified
  on disk: the inbox-recommended legacy fallback landed — a manifest carrying
  zero `[[member]]` tables (temper's own pre-`emit` dogfood) falls back to live
  `check::surface_units` extraction, so the gate never reads an empty corpus and
  the self-gate red does not recur. `config.stale` is the freshness fact in
  `drift.rs`.
- **This tick:** inbox drained (the MANIFEST-GATE-READ revert-diagnosis note: its
  in-fence recommendation shipped in 70eb831; its durable out-of-fence half —
  regenerating temper's own `temper.toml`+lock via `emit` — recorded as accepted
  debt, a human `chore(harness)` follow-on). Reconciled the queue: MANIFEST-GATE-READ
  shipped, so INIT-ONRAMP's `blockedBy` is discharged → flips to `open`, the new
  chain head. Verified INIT-ONRAMP unbuilt: `import` still copies members into the
  `.temper/` tree (`import.rs:262`, "the copy tree stays authoritative").
- **Pickable now:** **INIT-ONRAMP** (`open`) — `import`→`init`, scan into a config
  skeleton over members in place, zero `.temper/` copy tree. Serialized behind it:
  EMIT-OWNED-PLACEMENTS (`blockedBy` INIT-ONRAMP — shares `src/drift.rs`). Deferred:
  EXTRACTION-VOCAB-GAPS, AGENT-KIND (no consumer). Parked: PACKAGING-CHANNELS (creds).
- **What's next:** build ships INIT-ONRAMP (in-place skeleton, no copy tree; check
  live-extracts in-place members, emit skips them); the next reconcile flips
  EMIT-OWNED-PLACEMENTS `open`. Human still owes altitude ask (a) (SDK scaffolding)
  before any altitude-rung entry; and the accepted-debt `temper.toml` regen.

Plan continues: no — queue reconciled, INIT-ONRAMP is a pickable `open` head,
inbox drained. Hand to build; re-planning would re-emit the same queue.
