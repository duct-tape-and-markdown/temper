# 0056 — `reached-from` joins the vocabulary

- **Date:** 2026-09-22 · **Status:** accepted

## Context

`reachable` (0054) is the runtime-load closure: the world, live
registration channels, then `@import` directives, hop-capped. A kind
with no registration counts as always live, so in a user-declared corpus
it never fires. A consumer needed the model's own question: starting from
declared roots and following declared edges, does anything connect to
this member? One-hop `degree` catches the first orphan in a dead chain
and none after it. Second-corpus scope was ruled 09-22: opt-in,
domain-neutral predicates built from existing nouns meet
`representation.md` "Reach". Horizon `(field-reach)`; ratified by the
session on John's delegation, 2026-09-22.

## Decision

**`reached-from` is a predicate**, each-grain over the selection its
clause binds: a member holds when it lies in the forward closure of the
**roots** over the **via** field set. Roots are the satisfiers of a named
requirement, named as `membership` names its target, because rootness is
a role and roles are opt-in. A root holds trivially. Via is a field set
in 0052's sense, so containment and mentions count only when named. The
closure is well-defined over cycles. It ships in no default contract: a
clause demanding every member be reached is the declaration-density
demand invariant 1 forbids unless the author declares it.

## Rejected

- **Folding field edges into `reachable`**: a skill with a dead trigger
  that a live rule routes to would go silent though it never loads, the
  loss invariant 6 forbids.
- **A closure mode on `degree`**: it overloads a local count with a
  global walk, one predicate answering two questions.
- **Roots by kind**: the ceremony is lighter, but a kind cannot express a
  root that is one member among its peers.
- **Demoting edges to plain fields checked by `membership`** (the
  consumer's workaround): the edge leaves the one enumeration, and
  `explain` goes blind to it.

## Consequences

No model text: the enum in code is the vocabulary's authority. Plan
derives: the predicate, its SDK constructor, and its judge beside
`degree`. It rides after two queued fixes: acyclicity scoped to the
import relation (the flows it judges are cyclic), and 0052's field-set
filter (its via set).
