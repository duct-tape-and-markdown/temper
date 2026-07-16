<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- **Design proposal, route as one entry**:
  `docs/proposals/posture-recursion.md` — one posture vocabulary recursing
  harness → member → block; one edge noun (reference = declared intent,
  mention = reference with a position, display derived never authored);
  std-lib posture block kinds; the governance package (doctrine + intake +
  contract template) shipped in the box. Frame human-ratified in the field
  07-16 ("go"); rejected alternatives argued in the document, including the
  fragment risk this note exists to prevent — the earlier 07-16
  authoring-model asks (references-without-position, typed-block body
  grain) are absorbed here and must not route as separate entries beside
  it. Consumer evidence base: the centercode corpus's hand-rolled `cite()`,
  eleven inline-template bodies, and both corpora minting posture blocks
  independently. observed at f08ffca
- Horizon note, not a pending ask: **behavior is the unverified half**.
  `check` proves structure; the field campaign's highest-value facts (the
  `skill_listing` herald, `paths`-gate channel semantics) were unknowable
  from structure and cost hand-built headless probes (transcript-verified,
  2.1.210, 2026-07-16). `Requirement.verifiedBy` — the behavioral
  remainder — is in the model, dormant. The third leg (schema → structure
  → behavior: probe fixtures asserting over transcript attachments that a
  rule fires, a gated skill is selected, a reference is followed) is a
  quarter-scale product bet; belongs in `docs/horizons.md` when a human
  carries it there. observed at f08ffca
- Demand-arrived evidence for native skill-package modeling: the 07-15
  directory-sliced-governance ruling deferred a nesting kind "until demand
  arrives." The centercode corpus now runs a hand-rolled
  `supportingDocs(skillName)` factory minting one nested-root kind per
  skill directory (`.claude/skills/<name>/docs`, glob `*.md`,
  registration-free) — two live kinds (`harness-meta-doc`, `platform-doc`),
  each doc reached by a required incoming-degree clause and a typed mention
  from its skill body, with the previously unmanaged
  `platform/reference.md` brought under management by it. Every consumer
  harness with a skill that carries depth re-derives this factory; the
  vendor pattern it models (a skill packages its reference docs, loaded on
  demand) is documented at code.claude.com/docs/en/skills "Add supporting
  files" (retrieved 2026-07-16). observed at f08ffca
- **Review riders on the posture-recursion entry** — same fork, never
  parallel entries; the design-session review record (interactive, 07-16)
  the fork must carry. (1) The positionless reference owes a reader story:
  either every reference renders somewhere (inline or block) or a
  renderless reference is legal gate-only wiring like `satisfies` — the
  design must pick one; "the rendering is the agent's retrieval
  instruction" implies the former (0019, one face). (2) Display derivation
  is a declared projection fact on the kind — built-ins carry theirs in
  their shipped declarations, custom kinds get the same lever — never
  engine-hardcoded (`specs/intent.md`, spine rule). (3) Element C is
  spine-clean only as an adopted-by-import, opt-in package; `install`
  placing doctrine by default is baked judgment, and "correct is undefined
  without the economy" overclaims against invariant 4 — the economy is a
  shipped contract, not the definition of correct. (4) The migration
  sketch forward-references a "native skill-package form" no element
  ships; the 07-15 deferral stands, so the sketch is conditional on that
  fork, not evidence for it. Reviewer position on the demand note above:
  one consumer authored by the product's own human does not meet the
  deferral trigger, and a working page-of-primitives factory is evidence
  the primitive set suffices — route as fork-record evidence, not an
  entry. (5) BLOCKING HUMAN RULING: sequencing. Elements A+B respell the
  SDK authoring surface, `specs/builtins.md`, and both example corpora;
  pre-tag lands churn before adopters exist, post-tag churns the courted
  surface immediately after. Neither is free; the fork waits on the
  explicit ruling (launch gate: `specs/distribution.md`). observed at
  1ded3a8
