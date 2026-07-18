<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `(check-residual-owner)` ruled 07-17 (session, John): no perf threshold
  anywhere — the expectation is a **performance-oriented approach, held
  decidably**: engineering.md's new "Cost scale is hoisted, and pinned by
  count" section is the ruling's home (hoist whole-input work once per
  run; pin by work count, never wall-clock; measure-first diagnosis via
  generated fixtures). Both human calls the fork held are answered by it:
  no permanent perf furniture (the fixture is generated, diagnostic, and
  earns permanence only if count-pins need it), no corpus perf bar (the
  field's session experience is the true bar). Derive the diagnostic
  entry for the 22s residual from the fork's own candidate list (git
  archives it at a8bb83d): generated 17k-shape tree, coarse phase timing,
  numbers in the commit body, the cut the numbers pick, landing with its
  count-pin — plus a count-pin retrofit for the already-shipped walk
  share (walks == 1 per run per flavor). observed at 11a2815
