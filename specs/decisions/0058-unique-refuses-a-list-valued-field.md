# 0058 — `unique` refuses a list-valued field

- **Date:** 2026-09-23 · **Status:** accepted

## Context

The `(unique-over-a-list)` fork. A consumer on 0.0.18 and 0.0.19 found a
set predicate silently passing over a list-valued field, which was
reproduced at 4756671c. `duplicates` counts `Selection::values`, which keeps
scalars only, so a `unique` clause over a list decides nothing and exits 0:
invariant 6's silent pass. Its sibling, `membership`, ships a per-element
reading (MEMBERSHIP-READS-A-LIST-VALUED-FIELD) that follows from "drawn from
the satisfiers' values" with no new concept. `unique` over a list has two
defensible readings, and no adopter has asked for either. Ruled by John
2026-09-23 on the session's recommendation.

## Decision

**A list under `unique` is a finding, not a verdict.** Where the judge meets
a list-valued field under a `unique` clause, it reports at the clause's
severity, naming the member and the field and saying that `unique` over a
list is not defined. It never passes, and it never picks a reading. The
refusal fires at judge time, because a field's list-ness is member data and
admissibility runs before any member is read. An author who means a scalar
narrows with a `type` clause. A reading is ruled when a real need names one.

## Rejected

- **Flatten across the selection (plan's recommendation)**: every element
  across the selection must be unique, which is one reading for both set
  predicates, the "one algebra over selections" `contract.md` claims. It is
  the strongest candidate and the first to revisit. It was not taken because
  it also rules that a repeat inside one member's list collides with itself,
  a semantics nobody has asked for.
- **Flatten across members only**: each list is deduplicated first, so two
  members may not share an element. It is equally unrequested, and it gives
  the two predicates different readings of a list.
- **Refusal at admissibility**: unavailable as spelled, since the tier runs
  before member data exists.

## Consequences

No model text changes: invariant 6 already demands the loud outcome. Plan
derives the entry: `duplicates` reports a list-valued field, and a test pins
both the scalar verdict and the list refusal. The `(unique-over-a-list)`
record deletes. It ships in 0.0.20 with its sibling.
