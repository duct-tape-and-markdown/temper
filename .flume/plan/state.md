# Plan state

- **Phase:** reconcile. HEAD e70d689.
- **Last shipped:** GENRE-MANIFEST-LEAF (`build` 8a14f32 / `chore` e70d689) —
  typed `Genre`/`GenreValue` parsed from the `[[genres]]` header array
  (`kind.rs`), `fold_genres` folds a member's genre fences into typed values, and
  compose.rs serializes each into a `[[member.genre]]` table (`leaves` string
  table + keyed `collections`). Verified on disk.
- **This tick:** inbox empty; nothing new to file. Confirmed GENRE-MANIFEST-LEAF
  landed → flipped **IMPACT-LEAF-GRAIN** from `blockedBy GENRE-MANIFEST-LEAF` to
  **open** (its upstream shipped; the serialized leaf shape it reads now exists)
  and refreshed its notes. CONTEXT-VERB stays `blockedBy IMPACT-LEAF-GRAIN` (both
  edit read.rs + main.rs — not parallel-safe). Downstream refs re-checked
  accurate: `read.rs` has `why`/`impact`/`requirements`, no `context`; `main.rs`
  has `Command::{Why,Requirements,Impact}` (Impact dispatch ~565), no `Context`.
  EXTRACTION-VOCAB-GAPS (deferred, key-path half), AGENT-KIND (deferred),
  PACKAGING-CHANNELS (parked) all hold.
- **Pickable now:** **one `open`** — IMPACT-LEAF-GRAIN (read.rs/main.rs +
  tests/read_verbs.rs). The chain un-blocks CONTEXT-VERB on the following
  reconcile once it ships.
- **What's next:** build ships IMPACT-LEAF-GRAIN, then CONTEXT-VERB (the last
  link of the addressable-corpus floor engine chain, coverage disclosure bound
  in). NOT build's work: the corpus genre *package* (packages/**, kinds/** genre
  declarations) is out-of-fence — the interactive session lands it after the
  engine chain ships; altitude authoring stays on ask (a). Accepted floor debt
  still open: temper's own `temper.toml`+lock predate MANIFEST-EMIT — a human
  `chore(harness)` `emit` regen.

Plan continues: no — inbox empty, queue reconciled, IMPACT-LEAF-GRAIN is a live
`open` head. Building is how the chain drains; re-planning the same held queue
would spin.
