# 0057 — a `when` body clause is an ordinary clause

- **Date:** 2026-09-22 · **Status:** accepted

## Context

The `(guard-body-address-and-severity)` fork (plan's post-ship sweep of
c5093c52). `contract.md` "clause" calls a guard's body "a body of ordinary
clauses", and an ordinary clause carries its author's declared severity and
an address every finding prints. A body clause had neither. `decide`'s
`When` arm judged the body predicate alone, so every body violation reported
at the host guard's severity, and a severity the SDK let an author spell was
dropped. `stamp_clause_label` stamped a body row owner-less where it gives a
requirement's nested row its owner. So the shipped lock carried
`required.source.url` twice, and `clause_collision_diagnostics` never
descended into a body to refuse it. Ruled by John 2026-09-22 on the
session's recommendation.

## Decision

**A body clause is an ordinary clause.** Its label's owner is the host
clause's label, the requirement-nesting rule applied one level down, so
`marketplace.when.plugins[*].source.source=url.required.source.url` names
exactly one clause. It is deterministic, legible, and copied from the finding
that prints it. A body violation reports under the body clause's label at
the body clause's declared severity. The collision check descends into
bodies, and the dial reaches each body clause by its label. "Guard and body
share one address binding" (`contract.md`, "clause") means the **element**
binding: each element the guard's path locates is judged, and the body's
paths evaluate at it. It does not mean a shared clause label.

## Rejected

- **A body as the host clause's interior**: the host's severity and label
  govern, and the SDK narrows the body type so no severity is spelled. It is
  simpler, but it loses per-rule dialing and contradicts "a body of ordinary
  clauses".
- **Host label, body severity**: two clauses under one address at
  different severities, which is the undialable pair `clause.label-collision`
  exists to refuse.

## Consequences

No model text changes: the corpus already says "ordinary clauses", and this
record fixes which "address" the binding sentence means. Plan derives:
- body-label stamping under the host label;
- the body's severity honored in `decide`'s `When` arm;
- the collision walk descending into bodies;
- the builtin lock re-emitted, so `required.source.url`'s twins part.

It carries a release note, because every body clause's label changes
spelling and a dial entry naming an old one stops matching. The
`(guard-body-address-and-severity)` record deletes.
