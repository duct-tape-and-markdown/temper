# Plan state

- **Phase:** reconcile. Verified on disk this tick: **KIND-CHECK-SPEC shipped** —
  `src/main.rs` (lines 201–212) dispatches every declared custom kind through its
  **own** composed extractor (`custom.extraction.extract`) and **own** contract, the
  same two greens the built-ins run but data-driven. Still on disk: the built-in
  `Spec` scaffold (`src/spec.rs`, `lib.rs` `pub mod spec`, `extract::spec_features`
  — dead but for its test, `check::Workspace.specs`, `import::discover_spec_files`,
  drift's spec axis) and the standalone `[[edge]]` construct (`compose.rs`
  `parse_edges`, `graph.rs` over `AuthorLayer::edges`). No repo-root `temper.toml`.
- **Last shipped:** KIND-CHECK-SPEC (`dbcc53a`). Tree clean.
- **In flight / next:** the built-in-spec retirement, re-scoped this tick. The old
  single entry missed `main.rs` (its `custom_units`/`spec_unit` source the check off
  `ws.specs`) — split into **KIND-CUSTOM-UNIT-LOADER** (`open`, pickable: a generic
  `Unit::from_surface_dir` reader closing the `root == "specs"` special case) →
  **KIND-RETIRE-BUILTIN-SPEC** (then near-pure deletion; drift's spec axis dropped as
  a stated scope cut) → **KIND-EDGE-RELATIONSHIPS**, each `blockedBy` its predecessor
  (logical dependency; files near-disjoint) so exactly one is pickable at a time.
- **Frontier:** after the chain, `degree`/`acyclic` (`45-governance.md`) and the spec
  kind's references-resolve clause read the same `[kind.<name>.relationships]`;
  decisions-name-alternatives waits on `(decision-marker-predicate)`. None filable
  ahead of the shape they read.
- **Inbox:** empty. Open-questions unchanged.

Plan continues: no — chain head unblocked + pickable, tail serialized over a shared
logical/file dependency, inbox empty, no gap filable ahead of the shape; hand to build.
