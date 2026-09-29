# 0070 — a directive follows the import

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(directive-relation-scope)` fork. The directive primitive binds to
one kind, `memory`, so an `@`-line in any other kind's body is extracted
by nobody. Claude Code documents that "imported files can recursively
import other files, with a maximum depth of four hops"
(code.claude.com/docs/en/memory, retrieved 2026-09-22), so
`CLAUDE.md → @rule → @CLAUDE.md` is a ring the runtime walks and temper
reported green. The docs are silent on an `@`-line in a rule loaded by
its own channel. Ruled by John 2026-09-29 on the session's
recommendation, which reversed the fork's recorded one.

## Decision

**A file an import reaches carries the directives its format executes,
whatever kind governs it**, to the documented hop cap. A file loaded by
its own kind's channel carries directives only where its kind declares
them. Invariant 1 already counts a syntax the target format executes as
a declaration, so the model follows what the runtime runs.

## Rejected

- **Kind-scoping plus an SDK primitive authors opt into.** A harness that
  declares nothing still carries a ring temper reports green — invariant
  6's silent pass, with the mechanism right and the default wrong.
- **Ruling the middle hop out of scope.** The recursion is documented;
  leaving it unmodeled drops a declared fact.

## Consequences

Directive extraction re-runs on every import target within the hop cap.
`graph.acyclic` and `reachable` see the middle hop. An import target that
is not a member stays a path, resolved on raw disk like any path edge.
