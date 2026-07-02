# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox empty; tree clean.
  HEAD 362ae6c.
- **Last shipped (trunk):** REFERENCE-NORMALIZATION + COMMENT-DIET(compose,main)
  (362ae6c, build). Verified on disk: `strip_suffix` is on the `references` primitive
  in src/kind.rs (the kind-declared normalization, `15-kinds.md`); compose.rs/main.rs
  swept. The engine stays broad — import/check/drift/apply/re-add, bundle, install,
  schema, reporters, coverage/graph, roster set-scope + graph predicates, custom kinds,
  read verbs, section_contains, typed extraction.
- **This tick:** unblocked **COMMENT-DIET(kind)** — its `blockedBy
  REFERENCE-NORMALIZATION` cleared now that the `strip_suffix` addition has landed, and
  kind.rs is disjoint from every other open entry, so the sweep is parallel-safe. No new
  gaps: **CONTRACTS-RETIRE** (dead `contracts/*.toml` mirror — nothing loads it; build.rs
  embeds `packages/`; only comments + a test-fixture string name it) remains the one
  code↔spec gap outstanding. Inbox was empty.
- **Pickable now (8 `open`, all disjoint files):** CONTRACTS-RETIRE (delete-only) and
  7 COMMENT-DIET sweeps (roster/drift/graph/import/contract/kind — one file each).
  Deferred: AGENT-KIND (priority). Parked: PACKAGING-CHANNELS (release creds).

Plan continues: no — queue reconciled, inbox empty, eight disjoint `open` entries are
pickable; building drains them.
