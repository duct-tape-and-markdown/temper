# 0004 — nested members are members, structurally

- **Date:** 2026-07-07 · **Status:** accepted

## Context

The engine holds nested prose content in a bespoke value shape (the retired
genre machinery: a parallel struct folded from fences), while the kernel
says a nested member is a full member with its own kind. Fork: is that a
model statement the bespoke shape already satisfies, or a structural
mandate?

## Decision

Structural mandate. One member type in the model: nested content enters the
member tree as members — own kind, own fields, own prose, own edges —
whatever machinery folds them from the parent's body. The contract layer
(selections, clauses, edges) ranges over nested members exactly as over
top-level ones; that reach is the kernel's payoff.

## Rejected

A vocabulary rename over the bespoke struct — cheap, but selections and
edges cannot range over a shape that is not a member, so the contract layer
stays unrealized inside documents, exactly where mention targets and layer
counts live.

## Consequences

The fold arc: the genre-era value types dissolve into nested members
(extract, kind, read, engine, graph, roster); the embedded locus lands with
it; leaf addressing becomes member addressing.
