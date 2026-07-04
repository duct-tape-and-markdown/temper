# Plan state

- **Phase:** reconcile. HEAD 963b06e.
- **Last shipped:** IMPACT-LEAF-GRAIN (`build` c06fe46 / `chore` 963b06e) —
  `impact` gained leaf grain: `impact_leaf` resolves a
  `<member>/<genre>/<key>/<field-path>` address off the manifest's serialized
  genre values (`parse_leaf_address`/`resolve_leaf`), reporting citations
  separately from fallout. Verified on disk (`src/read.rs`).
- **This tick:** inbox empty; nothing to file. Confirmed IMPACT-LEAF-GRAIN
  landed → flipped **CONTEXT-VERB** from `blockedBy IMPACT-LEAF-GRAIN` to
  **open** (its upstream shipped; the serialized leaf shape it reads exists).
  Reconcile catch: `impact_leaf`'s **success** path discloses coverage only in
  its not-found error, not its found answer, yet `20-surface.md` binds the
  disclosure to **both** verbs — folded the retrofit into CONTEXT-VERB (it
  shares `read.rs`, so it can't be a parallel entry; the entry owns the shared
  coverage-disclosure helper). Downstream refs re-checked: `read.rs` has
  `why`/`impact`/`impact_leaf`/`requirements`, no `context`; `main.rs` has
  `Command::{Why,Requirements,Impact}` (Impact dispatch ~567), no `Context`.
  EXTRACTION-VOCAB-GAPS (deferred), AGENT-KIND (deferred), PACKAGING-CHANNELS
  (parked) all hold.
- **Pickable now:** **one `open`** — CONTEXT-VERB (read.rs/main.rs +
  tests/read_verbs.rs), the sole open head, parallel-safe.
- **What's next:** build ships CONTEXT-VERB → the addressable-corpus floor
  engine chain fully drains. NOT build's work: the corpus genre *package*
  (packages/**, kinds/** genre declarations) is out-of-fence — the interactive
  session lands it after the engine chain; altitude authoring stays on ask (a).
  Accepted floor debt: temper's own `temper.toml`+lock predate MANIFEST-EMIT —
  a human `chore(harness)` `emit` regen.

Plan continues: no — inbox empty, queue reconciled, CONTEXT-VERB is a live
`open` head. Building is how the chain drains; re-planning the same held queue
would spin.
