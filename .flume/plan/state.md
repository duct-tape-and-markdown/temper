# Plan state

- **Phase:** reconcile. HEAD f8dabad.
- **Last shipped:** both SDK-seam chain heads — GATE-KIND-RESOLVE (`build` ff493d9)
  and SDK-CARRIAGE-CONVERGE (`build` 1d2b9fd), shipped together (`chore` f8dabad).
  The Rust gate now resolves a manifest member's qualified kind to its bare key
  before dispatch; the SDK `toManifestMember` sectionizes per heading like the importer.
- **This tick:** verified both heads landed on disk and neither follower's work
  has — `src/assembly_artifacts.rs` is absent, and `projectMember` (project.ts:136)
  still calls `projectBytes(fields, body)` without the `placements` arg. So both
  serialized followers unblock: **GATE-READS-ASSEMBLY** and
  **SDK-PLACEMENT-ROUNDTHROUGH** flip `blockedBy → open`. They are cross-language
  disjoint (`src/**`+`tests/**` vs `sdk/**`), so both `open` and parallel-safe.
  Inbox empty. Carried entries hold: EXTRACTION-VOCAB-GAPS deferred (kind.rs:836),
  AGENT-KIND deferred, PACKAGING-CHANNELS parked.
- **Pickable now:** GATE-READS-ASSEMBLY and SDK-PLACEMENT-ROUNDTHROUGH (both `open`,
  disjoint). No followers remain behind them — these are the tails of the two chains.
- **What's next:** build ships the two tails in parallel. Once both land, the four
  ruled SDK seams are fully closed and the dogfood's full migration onto the SDK is
  the ledger's TS-primary human ceremony (not a pending entry).

Plan continues: no — queue reconciled, two disjoint `open` tails pickable. Hand to build.
