# Plan state

- **Phase:** reconcile. HEAD f0f64a3.
- **Last shipped (trunk):** BUILTIN-EXTRACT-GENERIC — built-in extraction now runs
  through the embedded KIND definitions via the generic driver (verified on disk:
  `src/builtin_kind.rs`, `build.rs` embeds `kinds/{skill,rule}/KIND.md`, no per-kind
  `*_features` fn remains). The whole extraction-unification wave is drained.
- **This tick:** verified the surface is far more built out than the queue implied —
  all CLI verbs (import/check/diff/apply/re-add/bundle/install/schema/why/
  requirements) are shipped and tested; coverage/roster/graph, member-published
  requirements, and the embedded built-in packages (`skill.anthropic`/`rule.anthropic`,
  the `rule`→`rule.anthropic` rename done in `src/builtin.rs`) all landed. Found one
  clean spec/code divergence and filed it: **APPLY-REEMIT-PROJECTION** — `apply` still
  runs the superseded surgical frontmatter-patch model; the live Decision
  (`specs/20-surface.md`) re-emits the projection deterministically. Inbox empty; no
  forks moved.
- **Pickable now:** APPLY-REEMIT-PROJECTION (open, sole — touches drift.rs/main.rs/
  tests only). AGENT-KIND deferred; PACKAGING-CHANNELS / COMMUNITY-DOCS parked. Sole
  live OPEN fork: (edge-representation-unify) — confirmed this tick (surface `[edge.*]`
  never reaches the gate's graph); human to settle the canonical form.

Plan continues: no — one open entry filed, disjoint from the parked/deferred set;
hand to build.
