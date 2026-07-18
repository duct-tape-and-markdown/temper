<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


- RULED (session, standing delegation) — four parked entries, unpark all:
  IMPORT-ROLLUP-WRITER-PLACEMENT: move RollupEntry/write_rollup/rollup_tables to drift.rs —
  the codemap already assigns "the lock" to drift and drift is the sole caller; within-pipeline
  module-job fix, no map edit needed.
  READ-CONTEXT-MEMBER-CITER-GRAIN: consolidate the shared citation-SELECTION into one
  predicate-taking helper; the two narrations stay separate — the grains genuinely need
  different print forms, and a predicate+formatter closure is more indirection than ~30
  shared lines earn.
  READ-VERB-STRAND-COHESION: extract the telemetry `field` strand to its own flat pipeline
  module (shares zero machinery — accidental cohabitation); the four explain traversals stay
  under read's one-verb framing (the Species dispatch is real cohesion). Revisit the residual
  only if it still strains after.
  MAIN-THIN-DISPATCH-COHESION: the invariant stands, main sheds — row→model constructors
  move beside the lock-row reading they interpret, verb bodies to library homes per the
  codemap's job assignments; derive the chain, and a function whose home is genuinely
  ambiguous parks that one question, never the campaign. observed at a8dd112

- NOTE: architecture.md's debt paragraph re-cut this session (three 0040 edges shipped →
  history; normalize_path stays the one declared edge; foundation invariant restated to its
  point — nothing outside foundation, intra-foundation leaf imports admitted). The
  architecture-debt-list friction capture is drained by that edit. observed at a8dd112
