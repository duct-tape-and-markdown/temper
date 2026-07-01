# Plan state

- **Phase:** reconcile. The inbox PRIORITY reframe (spec `dacec45`) is drained into a
  three-entry **serialized chain** — the meaningful-contract mechanism, not more kinds:
  REQUIREMENTS-PARSE → SATISFIES-REPRESENTATION → REQUIREMENT-COVERAGE. Verified
  unshipped: `rg` finds no requirement/satisfies/coverage code in `src/` (only prose hits).
- **Last shipped:** CHECK-REPORTERS (`9010dfe`); queue since reconciled.
- **In flight:** none.
- **Pickable now (1):** **REQUIREMENTS-PARSE** (`open`) — parse `[requirement.<name>]`
  (`means`/`required`) in `compose.rs`, parse-only. The other two are `blockedBy` behind
  it in order (disjoint files, chained so COVERAGE has both upstreams landed).
- **Deferred:** **AGENT-KIND** — deprioritized by the reframe (more built-in kinds is the
  wrong direction); revive only if a story demands the `agent` kind.
- **Inbox:** drained (both lines routed — reframe → the chain, AGENT-KIND → deferred).
  Open questions unchanged (no fork resolved; the reframe is spec-settled, carries a clean cite).

Plan continues: no — queue reconciled, one `open` entry pickable, inbox drained; hand to build.
