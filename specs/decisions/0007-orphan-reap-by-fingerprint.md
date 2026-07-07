# 0007 — ownerless projections: reap by fingerprint, report on drift

- **Date:** 2026-07-07 · **Status:** accepted

## Context

Deleting a member and re-emitting left its projection on disk — unowned,
still loading into the agent — with the lock forgetting the path: no reap,
no report (field report 2026-07-07). Detection is decidable (a lock-known
projection with no current owner); the remedy was the fork.

## Decision

Emit reaps an ownerless projection when the file is byte-identical to its
lock fingerprint: temper wrote every byte and the owner is gone, so nothing
authored is lost. A drifted ownerless file is a finding, never a silent
delete. The hash is the safety line.

## Rejected

Report-only (the live hazard persists: a stale projection keeps loading).
Unconditional reap (deletes hand-edited bytes — authored content lost
silently, against the verbatim invariant's spirit).

## Consequences

Emit compares the prior lock's rows against current members and reaps or
reports the difference; "N unchanged" can no longer be the whole story when
an owner disappeared.
