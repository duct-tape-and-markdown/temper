## Surface

Two lock-row→Clause projectors with two predicate-decoding strategies:
src/builtin.rs:80 `clause_from_row(&ClauseRow) -> Clause` (delegates to
`predicate_from_row`, `expect`s) vs src/main.rs:1082
`clause_from_row(&drift::ClauseRow) -> Option<contract::Clause>` (inline
`match` on `row.predicate`, tolerant). Both already share
`compose::severity_from_label` — a half-integrated seam. Two decoders is
the "two check paths" drift risk, and logic in main.rs also violates the
thin-dispatch rule.

## Observed at

0ccba8d

## Suggested consolidation

One row→Clause projector homed beside the predicate enum (the vocabulary
authority); main.rs delegates. The tolerant/strict split, if load-bearing,
is a parameter, not a second implementation.
