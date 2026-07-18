<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Field follow-up (centercode, verifying DISCOVERY-WALK-SHARE): the fix
  landed and halved the standing cost — `check` 41s → 22s measured on the
  16,814-file repo — which confirms the shared-walk mechanism and leaves a
  **22s residual with no named owner**: one gitignore-honoring walk of 17k
  files should cost well under a second, so the dominant cost now lives
  elsewhere (candidates: `scan_locus` re-globbing the shared set per kind,
  per-file reads or hashing on the read side, glob compilation per call —
  unmeasured, do not guess). Fix direction: measure before cutting — a
  synthetic large-tree perf fixture (17k-file shape, the gauntlet's
  sibling) with coarse phase timing to name where the residual goes, then
  the cut the numbers pick. A session open is the product's front door;
  22s is still an adoption tax. observed at ecd27b1
