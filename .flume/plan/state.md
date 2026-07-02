# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox drained; tree clean.
  HEAD 2521f7d.
- **Last shipped (trunk):** the corpus gates on its Decision discipline (2521f7d,
  hand) — decisions-name-alternatives required in the dogfood; READ-EDGE-UNIFY (6e80405)
  before it. Verified on disk: the `packages/` migration has fully landed — build.rs
  embeds built-in packages from `packages/<name>/PACKAGE.md` (src/builtin.rs); `.temper/`
  is authored; custom kinds feed the graph's `by_kind`. The engine stays broad —
  import/check/drift/apply/re-add, bundle, install, schema, reporters, coverage/graph,
  roster set-scope + graph predicates, custom kinds, read verbs, section_contains, typed
  extraction.
- **This tick:** filed two disk-verified code↔spec gaps — **REFERENCE-NORMALIZATION**
  (engine gains the kind-declared `strip_suffix` the dogfood spec KIND.md waits on;
  `graph::resolved_edges` does exact match with no normalization today) and
  **CONTRACTS-RETIRE** (the `contracts/*.toml` mirror is dead — nothing loads it since
  build.rs embeds `packages/` — the spec's 'no contracts/ mirror' wants it gone). Drained
  the inbox: **COMMENT-DIET** fanned out per heavy-offender module (8 disjoint entries;
  the `kind.rs` sweep serialized `blockedBy` REFERENCE-NORMALIZATION since both edit that
  file). SPEC-DECISION-DOGFOOD noted shipped-by-hand, not re-filed (`.temper/**` is human
  territory).
- **Pickable now (9 `open`, all disjoint files):** REFERENCE-NORMALIZATION (src/kind.rs),
  CONTRACTS-RETIRE (delete-only), and 7 COMMENT-DIET sweeps (compose/main/roster/drift/
  graph/import/contract — one file each). Serialized: COMMENT-DIET(kind) `blockedBy`
  REFERENCE-NORMALIZATION. Deferred: AGENT-KIND (priority). Parked: PACKAGING-CHANNELS.

Plan continues: no — inbox drained, queue reconciled, nine disjoint `open` entries are
pickable; building drains them.
