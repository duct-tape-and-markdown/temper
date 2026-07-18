# 0041 — when joins the vocabulary

- **Date:** 2026-07-18 · **Status:** accepted

## Context

A build tick's friction capture (drained 73c76ca) named the gap
precisely: the marketplace catalog's `source` union is **decidable but
unexpressible** — the docs type `source` as `string|object`, a string
form must spell a leading `./`, and each object form's `source`
discriminator decides which other fields are required
(code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-16).
The vocabulary has no conditional form, so a catalog Claude Code
refuses outright passes `temper check` clean, and
`marketplaceDefaultContract`'s header carries the hold by name. The
addressing prerequisite already shipped (`FieldPath`, the RFC 9535
subset; `Features::locate`), so the guard's field is addressable
today. Decidable-but-unexpressible and undecidable-deliberately-absent
are different categories, and the corpus must not conflate them: the
first is a vocabulary debt, the second a boundary. Demand is in-tree —
temper's own shipped contract cannot state documented rules, the same
coverage-bar argument that shipped 0036. Session-argued, human-ruled
("yep go") 2026-07-18.

## Decision

**The clause algebra gains a conditional form — `when`.** A clause may
carry a **guard**: one predicate from the existing scalar-decidable
set — initially `enum` (value membership; equality is its one-element
case) and `type` — over an addressable field, plus a **body** of
ordinary clauses. Where the guard holds, the body binds; where it does
not (including the guarded field's absence), the body is silent —
absence stays `required`'s to indict, so no `when` finding is ever
vacuous or forged.

**One binding, and it is the soundness invariant:** the guard and its
body share the guard's address binding. Each element the guard's path
locates is judged independently, and the body's paths evaluate at that
element — a mixed `plugins[]` (one string source, one object) must
never leak an object-form requirement onto the string element. A
`when` finding always names the concrete element address it fired at.

**Bounded on purpose:** no nested `when` — the first logic form stays
one level; the guard set widens only by deliberate addition, the same
bar every predicate crosses.

**Same driver, second (smaller) addition:** the `shape` family gains
`leading-dot-slash` — the relative-path spelling rule the string form
of `source` documents. A variant of the existing closed shape family
("a shared concept is one type"), never a new predicate class.

## Rejected

- **A bespoke discriminated-union predicate**: a one-format special
  case layered on the clause engine — exactly the too-shallow altitude
  `engineering.md` now names; `when` generalizes the mechanism until
  the union stops being special.
- **General boolean logic (`and`/`or`/`not`)**: more logic than any
  documented rule on file needs, and every operator is a place a
  contract gets clever. `when` is the smallest form that states the
  union; the day a documented rule needs disjunction, that is its own
  argument.
- **Waiting for a second union-shaped consumer format**: the driver is
  temper's own shipped default contract — the coverage bar does not
  wait on external demand (0036 precedent).

## Consequences

- `contract.md`, "clause": the atom's shape gains the guard paragraph
  (this decision's semantics — one guard, shared binding, one level).
  The predicate/shape additions themselves need no corpus enumeration
  (the enum in code is the authority).
- Engine: `when` evaluation with the shared-binding semantics; a
  gauntlet cell for the mixed-array case (the binding invariant's
  regression pin); vacuity-honest tests per `engineering.md`.
- SDK: the authoring face gains `when(guard, clauses)`; schema/lock
  round-trip carries the guard (derivation verifies the clause row and
  label shapes).
- `marketplaceDefaultContract` completes the `source` union with
  per-clause cites and retires its header hold; the other two shipped
  contracts naming vocabulary holds are re-examined against the
  widened vocabulary, each resolving to a completed clause or a hold
  that survives honestly.
- The friction capture's residue is fully routed; no fork record
  remains.
