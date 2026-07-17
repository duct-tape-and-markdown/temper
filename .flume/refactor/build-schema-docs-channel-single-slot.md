## Surface

The schema docs channel keeps **one** `description` slot per property, so N guided
clauses on one field collapse to whichever is authored last — the other N-1 guidances
are silently dropped from the emitted schema.

- `src/schema.rs:175-185` — the docs-channel loop `insert`s `description` per clause into
  the shared property map; each guided clause on a field overwrites the previous one's
  prose. `insert` is last-writer-wins, and nothing reports the loss.
- `src/contract.rs` `Clause::guidance` — guidance is per-clause by construction
  (`specs/builtins.md`, "The clauses live in code": the teaching "rides the clause value
  itself, so it cannot dangle from the check it explains"). The projection is where that
  property is lost.

Concretely at HEAD+this entry: `skill`'s `description` carries four guided clauses
(`required`, `min_len`, `max_len`, `shape`). Only the last one's guidance reaches an
editor's hover; the spec's 1024-cap teaching no longer does. Adding a clause silently
re-picks which teaching an author sees — this entry hit exactly that, and the choice is
an artifact of author order, not a decision anyone made.

The validation channel next door has no such flaw: `SHAPE-PREDICATE` added
`push_subschema` (`src/schema.rs`) so a second `pattern` composes into the property's
`allOf` instead of clobbering the first. The docs channel is the same hazard, unfixed —
one clause per property's `description`, no accumulation.

## Observed at

94012c4 (HEAD when observed)

## Suggested consolidation

Give the docs channel the accumulation its neighbour just got: join the field's guided
clauses into the one `description` (blank-line-joined in clause order), so every clause's
teaching reaches hover and adding a clause adds prose rather than replacing it. The
alternative — leave one slot and pick deliberately — needs a stated rule for *which*
clause wins; today there is none, and author order decides by accident.
