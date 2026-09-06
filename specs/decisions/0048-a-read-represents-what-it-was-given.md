# 0048 — a read represents what it was given, or refuses

- **Date:** 2026-09-05 · **Status:** accepted

## Context

An external adopter probe (GH #39–#54) found nine defects of one shape:
temper accepted authored input and represented less than it was given,
without a finding — a leading title bound to the first slot with every
later region empty; a collection member's own paragraph in no leaf; a
prose region that could never fill; an undeclared document yielding no
rows; the first of two same-named kinds kept and the second lowered to
nothing; a cast-added edge field inert in frontmatter. Each was read
downstream as a property of the model. Invariant 3 forbids dropping
authored words but scopes the bound to emit; invariant 6 forbids silent
degradation but names a *failure* temper detects — and a read that
succeeds at representing less detects nothing. The gap was between them.

## Decision

**A read represents what it was given, or refuses.** Every authored span
and every declared fact reaches a row, a projection, or a finding — never
nothing. The rule binds three surfaces that were outside it: the layout
reader (every heading and every span is bound to a region, a leaf, or a
finding), the SDK's lowering (every fact of a declared kind or value
reaches a lock row, or the lowering refuses — two declarations under one
name are a refusal, never a first-wins), and resolution (an address names
exactly one thing or refuses; ambiguity is a well-formedness fault, not a
choice). Invariant 6 carries the sentence; invariant 3's scope is
unchanged. A region that can never fill is inadmissible for the reason a
vacuous clause is. The mechanism is a conservation test at each seam:
what went in is what came out, or a finding names the remainder.

## Rejected

- **Nine fixes, no rule**: closes the incidents; the probe finds the tenth.
- **Read-time derivation for undeclared committed sources**: makes the
  reader loud by widening the lock family — the gate would read
  declarations from disk, which `pipeline.md` forbids for committed loci.
  The finding is the fix; the lock family stands.
- **Advisory by default for the reader's findings**: a read that dropped
  authored structure is a wrong model, not an opinion; it is an error, and
  only the *floor* clauses (a region may be empty) are dialable.

## Consequences

`intent.md` invariant 6 recuts to carry the sentence (the file's overrun,
111 against ≤100, predates this and owes its own subtraction). Code: a
conservation test per seam — `layout` (heading tree ↔ reading),
`declarations.ts` (facts ↔ rows), `graph` (address ↔ node) — with the
nine probe findings as acceptance cases; `kindsInPlay` refuses a
duplicate name; a second verbatim prose region, and a field region that
binds a heading carrying sub-headings while a later region stays unbound,
both refuse. The member-class × operation parity matrix is engineering
under this rule, not a decision.
