# Plan state

- **Phase:** reconcile. HEAD bec3dad.
- **Last shipped:** ACTIVATION-KEY-PARSE + EDGE-CLAUSE-RETIRE (build ceb8bba/c87d3ab,
  shipped bec3dad). Verified on disk: typed `Activation` enum (Always /
  DescriptionTrigger / PathsMatch / Event) + `parse_activation` live in `src/kind.rs`
  on `CustomKind.activation`, nothing consumes it yet; no `EdgeClause` / `[edge.*]`
  in `document.rs`/`frontmatter.rs` (only stale comment mentions in read.rs/graph.rs).
- **This tick:** unblocked **REACHABILITY** (`blockedBy ACTIVATION-KEY-PARSE` → `open`)
  — its consumer, the typed `Activation`, is now live; confirmed the world node +
  `reachable` predicate are still absent from `src/graph.rs` (real open work). Scoped
  it truthfully: pure `fn` over graph.rs only, repo file-set as a *parameter*.
  Corrected drifted `kind.rs` anchors in MEMORY-KIND/EXTRACTION-VOCAB-GAPS (Primitive
  361→456, Field-get 404→499, parse_primitive 933→1048). Inbox empty — nothing to drain.
- **In flight / pickable:** REACHABILITY (`open`, `src/graph.rs` + `tests/graph.rs`
  only — the sole pickable entry, trivially parallel-safe). The other five stay parked
  (MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS) or deferred (EXTRACTION-VOCAB-GAPS,
  AGENT-KIND) — all human-gated.
- **Next:** build ships REACHABILITY's machinery; then a human wires the gate
  (main.rs) + a severity-carrying package clause + the curated kinds/skill|rule
  `activation` lines (outside the fence) for the reachability dogfood to fire.

Plan continues: no — queue reconciled, inbox empty, one `open` entry ready. Hand to
build; building is how the queue drains.
