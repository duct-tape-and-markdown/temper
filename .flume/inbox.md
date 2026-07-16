<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- Consumer field report (centercode, 07-16, verified on disk this
  session): `ResolvedEmbeddedMemberValue` and `EdgeTargetFacts` are
  exported from `sdk/src/kind.ts` (lines 367/384) but not re-exported
  from the package root — yet their own doc comments name them the
  consumer surface ("the shape a kind's own `render` hook receives";
  "the data a `render` hook selects to spell a reference"). A posture-kind
  author cannot type a render hook parameter without deep-importing
  dist internals. Fix as a closure, not a pair: export from the root
  every type a public hook or constructor signature names, transitively
  (`ResolvedEmbeddedMemberCollectionEntry` and kin included) — an
  exported signature naming an unexported type just moves the trap one
  level down. observed at 58efe11
