# 0072 — an empty contract is a declared contract

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(empty-contract-unspellable)` fork. An authored empty clause array
was unspellable: the lock encodes "the author declared nothing" and "this
lock predates the default" the same way, so `expect(kind, [])` and a root
`contract: []` both composed the full built-in contract. `builtins.md`
promises that overriding is array surgery and the built-ins are "never a
privileged form"; the empty array was the one surgery the mechanism
reversed. The spec wins over code on intent. Ruled by John 2026-09-29 on
the session's recommendation.

## Decision

**An empty array is a declared contract with no clauses; the default
applies only where nothing is declared.** The lock records that a
contract was declared, per kind and at the root. An older lock without
the record reads as undeclared under 0024's read-time normalization, so
its default survives the upgrade.

## Rejected

- **Ruling the empty array a no-op.** It makes the corpus's own "never a
  privileged form" false for the surgery an author is likeliest to try.
- **A `none()` clause.** A predicate meaning no predicates is the
  precedence table `builtins.md` refuses.
- **Waiting for an adopter.** The evidence bar governs vocabulary growth;
  this is a collision between the spec and the code, and the spec wins.

## Consequences

A declared-contract marker in the lock, written by every emit, normalized
at read for older locks. Both rows-or-default branches in `compose.rs`
read the marker.
