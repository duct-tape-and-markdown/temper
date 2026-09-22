# 0052 — containment is an incidence

- **Date:** 2026-09-08 · **Status:** accepted · **Amended:** 2026-09-22

## Context

The `(containment-selection-family)` fork. The first adopter's contract —
"every rule body carries at least one directive or consult" — is a
per-host floor over the embedded members a body composes, and the
vocabulary could not spell it: selections are by kind, by opt-in, and by
incidence, and a host's containment of its values was not an edge, so
`count` over an embedded kind ranged corpus-wide and `degree` saw no
containment arc. 0004 makes nested members members over which the
contract layer ranges "exactly as over top-level ones"; the host's
admission declares which kinds compose its body, so containment is
declared, never mined. `contract.md` makes adding a selection a
deliberate language change. Ruled by John 2026-09-08.

## Decision

**Containment joins the resolved edge set as a derived incidence
family**: a host carries one edge to each embedded member its body
composes, under the field `contains:<kind>` for the admitted kind, derived
from the admission and the composed members, never from prose. It is an
incidence like any other, so the existing algebra spells the floor. **A
by-incidence selection filters by a set of fields**, not one: a bound
ranges over the union, so "at least one directive or consult" is one
`degree` floor over `contains:directive` ∪ `contains:consult`. Selectors
stay atomic: a field set is one selector's filter, as an edge's target is
one field's set of kinds. **An unfiltered bound excludes containment**:
every embedded member has exactly one incoming containment edge, so
counting it would make the adopter's planned `degree(incoming ≥ 1)` on
invariants vacuous; containment counts only where a filter names it.

## Rejected

- **A new `contains` predicate**: a second machinery for what a selection
  plus `degree` already says.
- **One field per admitted kind with a single-field filter**: spells the
  per-kind floor and not the union the adopter actually wrote.
- **Mining containment from a body's headings**: edges are declared;
  the admission and the composed members are the declaration.
- **Counting containment in every unfiltered bound**: silently flips an
  existing clause's verdict, and a finding that disappears is the loss
  invariant 6 forbids.

## Consequences

`contract.md`: the edge section names containment among the derived edges;
the by-incidence selector filters by direction and a field set. Plan
derives the entries: `graph` emits the containment family from the
admission; `Predicate::Degree` gains a field-set filter and counts
containment only when named; `explain` narrates containment as an
incidence. The `(containment-selection-family)` record deletes; cascade's
contract work unblocks.
