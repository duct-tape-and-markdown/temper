<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- BUG (dogfood catch from the second harness): the READ verbs and the graph
  tier disagree on what an edge is. `temper why <member>` reports "declares no
  references" / "no member points at it" for members connected by a built-in
  `routes_to` edge that `graph::check` demonstrably reads and gates on (the
  degree probe fired on those exact edges). `why`/`requirements` appear to
  range only over the surface document's `[edge.*]` clauses, not the same edge
  set the engine acts on (extracted features incl. built-in `routes_to` and
  custom-kind relationships). Fix: the read family must traverse THE SAME graph
  the gate ranges over — one edge set, one source of truth — never a private
  re-derivation. This is the "every placement speaks the corpus's vocabulary /
  one model, every surface" decision (specs/50-distribution.md, the gate
  teaches) violated by the tool's own teaching surface: a read verb that
  disagrees with the gate forks the reader's mental model. Repro: a rule with
  `routes_to: <skill>` in a harness; `why <skill>` must show the incoming edge,
  `why <rule>` the outgoing one.
