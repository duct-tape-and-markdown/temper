# Plan state

- **Phase:** reconcile. Verified on disk this tick: **KIND-IMPORT-DISCOVERY
  shipped** — `src/import.rs` discovers custom-kind units data-driven off each
  declared `governs` locus (`AuthorLayer::custom_kinds()`), the hardwired
  `specs/*.md` scan is gone, and absent a `temper.toml` custom kind `import` writes
  the built-ins only. Still un-shipped and confirmed on disk: `src/main.rs` `Check`
  dispatches only `skill`+`rule` contracts (custom kinds never reach `engine`); no
  repo-root `temper.toml`; the built-in `Spec` remains (`src/spec.rs`, `lib.rs`
  `pub mod spec`, `extract::spec_features`, `Workspace.specs`, drift's spec axis
  reading `workspace.specs` + `import::discover_spec_files`); `src/graph.rs` reads
  the standalone `[[edge]]` list (`AuthorLayer::edges`).
- **Last shipped:** KIND-IMPORT-DISCOVERY (`2e04070`). Tree clean.
- **In flight / next:** the kind-declaration chain, one linear serialized run over
  shared files (main/check/drift/compose/graph). **KIND-CHECK-SPEC now `open`,
  pickable** (blocker KIND-IMPORT-DISCOVERY shipped) → KIND-RETIRE-BUILTIN-SPEC →
  KIND-EDGE-RELATIONSHIPS, each `blockedBy` its predecessor so exactly one is
  pickable at a time.
- **Frontier:** after the chain lands, `degree`/`acyclic` (`45-governance.md`) read
  the same `[kind.<name>.relationships]`, and the spec kind's references-resolve
  clause follows KIND-EDGE-RELATIONSHIPS; decisions-name-alternatives waits on the
  `(decision-marker-predicate)` fork. None filable ahead of the shape they read.
- **Inbox:** empty. Open-questions unchanged.

Plan continues: no — sole stale gate flipped (chain head unblocked, pickable), tail
serialized over shared files, inbox empty, no gap filable ahead of the chain; hand
to build.
