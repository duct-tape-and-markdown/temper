# 0117 — a lock from another engine is stale, never malformed

- **Date:** 2026-11-27 · **Status:** accepted

## Context

Two incidents in one day showed the same class:

1. **The session gate went red on this repository.** The installed release engine could not read a lock that main's engine had written (`governs_glob` had become an array). It died on "not the expected string" instead of saying that a different engine wrote the lock. Both engines also stamped `0.0.25`, so the stamp could not tell them apart.
2. **LOAD-MOMENT-IS-A-REGISTRATION-FACT parked.** A committed lock still held the old snapshot of the built-in memory and skill rows. Admissibility read a built-in-named row whose registration differs from the embedded built-in as a collision. Every adopter would hit this on an upgrade that moves a built-in fact, and the collision text is false for them.

The intent was already stated: "a gate run by a different engine says so", and a finding is true for its reader (0103).

## Decision

**A lock written by another engine is stale, never malformed** (`pipeline.md`, "The lock").
- The stamp tells an unreleased build from the release it follows.
- An engine reads the stamp before any row, so a lock it cannot read is reported as written by another engine.
- A built-in's row snapshotted by another engine reads as the embedded built-in, with a finding that the lock wants a re-emit.

## Rejected

- **Widening the parked entry's fence to `.temper/lock.toml`.** That fixes this repository and leaves every adopter's upgrade reporting a false collision.

## Consequences

LOAD-MOMENT-IS-A-REGISTRATION-FACT unblocks once the stale-row reading ships. An adopter's upgrade path is a re-emit, as the finding says.
