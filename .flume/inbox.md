<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- 0020's edge-slot consequence is half-shipped by an honestly-stated scope
  cut — filed as the remainder, same-day. The decision says an edge slot's
  entries are "derived to ordinary edge rows — `satisfies` among them";
  LAYOUT-EDGE-SLOT (670d54e) shipped the satisfies half and its commit
  body names the boundary: satisfies is "the sole edge field with a lock
  family to lower into," so a kind's own declared relationships are
  deliberately excluded at the lowering (`src/drift.rs` ~625,
  `edge_fields = {SATISFIES_EDGE_FIELD}`) — a Depends-on-style slot reads
  as a verbatim field section, visible and inert, no silent drop. The
  remainder is entry-shaped: a member-authored edge-row lock family
  (member/field/target — today's `[[declaration.assembly]]` edge facts are
  assembly-scope, no member-authored home), the lowering widened to the
  kind's declared relationships, and the graph's route-resolution ranging
  over the new family. Real customer waiting: a counterpart corpus's
  `Depends-on` field sections (relayed 2026-07-10; their fallback costs
  nothing meanwhile). Observed at e30e5a6.
