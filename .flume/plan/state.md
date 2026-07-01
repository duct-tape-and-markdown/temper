# Plan state

- **Phase:** reconcile. Verified on disk this tick: **KIND-RETIRE-BUILTIN-SPEC
  shipped** — `src/spec.rs` gone, `lib.rs` carries no `pub mod spec`; the built-in
  `Spec` scaffold (extractor, workspace `specs` axis, spec drift axis) is retired.
  Still on disk (the edge-reconcile target): the standalone `[[edge]]` construct —
  `compose::parse_edges`/`parse_edge`/`edge_str`, the `[[edge]]` reader,
  `EdgeRootNotArray`/`BadEdge`, `AuthorLayer::edges`, and `graph`/`main` reading it.
  The custom-kind machinery (`CustomKind`, `[kind.<name>]` parse) is on disk and its
  comment notes `.relationships` is "folded in elsewhere" — this entry.
- **Last shipped:** KIND-RETIRE-BUILTIN-SPEC (`de491e7`). Tree clean.
- **In flight / next:** **KIND-EDGE-RELATIONSHIPS** is now `open` (blocker shipped)
  — move edge declaration from the top-level `[[edge]]` array into
  `[kind.<name>.relationships]` under the owning kind, preserving the `Edge`/graph
  behavior. File cites re-verified accurate against `src/compose.rs`, `src/graph.rs`,
  `src/main.rs`, `tests/graph.rs`, `tests/temper_toml.rs`.
- **Frontier:** after this, `degree`/`acyclic` (`45-governance.md`) and the spec
  kind's references-resolve clause read the same `[kind.<name>.relationships]`;
  decisions-name-alternatives waits on `(decision-marker-predicate)`. None filable
  ahead of the relationships shape they read.
- **Inbox:** empty. Open-questions unchanged.

Plan continues: no — sole entry unblocked + pickable, file cites verified, inbox
empty, no gap filable ahead of the relationships shape; hand to build.
