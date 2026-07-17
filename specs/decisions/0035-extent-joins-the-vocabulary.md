# 0035 — extent joins the vocabulary, and the family collapses into the algebra

- **Date:** 2026-07-17 · **Status:** accepted

## Context

First real-consumer vocabulary demand on record: centercode (migrating
onto the 0025 surface) declared advisory budgets on its embedded posture
kinds — an orientation capped at ~12 rendered lines, a directive at ~4, a
step at ~3 — and had to withdraw the clauses because no predicate ranges
over a value's size. This clears the evidence bar the war game could not
(simulated demand argues, never rules): the consumer authored the
clauses and was forced to delete them. The budget is decidable — a line
count carries no judgment — and it is the posture grain's one
load-bearing check: a kind that exists to keep skills terse must be able
to say "terse". Session-argued, human-ruled 2026-07-17.

## Decision

**`extent` joins the predicate vocabulary: the rendered extent of the
selected item, in a declared unit — lines or characters.** Measurement
is **render-side**: the bytes the selected item contributes to its
projection as emit resolved them — for an embedded value, its rendered
form in the host's artifact; for a file member, its projection. The
budget a harness author means is context cost, context cost is what the
model reads, and what the model reads is the rendered bytes — source
counts diverge the moment a reference resolves or a render hook runs.
Decidable off the committed artifacts the gate already holds.

**The family is one word riding the existing algebra, not a family of
words.** Extent composes with the axes every clause already has:

- **each-grain** — a per-item budget: centercode's posture ceilings, or
  "no skill's projection over N lines".
- **whole-grain** — the selection's summed extent: an ambient-context
  budget ("everything always-on under N lines") falls out of the grain
  axis for free; no new word, and no fence to keep it out — refusing it
  would take a special mechanism.
- **selection** — which items are budgeted is the selector's job (by
  kind, by opt-in, by incidence), so per-channel or per-domain budgets
  are spelled by selection, never by new predicates.

## Rejected

- **Token-count extent**: the truest measure of context cost and an
  unstable one — a verdict that moves when a tokenizer or model updates
  is a gate that changes its mind without a diff (invariant 2's
  stability, not just its decidability). Lines and characters are the
  stable proxies; the guidance channel can say "a line is roughly N
  tokens".
- **Source-side measurement**: measures what the author wrote, not what
  the harness pays; the two diverge exactly where the model's cost
  accrues (resolved references, render hooks, projection formatting).
- **A budget-predicate family** (value-extent, member-extent,
  corpus-extent as separate words): the grain and selection axes already
  carry every member of the imagined family; minting words for algebra
  positions is the duplicate-surface disease.
- **Count-based budgets under this word**: "at most N embedded steps" is
  cardinality, already spellable; extent is size, not multiplicity.

## Consequences

Plan derives the entry: the predicate in engine and SDK, each-grain and
whole-grain evaluation over rendered extent, units lines and characters,
admissibility for the unit. The inbox demand note discharges into it.
No shipped default contract adopts extent — budgets are authored
opinion, and the shipped defaults stay opinion-free; this is an
author-facing word.

## Amended — 2026-07-17 — extent subsumes max_lines

Plan's routing caught what the ruling missed: the vocabulary already
shipped `max_lines` — source-side, lines-only, each-grain over the
authored body (`features.body_lines`) — and 0035's own measurement
ruling rejects source-side counting for budgets, indicting the shipped
word. Ruled same day: **`extent` subsumes `max_lines`, which retires.**
Two words for "how long is this", split by which side of emit they
count, is the duplicate-surface disease — and for the prose bodies the
shipped defaults cap, emit is verbatim, so the two counts barely
differ. The shipped defaults re-spell (`maxLines(500)` /
`maxLines(200)` become extent budgets over the same selections) — which
supersedes this record's own Consequences sentence "no shipped default
contract adopts extent": the subsumption carries the *existing* cited
budgets across, and what stays true is that no *new* budget opinion
enters the defaults. The deriving entry defines the measured span and
carries the shift in its guidance. Retirement is loud where it lands: a lock still carrying
`max_lines` refuses at load — the closed vocabulary's ordinary
behavior, never a silent migration shim. Plan derives the entry.
