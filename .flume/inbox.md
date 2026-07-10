<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `(manifest-authoring-surface)` resolved — Decision 0021 ratified
  (`specs/decisions/0021-manifest-authoring-surface.md`, committed a9f7b9e).
  Encode: delete the open-questions record and derive phase 1 as open
  entries per the Decision's Consequences — hook-first, severable at the
  kind boundary; phase 2 derives behind phase 1. observed at a9f7b9e
