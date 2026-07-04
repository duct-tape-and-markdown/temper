# Plan state

- **Phase:** reconcile. HEAD 4bdc612.
- **Last shipped:** the scripted-altitude floor chain (7/7, drained) + the
  addressable-corpus ratification (`specs:` 52f149c) with the dogfood manifest
  regenerated. Inbox carried the ratified floor slice's routing.
- **This tick:** drained the inbox (2 lines). Filed the **addressable-corpus
  floor engine** as a serialized chain — FENCED-PRIMITIVE (open) →
  GENRE-MANIFEST-LEAF → IMPACT-LEAF-GRAIN → CONTEXT-VERB — each `blockedBy` the
  prior (they share extract.rs/kind.rs then read.rs/main.rs; not parallel-safe).
  Also filed INSTALL-DRIFT-STRINGS (open) — reword install's placed managed-by
  note/guard off the retired `re-add` verb (inbox line 1). Split
  EXTRACTION-VOCAB-GAPS: its `Fenced` half graduated to FENCED-PRIMITIVE (consumer
  arrived); the key-path `field` half stays deferred. AGENT-KIND (deferred),
  PACKAGING-CHANNELS (parked) verified still holding.
- **Pickable now:** **two `open`, disjoint** — INSTALL-DRIFT-STRINGS
  (src/install.rs) and FENCED-PRIMITIVE (src/kind.rs, src/extract.rs). No shared
  path. The rest of the genre chain is `blockedBy` and un-blocks one link per
  ship on the following reconcile.
- **What's next:** build ships the two open entries, then the genre chain
  link-by-link. NOT build's work (inbox points 2–4): the corpus genre *package*
  (packages/**, kinds/** genre declarations) is out-of-fence — the interactive
  session lands it after the engine chain ships (parse-first, red-interim lesson);
  altitude authoring stays on ask (a); rung-3 adoption is a staged human ceremony
  per document. Accepted floor debt still open: temper's own `temper.toml`+lock
  predate MANIFEST-EMIT — a human `chore(harness)` `emit` regen.

Plan continues: no — inbox drained, queue reconciled, two disjoint `open` entries
ready. Building is how the chain drains; re-planning would re-emit the same held
queue.
