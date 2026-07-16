<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- The mention-discovery deferral (ed5bb8e) shipped its SDK half and left
  the engine trusting every lock mention row as emit-resolved: mention
  edges feed only `graph::degree`, never route resolution (`src/main.rs`,
  "resolved once at emit and lifted off the lock" — true pre-deferral,
  false now). Field-verified in examples/base-harness: a skill mention of
  `source:main` emits deferred, rides the lock, `explain` shows the edge
  resolved, and BOTH `check` and `explain` stay green with `src/main.js`
  moved away — the deferred mention is verified nowhere, the silent gap
  the fork's ruling forbade ("the row rides the lock and check owns the
  verdict", pipeline.md "Emit"/"Refusing"). Work: route-resolve mention
  edges at check against the by-kind corpus (discovered members included)
  and fire a graph.route-class finding on a dangling target; explain
  narrates the dangle. observed at 955dc30
