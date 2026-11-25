# 0092 — a relationship carries its intent

- **Date:** 2026-11-25 · **Status:** accepted

## Context

An edge is typed and checked for structure: it resolves, its degree is
bounded, its target kind is right. Nothing states what it means. The
lower rungs stay: "an edge carries no obligation of its own", and the
clauses that range over it decide. John, 2026-11-25: with AI authors "the
power comes with being able to dictate the intent for a relationship and
have that reasoned over as the contract set expands." Take his example:
a skill owns code standards. Whether it covers them is undecidable, but
an agent about to add a second standards skill should see that the
domain is already contracted. A model trained on schemas reads an edge as
a foreign key whose meaning must be verified against content, and
invariant 1 forbids that (matching is mining).

## Decision

**What an edge means is its intent, declared and never checked**
(`contract.md`, "edge"):

- an edge field's intent is its field guidance (0091), stated once on the
  kind that declares it;
- a satisfies entry's intent is its requirement's prose.

Intent is delivered at both ends of the relationship, with the declared
requirements and their satisfiers, through 0091's path-scoped rule. An
agent touching a kind's locus then sees what is already contracted
before it adds to the set. The gate keeps the structure honest (an
at-most-one-owner clause catches a second declared owner). The judgment
of meaning is the agent's.

## Rejected

- **Checking intent against content.** That is mining (invariant 1), and
  it is undecidable (intent, "The honest bound").
- **Intent as a new per-edge prose field.** The kind's edge field already
  has guidance, so a second channel would be a second spelling (0078).
- **Delivery through `explain` alone.** Exposed is not delivered
  (invariant 8).

## Consequences

0091's synthesized rule also carries each satisfied requirement's
identity, prose and satisfiers, and each edge field's intent, at both
endpoint kinds' loci. A requirement's prose is delivered verbatim, so it
stays short. That is the author's counsel to follow, never a length gate
(0045).
