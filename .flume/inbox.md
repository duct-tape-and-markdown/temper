<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): CORPUS CLASSED — the spec corpus migrated to its class
  directories (1d8448e, per specs/process/90-spec-system.md "Decision: classes
  are kinds"): intent/ (00-intent, 05-model, 55-offering), architecture/ (10,
  15, 20, 30, 40, 45, 50), process/ (90-spec-system). Pending entries and
  open-question notes cite the OLD flat `specs/NN-*.md` paths — refresh every
  citation in your registers to the classed paths this tick (filenames are
  unchanged and unique, so the mapping is mechanical). The `.temper/` dogfood
  now registers three class kinds (`intent`/`architecture`/`process`), each
  binding its own package; the transitional `spec` kind/package is retired.
  Header authoring (entity manifests + satisfies) is deliberately NOT part of
  this — it stays a human ceremony for a later session; do not file entries
  inventing it.

- 2026-07-02 (human): BINDING-QUALIFY's park condition is satisfied — the
  curated kinds moved to kinds/claude-code/{skill,rule} with
  provider = "claude-code" header lines (3cf756b); the nested embeds key
  qualified and gates are green over the live layout. Un-park it.
