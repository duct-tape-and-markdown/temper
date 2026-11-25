# BARE-EDGE-TARGET-DEFERS-LIKE-ITS-QUALIFIED-SPELLING

Shipped. `edgeTargetFacts` (sdk/src/emit.ts) now judges `defersToGate` on the
`edgeLookupKey` address, so a bare name in a one-element `to` set defers exactly as
its `kind:name` spelling does; a bare name in a multi-element set stays bare, has no
host-address reading, and still refuses. The engine side needed no change: check
already route-resolves a bare one-element leaf and reports `graph.route` for both
spellings (tests/it/emit.rs end-to-end). No debt observed.

Named-lines gate collects only cargo test titles: the SDK `node --test` titles in
refusals.test.ts went unseen, so the pin also lives in tests/it/emit.rs. Plan
should place `tests[]`/`pins[]` lines in Rust tests while that holds.
