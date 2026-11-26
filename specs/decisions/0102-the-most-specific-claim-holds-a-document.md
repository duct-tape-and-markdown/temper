# 0102 — the most specific claim holds a document

- **Date:** 2026-11-25 · **Status:** accepted

## Context

Overlapping claims were ruled one at a time:

- 0084: a built-in claims what its loader loads;
- 0085: a collision is judged per document;
- 0094: a subkind over its base;
- 0096: a skill's template over memory's any-depth reach.

The `host-unit-cede-scope` question then asked against whom a host cedes,
and
which hosts cede. The shipped cede applied to built-in hosts only, and emit
recomputed it from base rows, so it missed the host case.

## Decision

**Where claims overlap, the most specific documented claim holds the
document, for built-in and custom kinds alike** (`builtins.md`, opening):

- a subkind holds over its base;
- a host's template holds over an any-depth reach.

The cede is against the any-depth reach that overlapped. For a skill's
directory that reach is memory. Ownership, not privilege, means a custom
host cedes as a built-in does. The lock records each resolution, so check,
emit, the guard and coverage all read the same answer.

## Rejected

- **Ceding against every kind.** It takes files from a custom kind that
  deliberately claims inside a host's unit.
- **Built-in hosts only.** That is a privilege, which 0068 forbids.

## Consequences

Emit's placement reads the cede from the lock and never learns which kinds
are built-in. Future overlaps resolve by this rule and need no new decision.
