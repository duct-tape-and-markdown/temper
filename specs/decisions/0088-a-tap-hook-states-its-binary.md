# 0088 — a tap hook states its binary

- **Date:** 2026-11-25 · **Status:** accepted

## Context

The `tap-hook-binary` question, following GH #67. A telemetry
declaration's synthesized tap hooks render a bare `temper tap
"$CLAUDE_PROJECT_DIR"`. On a harness whose gate hooks state a workspace
binary (0082) and have temper off the PATH, which is this repository's
own case, every tap exits 127 and no telemetry records. The tap has
neither 0082's binary fact nor 0086's anchor. Ruled by the session under
John's standing direction.

## Decision

**A telemetry declaration states the temper binary its taps run**,
`temper` on the path unless declared, and each tap command is anchored at
the project as a gate command is (`pipeline.md`, "Telemetry"). The
binary belongs to the declaration that makes the hooks, as 0082 makes it
each gate hook's own fact.

## Rejected

- **Borrow the gate hooks' binary.** One member would read another's
  field, and it needs a rule for no gate hooks or for gate hooks that
  disagree.
- **A harness-wide binary field.** 0082 rejected it, and nothing here
  changes that ruling's reasoning.

## Consequences

An author who runs every hook on one binary shares one constant across
the declarations, which is one spelling bound by the type system (0078).
The tap command takes the anchored rendering through the engine's one
renderer (0073).
