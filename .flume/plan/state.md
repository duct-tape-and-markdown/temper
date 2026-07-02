# Plan state

- **Phase:** reconcile. HEAD 30c15f3.
- **Last shipped (trunk):** APPLY-REEMIT-PROJECTION — `apply` re-emits each
  projection deterministically over the three-state drift model (verified on disk:
  `src/drift.rs`/`src/main.rs` Apply arm; the superseded surgical frontmatter-patch
  model is gone).
- **This tick:** drained the inbox's read-path-unification item into two serialized
  entries — SURFACE-READING-GENERIC (check reads every kind's surface member document
  through the one generic `Unit` loader; retires the skill/rule IR→Unit adapter on the
  check path) then IMPORT-DISCOVERY-GENERIC (discovery keys off the embedded built-in
  KIND.md `governs`), blockedBy the first since both touch `src/drift.rs`. Rewrote the
  deferred AGENT-KIND to the post-unification architecture (typed IR + embedded KIND.md
  + `agent.anthropic` package, generic-loader read). Parked trio unchanged (none shipped).
- **Pickable now:** SURFACE-READING-GENERIC (open, sole). IMPORT-DISCOVERY-GENERIC
  blockedBy it; AGENT-KIND deferred; PACKAGING-CHANNELS / COMMUNITY-DOCS parked. Sole
  live OPEN fork: `(edge-representation-unify)` — human to settle the canonical edge form.

Plan continues: no — inbox drained, one open entry filed disjoint of the blocked/
parked/deferred set; hand to build.
