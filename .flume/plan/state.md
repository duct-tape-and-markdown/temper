# Plan state

- **Phase:** reconcile. HEAD ac60a0c.
- **Last shipped:** EMBED-NESTED-WALK (build 1f3d761, chore c65c2ed) — build.rs's
  kind embed walk tolerates the nested `kinds/<provider>/<name>/` layout; then two
  human chores landed: the curated skill/rule KIND.md moved to
  `kinds/claude-code/{skill,rule}/` with `provider = "claude-code"` lines (3cf756b),
  and the spec corpus classed into intent/architecture/process dirs (1d8448e).
- **This tick:** drained the inbox (2 lines). Refreshed every flat `specs/NN-*.md`
  citation across pending + open-questions to its classed path (mechanical, per
  1d8448e). Un-parked **BINDING-QUALIFY** → `open`: both blockers cleared (nested
  embeds shipped; file-move landed, verified on disk — `BUILTIN_KINDS` is still bare
  `["skill","rule"]` and floor tuples still bare, so the qualify work is unshipped).
  Retargeted MEMORY-KIND/AGENT-KIND curated KIND.md paths to `kinds/claude-code/*`
  under the provider axis. Other entries reconciled unchanged.
- **In flight / pickable:** **BINDING-QUALIFY** (open) — the sole buildable entry.
  Parked: MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS. Deferred:
  EXTRACTION-VOCAB-GAPS, AGENT-KIND (both no-consumer).
- **Next:** build picks BINDING-QUALIFY — qualify the floor tuples + route bare
  `[kind.*]` refs through `resolve_bare`, published-binds-qualified as a bundle check.

Plan continues: no — queue reconciled, inbox drained, one open entry (BINDING-QUALIFY)
ready for build. Building is how the queue drains.
