# 0066 — a retitle without a key is a delete and a create

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(nested-member-rename-identity)` fork (GH #51). 0019's consequences
promised that "fingerprints report a rename as a move, loudly";
`representation.md` states only that "an explicit key survives
retitling", and the engine detects no moves. A layout member retitled
without a key silently becomes a new member; renaming a composed value's
key does the same. The model body is current intent and a decision's
consequences are history, so the collision resolves toward the body.
Ruled by John 2026-09-29 on the session's recommendation.

## Decision

**Identity without an explicit key is the slugged heading, so a retitle
is a delete and a create.** The explicit key is the declared way to keep
identity across a retitle. The same holds for a composed value's key:
one identity story for nested rows.

## Rejected

- **Move detection** (same host, kind and leaves, different key). It
  infers identity from content — invariant 1's mining — and a heuristic
  that pairs rows guesses, which invariant 2 forbids.

## Consequences

0019's move-report promise is struck. An edge to the old key still fails
`graph.route` loudly, and the lock diff shows the delete and the create.
