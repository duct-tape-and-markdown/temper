# 0091 — guidance arrives as a path-scoped rule

- **Date:** 2026-11-25 · **Status:** accepted

## Context

0045 delivers kind guidance through `schema` hover and `explain`. An AI
author does neither unprompted, so it sees counsel only once a clause
fails, and invariant 8's "delivered at authoring time" is not met.
Cascade's design session, an active SDK author, tested this on Claude Code
2.1.332 under flume's launch (`claude -p --setting-sources project`). A
path-scoped rule's full content arrives as nested memory the first time
Read, Write or Edit touches a matching file, once per session, and never
blocks. Its counsel today, such as spec authoring for its domain,
invariant and intent kinds, lives in hand rules because the kinds cannot
carry it. Per-field guidance was deferred in 2026-07 for lack of a
consumer, and now it has one. Ruled by the session under John's direction
to settle this with Cascade.

## Decision

**A corpus kind's guidance, and its fields', projects as a path-scoped
rule over the kind's locus** (`contract.md`, "clause"). For an embedded
kind, the rule's paths are the loci of the hosts that admit it. Field
guidance is keyed by the kind's typed surface, so a field it names must
exist (0078). The runtime does the delivery: once per kind per session, on
first touch, before the write. A shipped built-in that the program only
imports projects nothing. One that the program relocates carries what its
relocation declares.

## Rejected

- **A guard hook that injects counsel on each write.** It is new machinery
  for what the runtime already does, and every write pays for it.
- **Counsel on every write, or only on a failure.** Clause guidance
  already teaches at failure, and repeating it on each write wastes the
  context the counsel is meant to inform.
- **Projecting the built-ins' guidance by default.** Every harness would
  get rules it never asked for. Relocation is the opt-in.

## Consequences

The SDK gains field guidance on `kind()`. Emit synthesizes one rule member
per guided kind, as telemetry synthesizes tap hooks, placed where the
author's own rules cannot collide with it. Counsel past about 1k tokens per
kind is the wrong vehicle, and is better as a skill or a standard.
