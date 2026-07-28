<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## Human ruling on plan-read-roster-overview-dead-branch: delete — observed at 477895ab

John ruled (session, 2026-07-27): the dead branch goes. Cut
`roster_overview`, narrow `requirements()` to `name: &str` (drop the
`Option`), and trim the module header's stale "the roster → each
requirement's satisfier set + coverage state" clause to match. No
no-target CLI spelling: both Read-verbs spec sections define `explain` as
narrating one named target, no consumer asks for a roster listing, and the
gate already reports coverage. If a roster listing ever earns a consumer,
it returns as a deliberate decision. The capture's satisfiers_of re-walk
concern is moot with the branch deleted.
