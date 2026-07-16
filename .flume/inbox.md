<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- Ruled 07-16 (interactive), resolving `(supporting-doc-reach-clause)`:
  the reachability check is a **clause, never a requirement** — the vendor
  fact is universal (every supporting file is invisible unless SKILL.md
  references it; code.claude.com/docs/en/skills "Add supporting files",
  retrieved 2026-07-16), so the spelling is the by-kind universal binding
  with each-grain incidence `contract.md` already states, advisory per
  0027, in `supporting-doc`'s default contract. The reach is **any
  resolved edge from the host skill, locus-agnostic** (0025: one noun, one
  degree semantics; mention stays obligation-free by default — this is the
  "a contract may"). No decision owed: no vocabulary change, code lags
  explicit spec text. Two entries: (1) **Indeterminate is never silent** —
  an admissible clause the engine cannot evaluate over its selection is an
  admissibility error or a finding, never a green pass
  (`src/engine.rs:506-509` returns `Outcome::Indeterminate`, `engine.rs:216`
  drops it; invariant 6); land this first so the gap can never recur
  silently. (2) **Predicate generalization** — `count`/`unique`/
  `membership`/`degree`/`kind` move off the requirement facet onto the
  selection algebra (`contract.md`, "selection"; the recut
  `src/engine.rs:492-504`'s own comments name); the reach clause then
  spells and fires. observed at 688d8cc
- For the skill default contract and the market-formats digest: the vendor
  documents a **concrete SKILL.md budget** — "Keep `SKILL.md` under 500
  lines. Move detailed reference material to separate files." — and that
  loaded skill content **stays in context across turns** ("every line is a
  recurring token cost"), both at code.claude.com/docs/en/skills
  (retrieved 2026-07-16). The budget is decidable and citable: a clause
  candidate for the skill default contract (advisory per invariant 5,
  strictest-documented-profile posture). The stays-in-context fact is
  vendor backing for the prose-economy guidance and belongs in the digest
  beside the paths-gate cite. observed at 688d8cc
