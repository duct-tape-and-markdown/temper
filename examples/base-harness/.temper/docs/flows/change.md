The change flow is how anything in this repository moves: intent enters the
corpus, work derives from the delta, and the gate holds the result. It is
the spec-to-code direction of the authority arrow; the reverse direction
exists only as the declared return paths the corpus system names.

## Trigger

A human authors or amends intent — a new system, an amended decision, a new
requirement — in the harness program (or, for the glossary, its source
document).

## Steps

1. The corpus change lands as an edit to the owning `.temper/` module, and
   `emit` compiles it: projections rewritten whole, every reference
   resolved or refused before a byte is written.
2. Work is derived from the corpus-versus-implementation delta — planned
   from the documents, never invented beside them.
3. Implementation reconciles toward the corpus, and the gate judges the
   result: drift routed to owning sources, coverage clauses re-evaluated.
