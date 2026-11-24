# 0078 — one fact, one spelling

- **Date:** 2026-11-24 · **Status:** accepted

## Context

An adopter calls the SDK "a configuration minefield". An audit of the
authoring surface on 2026-11-24 found the cause is restatement:

- A layout's slot names appear in both `content.regions` and `leaves`,
  and nothing checks one against the other.
- The body is described by three knobs: `format`, `content` and `shape`.
- A manifest path is spelled in `locus.glob` and again in
  `collectionAddress`.
- Five dependency spellings are fingerprinted unevenly (GH #61).
- "Required" means four things, and the requirement default contradicts
  `contract.md` (GH #60). This repo's own program writes `required: true`
  on most of its requirements to get the default the spec states.

`process/engineering.md` bars this inside the code: one job, one home,
and derived state is computed. Nothing bars it on the surface the author
writes, which `intent.md` names as the product. Ruled by John 2026-11-24 on the
session's recommendation.

## Decision

**One fact, one spelling** (`model/authoring.md`, "The SDK"). Each fact
the author declares has one place to declare it, and anything the
declared facts already determine is derived. A restatement the type
system binds to its source counts as one spelling (0053's exhaustive
`LeafSet` is the precedent). An unbound restatement is a defect, never an
option. A default the SDK lowers is the corpus's default, never a second
decision.

## Rejected

- **Leave it to engineering.md.** That page governs code shape and the
  posture sweep reads one module neighbourhood at a time. Restatement
  across `kind()`'s fields sits inside one constructor and still passes:
  the surface needs a product rule, not a code rule.
- **Document the knobs better.** Guidance cannot fix two spellings that
  can disagree. The disagreement is a silent failure (invariant 6), and
  the fix is to remove one spelling.
- **The objection to this ruling:** TypeScript erases `T`, so some
  restatement is forced. Only an unbound one is. A runtime witness the
  compiler holds exhaustive is the same fact carried across the erasure.

## Consequences

Each class from the audit becomes a pending entry against this section,
or a fork where the single spelling needs a ruling (#61's dependency
spelling, #64's placement, #65's gate command). The requirement default
is derivable now: the SDK lowers `contract.md`'s fill default.
