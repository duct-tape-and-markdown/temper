<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- The kind-declaration mechanism is now specced (`15-kinds.md` "A kind definition" +
  Decision "a custom kind is declared data, never engine code"; `40-composition.md`
  "Declaring a custom kind" + its Decision; `20-surface.md` import discovery). This
  is a code↔spec reconciliation, not a fork — the intent is settled. Reconcile:
  (a) RETIRE the built-in treatment of the `spec` kind — the hardwired `src/spec.rs`
  extractor, the unconditional `specs/*.md` import scan, and SPEC-KIND-GATE's
  embedded-`contracts/spec.toml` approach; a custom kind must not ship as a built-in.
  Drop/retire SPEC-KIND-GATE — its shape is superseded (not "parked on a human
  file"). (b) BUILD the mechanism — extraction primitives as composable data +
  `[kind.<name>]` parsing in `temper.toml` (extraction / entities+relationships /
  clause) + custom-kind import discovery. (c) RE-DECLARE temper's own `spec` kind in
  temper's `temper.toml` via the mechanism. (d) The graph foundation already shipped
  soundly (54eb52f, `src/graph.rs`): the harness reference graph + route-resolution
  over DECLARED edges (temper.toml, read off extracted fields, never prose-grep).
  Remaining graph-scope work is `degree`/`acyclic`, reading the same edges. Reconcile
  the shipped edge-declaration shape (`AuthorLayer::edges`) with the specced
  `[kind.<name>.relationships]` so an edge is a kind capability, not a separate
  construct. Keep entries disjoint or serialized (shared-file rule).
  Also refresh open-questions: `(spec-landscape-kind)` is superseded by this
  mechanism; `(model-declaration-format)`'s format is now actually carried, not just
  forwarded.
