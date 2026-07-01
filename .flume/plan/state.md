# Plan state

- **Phase:** reconcile. Verified on disk this tick: **KIND-DECLARATION-PARSE
  shipped** — `src/compose.rs` parses `[kind.<name>]` custom kinds into
  `CustomKind { governs, extraction, clauses }` off `AuthorLayer::custom_kinds()`
  (`is_custom_kind_declaration` disambiguates a full custom kind from a built-in
  contract layer; `Extraction::from_table` reused, so an out-of-vocabulary
  primitive is a load error). Still un-shipped and confirmed on disk: `src/import.rs`
  hardwires `discover_spec_files`/`import_spec`; `src/main.rs` `Check` dispatches
  only `skill`+`rule` contracts; `src/graph.rs` reads the standalone `[[edge]]`
  list (`AuthorLayer::edges`); the built-in `Spec` IR (`src/spec.rs`,
  `Workspace.specs`, drift's spec axis, `extract::spec_features`) all remain.
- **Last shipped:** KIND-DECLARATION-PARSE (`fe27364`). Tree clean; no repo-root
  `temper.toml` yet.
- **In flight / next:** the kind-declaration chain, one linear serialized run
  (shared files import/main/check/drift/compose/graph). **KIND-IMPORT-DISCOVERY now
  `open`, pickable** (blocker KIND-DECLARATION-PARSE shipped) → KIND-CHECK-SPEC →
  KIND-RETIRE-BUILTIN-SPEC → KIND-EDGE-RELATIONSHIPS, each `blockedBy` its
  predecessor so exactly one is pickable at a time.
- **Frontier:** after the chain lands, `degree`/`acyclic` (`45-governance.md`) read
  the same relationships; the spec kind's references-resolve clause follows
  KIND-EDGE-RELATIONSHIPS, and decisions-name-alternatives waits on the
  `(decision-marker-predicate)` fork.
- **Inbox:** empty. Open-questions unchanged.

Plan continues: no — sole stale gate flipped (chain head unblocked, pickable),
tail serialized over shared files, inbox empty, no new gap filable ahead of the
chain; hand to build.
