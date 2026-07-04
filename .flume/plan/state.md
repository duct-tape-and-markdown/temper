# Plan state

- **Phase:** reconcile. HEAD 80216ad.
- **Last shipped:** the two disjoint `open` heads drained — FENCED-PRIMITIVE
  (`build` 3278caf: the closed-vocab `Fenced` primitive → `body_fenced_blocks`
  /`FencedBlock`, raw fenced blocks serialized into the `[[member]]` manifest at
  compose.rs ~1842) and INSTALL-DRIFT-STRINGS (`build` bb76342: install's placed
  managed-by note/guard reworded off the retired `re-add` verb). Both shipped +
  chore'd (`80216ad`).
- **This tick:** inbox empty; nothing new to file. Verified on disk that the
  `Fenced` primitive landed (kind.rs `Primitive::Fenced`, extract.rs
  `FencedBlock`, compose.rs fenced-block serialization) → flipped
  **GENRE-MANIFEST-LEAF** from `blockedBy FENCED-PRIMITIVE` to **open** (its
  upstream shipped) and refreshed its compose.rs description/notes. Confirmed
  GenreValue/`genres` NOT yet on disk (no type, no kind field), so the entry is
  live work, not shipped. The rest of the addressable-corpus chain
  (IMPACT-LEAF-GRAIN → CONTEXT-VERB) stays `blockedBy`; downstream refs re-checked
  accurate (main.rs Impact ~626, read.rs has `impact` but no `context`).
  EXTRACTION-VOCAB-GAPS (deferred, key-path half), AGENT-KIND (deferred),
  PACKAGING-CHANNELS (parked) all hold.
- **Pickable now:** **one `open`** — GENRE-MANIFEST-LEAF (kind.rs/extract.rs/
  compose.rs + tests/genre_leaf.rs). The chain un-blocks one link per ship on the
  following reconcile.
- **What's next:** build ships GENRE-MANIFEST-LEAF, then IMPACT-LEAF-GRAIN, then
  CONTEXT-VERB (the last link, coverage disclosure bound in). NOT build's work:
  the corpus genre *package* (packages/**, kinds/** genre declarations) is
  out-of-fence — the interactive session lands it after the engine chain ships;
  altitude authoring stays on ask (a). Accepted floor debt still open: temper's
  own `temper.toml`+lock predate MANIFEST-EMIT — a human `chore(harness)` `emit`
  regen.

Plan continues: no — inbox empty, queue reconciled, GENRE-MANIFEST-LEAF is a
live `open` head. Building is how the chain drains; re-planning the same held
queue would spin.
