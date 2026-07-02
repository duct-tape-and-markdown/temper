# Plan state

- **Phase:** reconcile. HEAD 39269ef.
- **Last shipped:** DECLARED-FRONTMATTER-ADAPTER (fab4f79 / build 2a11cb0) — the
  per-kind IRs collapsed into one generic frontmatter adapter (verified on disk:
  no `src/skill.rs`/`src/rule.rs`; `document.rs`/`frontmatter.rs` carry it;
  `BUILTIN_KINDS = ["skill","rule"]`).
- **This tick:** drained the inbox — the join/boundary spec revision (39269ef,
  d53e207) filed as its three enumerated engine consequences: **ACTIVATION-KEY-PARSE**
  (open — parse an inert `activation` key into the closed vocab, red-interim shape of
  FORMAT-KEY-PARSE), **EDGE-CLAUSE-RETIRE** (open — remove dead `[edge.*]` surface
  syntax; coupling is a join), disjoint files (kind.rs vs document.rs+frontmatter.rs)
  so **parallel-safe**; and **REACHABILITY** (blockedBy ACTIVATION-KEY-PARSE — world
  node + `reachable` predicate). Updated `(edge-representation-unify)` with a
  join-retirement datum. Verified anchors on disk (BUILTIN_KINDS:30, EdgeClause in
  document.rs/frontmatter.rs only, graph.rs `Edge` is `compose::Edge`).
- **In flight / pickable:** ACTIVATION-KEY-PARSE + EDGE-CLAUSE-RETIRE (both `open`,
  file-disjoint → build fans them out in parallel). REACHABILITY waits on the first.
  The other five entries stay parked (MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS)
  or deferred (EXTRACTION-VOCAB-GAPS, AGENT-KIND) — human-gated.
- **Next:** build ships the two open entries, then REACHABILITY; a human adds the
  curated kinds/skill + kinds/rule `activation` lines (outside the fence) for the
  reachability dogfood to fire.

Plan continues: no — queue reconciled, inbox drained, two file-disjoint `open`
entries ready. Hand to build; building is how the queue drains.
