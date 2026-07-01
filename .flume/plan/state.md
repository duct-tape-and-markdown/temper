# Plan state

- **Phase:** reconcile. Verified on disk: **GOV-GRAPH-DEGREE shipped** — `graph::degree`
  (per-role in/out edge-count bound over the resolved reference arcs) is in `src/graph.rs`,
  wired opt-in at `main.rs:209`. The whole graph scope (route resolution, admissibility,
  `acyclic`, `degree`) and the whole set scope (`count`, `unique`, `membership`, typed
  reference) are on disk (`graph.rs`/`roster.rs`), plus custom-kind declaration + extraction
  algebra (`kind.rs`/`compose.rs`) and the `spec` custom kind in `temper.toml`.
- **Last shipped:** GOV-GRAPH-DEGREE (`be77947`). Queue was empty; state was stale (it still
  listed DEGREE in flight).
- **Filed this tick (2):** **SCHEMA-EMIT** (`open`, pickable) — `temper schema` emits the
  active per-kind contract as an editor JSON Schema (validation channel; docs/hover deferred,
  no guidance-prose source exists). **APPLY-WRITEBACK** (`blockedBy` SCHEMA-EMIT — both add a
  `Command` variant to `main.rs`) — `temper apply`, the three-state write-back engine.
- **Frontier:** distribution + drift write-back are the big unbuilt areas (`schema`, `apply`,
  then `re-add`/`bundle`/`install`/reporters, all fork-free). Spec-kind `references-resolve`
  is NOT a mere config task — the graph scope excludes custom kinds from `by_kind`, and the
  filename-ref vs stem-id mismatch is a new soundness fork `(reference-id-normalization)`.
  `decisions-name-alternatives` still waits on `(decision-marker-predicate)`. More built-in
  harness kinds (agent/hook/command/MCP/settings/plugin) remain adapters to add.
- **Inbox:** empty (no lines to drain). Added open question `(reference-id-normalization)`.

Plan continues: no — queue reconciled, SCHEMA-EMIT pickable and APPLY queued behind it, inbox empty; hand to build.
