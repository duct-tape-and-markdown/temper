# 0120 — a guard may read the load moment

- **Date:** 2026-11-27 · **Status:** accepted

## Context

SIZE-ADVISORY-AT-SUBDIRECTORY-MEMORY-AND-NESTED-RULES parked. The advisory must fire on a subdirectory's `CLAUDE.md` and on an unscoped nested rule, but not on the root `CLAUDE.md`. Those cases differ only in where the member loads. 0115 made the load moment a registration fact that the advisories read, but no clause could spell that reading. A guard read only a field, and memory has none.

## Decision

- **A guard may read the member's load moment**, at launch or on demand, as a registration fact (`contract.md`, guard). An example is `when(loads("on-demand"), [extent(…)])`. The language's one conditioning construct carries it.
- **A subdirectory's own rules register on a read beneath its directory** (`builtins.md`), as its memory does. An unscoped nested rule's on-demand load is then a fact a guard reads.

## Rejected

- **An `extent` argument that bounds only on-demand members.** That hides a guard inside one predicate.

## Consequences

The parked size advisory and its blocked sibling, the advisory over local memory and unclaimed imports, both unblock.
