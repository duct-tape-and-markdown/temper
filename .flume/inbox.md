<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `emit` overwrites a hand-edited projection with the same report line a source change gets — edit `docs/alpha/two/intents/home.json` by hand, run `emit`, and it prints `emitted  intent  home`, nothing more; the hand-made bytes are gone without a word. Regenerating is right — `specs/model/pipeline.md` ("Total, and write-only": a hand-edited projection "is drift by hash, answered by editing the owning source"; nothing parses a projection back) — so refusing is wrong. The gap is the silence: "Drift" says "A mismatch is never silently reconciled" and that a finding names "the side that moved", and the reap rule already refuses to lose drifted bytes no member owns. Emit's report distinguishes a projection-side overwrite from a source-side re-emit, naming the projection and that its hand-edit was replaced from the owning source. Observed at fca7b4f9.
