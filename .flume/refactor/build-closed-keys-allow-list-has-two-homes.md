## Surface

`closed-keys`' allow-list — "the keys this clause's siblings declare" — is read
from two different clause sets, and inside a `when` body they are not the same
set:

- `src/engine.rs:567` (`vacuities`) judges the clause against the siblings it is
  handed, which for a body clause is the **guard's body**
  (`src/engine.rs:276`, `inadmissibilities(&clause.predicate, locus, body)`).
- `src/engine.rs:1430` (`decide`) reads `contract::declared_keys(&contract.clauses)`
  — the **top-level** clause list — because `when`'s element loop evaluates body
  clauses against the whole contract (`src/engine.rs:1704`, `:1715`).

So a `closed-keys` in a body is admitted on the body's declared keys and then
decided against the member contract's, over the element's fields: a body
declaring `url` passes admissibility and indicts `url` as undeclared. Out of
`WHEN-BODY-FENCES-THE-FEATURES-AN-ELEMENT-LACKS`'s class — the element's
*fields* are real, so this is the wrong allow-list rather than a zeroed feature
— and no shipped `when` body carries `closed-keys`, so nothing in the built-in
contracts hits it today.

## Observed at

dee31694 (HEAD when observed)

## Suggested consolidation

One home for the allow-list: give the element loop the body as the sibling set
the judge reads (pass the clause set `closed-keys` ranges over to `decide`
rather than the whole `Contract`), so admissibility and conformance ask
`contract::declared_keys` the same question — or fence `closed-keys` at
`Binding::Element` if the body's keys are not meant to be an exhaustive set at
all.
