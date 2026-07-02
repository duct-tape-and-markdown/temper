# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox drained; tree clean.
  HEAD 7de5a75.
- **Last shipped (trunk):** IMPORT-SKILL-LOCUS hand-shipped (d057e74) — skills scan
  `.claude/skills/`, so one project-root `harness_path` captures skills + rules.
  Confirmed on disk: `discover_skill_dirs` (src/import.rs:221) scans
  `<harness>/.claude/skills/`. The engine stays broad against the corpus —
  import/check/drift/apply/re-add, bundle, install, schema, reporters, coverage/graph,
  roster set-scope predicates, custom `.temper/kinds/`, the read verbs, section_contains.
- **This tick:** drained the inbox BUG into **READ-EDGE-UNIFY** (`open`). `temper why`
  narrates edges from the surface's `[edge.*]` document clauses (src/read.rs, via
  `skill.edges`/`rule.edges`), while `graph::check`/`acyclic`/`degree` (src/graph.rs)
  resolve the gate's graph from `[[kind.relationships]]` over extracted `routes_to`
  features (src/main.rs check arm) — the two disagree on a `routes_to` edge. Fix
  (inbox-directed): the read family consumes the gate's resolved edge set, one source of
  truth. Verified on disk the two edge paths are fully disconnected — extraction never
  reads `[edge.*]` — so filed the deeper representation fork as `(edge-representation-unify)`.
- **Pickable now (1 `open`):** READ-EDGE-UNIFY (single atomic edge-source unification; no
  parallel peer). Deferred: AGENT-KIND (priority). Parked: PACKAGING-CHANNELS (human
  release creds).

Plan continues: no — inbox drained, queue reconciled, READ-EDGE-UNIFY is pickable
`open`; building drains it.
