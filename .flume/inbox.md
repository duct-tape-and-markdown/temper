<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## Centercode dogfood (filed 2026-07-23) — edge-vocabulary gap

9. **The edge grammar cannot express two relation classes the centercode
   harness needs, so they live as hand-authored prose — the exact drift
   class temper exists to kill.** Surfaced by the satisfier-agnostic
   review (prose must not name what an edge already declares):
   (a) **no plugin-citing edge** — `consult`/`reference` target only
   skill/rule/supporting-doc, so the "ask runner / ask cartograph"
   routing idiom in 5 member bodies hard-codes plugin names with no
   derivable rendering; (b) **no reverse-reference/derivation** — a
   supporting doc cannot render "which rule consults me," so ~8
   standards modules open by hand-naming their consuming rule ("The
   `cls` rule holds the invariant; this reference holds the detail").
   Both are candidates for one edge-vocabulary extension; the interim
   ruling on (b) is name-agnostic phrasing (the engineering module's
   form), which loses the concrete pointer a derived rendering would
   keep.
   *observed at 68a15282 (findings in testbed centercode
   landing/phase1-true-up @ 21b7304590, a sha outside this repo).*
