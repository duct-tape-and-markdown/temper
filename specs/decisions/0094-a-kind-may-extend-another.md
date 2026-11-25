# 0094 — a kind may extend another

- **Date:** 2026-11-25 · **Status:** accepted

## Context

Cascade's `standard()` is one concept: what holds as a rule, how as a
skill with topic docs, plus typed owner, area, holder and tenets. On the
current SDK its fields gate nothing, and `relocate` adds only edge fields,
so it left the built-ins for a hand-written factory (Cascade's report,
2026-11-25). Admission proved a workaround: a rule hosting a Cascade
`invariant` kind rendered byte-identical prose and gated `holder`. But it
mints a second member the author never meant to be separate. John's model:
a custom kind is an extension of a type the SDK holds. The boundary: the
SDK lets a consumer build its harness as a typed program interfaced with
temper, and never guesses the consumer's vocabulary or shape.

## Decision

**A kind may extend another** (`representation.md`, "kind"):

- a subkind keeps its base's facts, adds fields, edge fields and counsel,
  and governs a locus within its base's;
- every selection of the base binds the subkind's document, so the base's
  contract holds it, with the subkind's clauses added;
- a document there is one member, owned by the most specific kind, never a
  collision (0085);
- the subkind's own fields live where the base's format admits them (a
  body region or the kind's render), never as an undocumented key.

Extending under the base's own name is relocation, so `extend` and
`relocate` are one SDK operation.

## Rejected

- **Admission alone.** It works today, but splits one concept into a host
  and a part.
- **A first-class "concept over surfaces".** Cascade's bar is a third
  instance, and there is one today.
- **Owner as an amend authority temper keeps.** Harness ownership is the
  consumer's rule (John, 2026-11-25).

## Consequences

Cascade's shape becomes `standard` extends `rule` and `procedure` extends
`skill`. A typed edge `serves: procedure → standard` keeps the rule
self-contained, and the procedure derives its paths from its standard.
`relocate`'s delta gains plain fields and a new name.
