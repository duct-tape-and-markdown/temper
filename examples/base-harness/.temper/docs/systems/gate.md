The gate is the checked seam between the corpus and everything derived from
it. `temper check` judges the committed artifacts against the lock; `temper
emit` compiles the authored program into projections and refuses on a
dangling reference before writing a byte. The gate holds structure only —
what a member must declare, what an edge must resolve to, what drift looks
like — and delegates every behavioral question to a named verifier.

## Purpose

Move corpus failure to author-time — and to the earliest layer that can
hold it. A dangling participant is a compile error in the program; a
hand-edit to a rendered document is a drift finding; a superseded ruling
without a successor does not typecheck. Each failure is an error message
at the moment it is made, not a discovery weeks later.

## Invariants

### Read or written never both

Every governed path is exactly one of: a source the tool reads (the
glossary, the `.temper/` modules) or a projection it writes (`CLAUDE.md`,
the documents under `docs/`, the rule files). No projection is parsed back
for meaning; no source is regenerated.

### Structure never intent

The gate checks that declared contracts are filled. It never decides the
corpus is missing something nobody declared; gaps are surfaced, not filled.

### Loud or nothing

A detectable failure is an error at author-time. No path silently
degrades, reconciles, or emits over an unresolved input.
