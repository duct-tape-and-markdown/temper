# Plan state

- **Phase:** reconcile. HEAD cdd26ca.
- **Last shipped:** MANIFEST-EMIT (`build` adbd18a / `chore` cdd26ca) — import
  serializes the generated-canonical manifest beside `.temper/`: every member's
  extracted `Features` lands as a `[[member]]` table (`emit_manifest`,
  main.rs:260), the hand-authored floor is patched format-preserving, the
  `member` root re-emitted whole. Verified on disk: `compose.rs` parses the
  `[[member]]` tables (`ManifestMember`, `members` field) but the read side is
  **inert** — the gate does not yet consume them.
- **This tick:** inbox empty. Reconciled the queue: MANIFEST-EMIT shipped, so
  MANIFEST-GATE-READ's dangling `blockedBy` flips to `open` — the new chain
  head. Verified MANIFEST-GATE-READ unbuilt: `check::surface_units` still ranges
  the `.temper/` copy tree (check.rs:126, main.rs:438), `drift.rs` carries no
  `config.stale`/`emit_hash` finding.
- **Pickable now:** **MANIFEST-GATE-READ** (`open`). Serialized behind it:
  INIT-ONRAMP → EMIT-OWNED-PLACEMENTS (each `blockedBy` the prior). Deferred:
  EXTRACTION-VOCAB-GAPS, AGENT-KIND (no consumer). Parked: PACKAGING-CHANNELS
  (creds).
- **What's next:** build ships MANIFEST-GATE-READ (manifest becomes the gate's
  corpus, `config.stale` lands); the next reconcile flips INIT-ONRAMP `open`.
  Human still owes ask (a) (SDK scaffolding) before any altitude-rung entry.

Plan continues: no — queue reconciled, MANIFEST-GATE-READ is a pickable head,
inbox drained. Hand to build; re-planning would re-emit the same queue.
