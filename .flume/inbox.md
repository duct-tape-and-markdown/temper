<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- PRIORITY / direction change (see spec dacec45). The value is encoding intent through
  meaningful contracts in an organized authoring space — NOT more artifact kinds. Build
  the requirements + satisfies mechanism and the representation surface:
  (a) parse `[requirement.<name>]` (`means`, `required`) in `temper.toml` (`compose.rs`,
      `specs/10-contracts.md` "Requirements and satisfies"). `requirement.` is its own
      namespace, distinct from the `rule` kind.
  (b) carry `satisfies` on the artifact representation — extend the IR + import so an
      artifact's `meta.toml` holds `satisfies = [..]` (`specs/20-surface.md` "Each
      artifact directory is a representation").
  (c) `check`: referential COVERAGE — every `required` requirement has ≥1 artifact whose
      representation declares a resolving `satisfies` link; diagnose unfilled requirements
      and dangling `satisfies`. Decidable; gate stays closed. temper NEVER judges `means`.
  (d) representation fields: also carry `rationale` (the why, G) on the representation;
      edges already exist via the graph. conformance status is derived, never persisted.
- Deprioritize AGENT-KIND (more built-in kinds is the wrong direction under the reframe);
  hold/park it unless a story demands it. Keep entries disjoint or serialized.
