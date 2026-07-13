The change flow is how anything in this repository moves: intent enters the
corpus, work derives from the delta, and the gate holds the result. It is
the spec-to-code direction of the authority arrow; the reverse direction
exists only as the declared return paths the corpus system names.

## Trigger

A human authors or amends intent — a new system section, an amended
decision, a new requirement — in the corpus.

## Participants

- corpus
- gate

## Steps

1. The corpus change lands as an edit to a governed source document, and
   `temper check` confirms the document still fits its declared layout and
   every address still resolves.
2. Work is derived from the corpus-versus-implementation delta — planned
   from the documents, never invented beside them.
3. Implementation reconciles toward the corpus, and the gate judges the
   result: projections re-emitted, drift routed to owning sources, coverage
   clauses re-evaluated.
