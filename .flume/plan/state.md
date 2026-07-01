# Plan state

- **Phase:** reconcile. Verified on disk this tick: **KIND-CUSTOM-UNIT-LOADER
  shipped** — `src/main.rs` `custom_units` reads every declared custom kind's units
  via the generic `kind::Unit::from_surface_dir` off its `governs.root`, with no
  `ws.specs`/`root == "specs"` special case. Still on disk (the retirement targets):
  the built-in `Spec` scaffold — `src/spec.rs`, `lib.rs` `pub mod spec`,
  `extract::spec_features` (dead but for its test), `check::Workspace.specs`,
  `import::discover_spec_files` (drift-only caller), drift's spec axis — and the
  standalone `[[edge]]` construct (`compose::parse_edges`, `graph` over
  `AuthorLayer::edges`). No repo-root `temper.toml`.
- **Last shipped:** KIND-CUSTOM-UNIT-LOADER (`7901241`). Tree clean.
- **In flight / next:** **KIND-RETIRE-BUILTIN-SPEC** is now `open` (its blocker
  shipped) — near-pure deletion of the built-in `Spec` plumbing; drift's spec axis
  dropped as a stated scope cut. Reconciled this tick: dropped `tests/cli.rs` from
  its blast radius (not spec-dependent — self-host import carries no root
  `temper.toml`, `diff` runs over spec-less harnesses) and corrected the phantom
  "spec drift tests" claim (none exist). Then **KIND-EDGE-RELATIONSHIPS** (`per`
  re-cited to 15-kinds "The entity graph is a kind capability"), `blockedBy` the
  retirement — file-disjoint but serialized for review clarity.
- **Frontier:** after the chain, `degree`/`acyclic` (`45-governance.md`) and the
  spec kind's references-resolve clause read the same `[kind.<name>.relationships]`;
  decisions-name-alternatives waits on `(decision-marker-predicate)`. None filable
  ahead of the shape they read.
- **Inbox:** empty. Open-questions unchanged.

Plan continues: no — head unblocked + pickable, tail serialized behind it, inbox
empty, no gap filable ahead of the shape; hand to build.
