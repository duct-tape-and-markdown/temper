<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- From `.flume/friction/plan-declarations-ts-nul-byte-greps-blind.md`
  (routed, human-agreed): `sdk/src/declarations.ts` carries a literal NUL
  byte in `placementKey`'s delimiter, so grep/rg classify the 715-line
  file as binary and print nothing — a silent zero-hit trap in the seam
  file every kind/clause/row question routes through (it cost a tick six
  wasted calls and nearly inverted an audit conclusion; it reproduced
  verbatim in the interactive session today). Fix as the capture argues:
  swap NUL for U+001F (unit separator) in `placementKey` — identical
  non-collision property, the key is internal and never round-tripped
  through the lock, and the file returns to plain text for every tool.
  One line. observed at 9862b2e
