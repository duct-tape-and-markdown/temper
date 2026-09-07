<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


## lock: a second `[[declaration.assembly]] fact="edge"` row for the same from/field cross-wires instead of refusing — observed at 67abe15f

From cascade-integrations' probe of 67abe15f, a mis-staged fixture rather
than a defect claim: two `[[declaration.assembly]]` rows with
`fact = "edge"` for the same `from`/`field` but different `to` sets are
both accepted, and the existing edges cross-wire — old cites suddenly
"resolve to no directive artifact", the new one "no skill artifact" — where
`specs/model/representation.md` ("member") makes a coincident declaration a
malformed lock. The SDK probably never writes that shape; the reader
accepts it silently. Verify at HEAD how edge rows are collected (a push
onto a list vs an insert keyed by from/field) and whether admissibility
has a coincidence rule for assembly facts the way `nested_member_
coincidence` now does for nested members; if not, it is one small entry on
the same rule id family, or a debt line if the corpus already excludes
hand-authored locks from that bar.
