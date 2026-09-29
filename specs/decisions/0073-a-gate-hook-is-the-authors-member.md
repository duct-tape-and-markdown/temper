# 0073 — a gate hook is the author's member

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(represented-harness-gate-upgrade)` fork. Since install scaffolds
gate hooks as members, re-running it on a represented harness wires
nothing: a missing gate hook answers `Conflicted` forever, and a member
running a stale spelling of temper's command answers
`SupersededByMember`, which install skips. Scaffold writes the commands
as literals, so every represented harness goes stale the day a command
changes; this repo did, and its `PostToolUse` row failed on every Bash
call with `check` silent. Ruled by John 2026-09-29 on the session's
recommendation.

## Decision

- **Install never re-adds a gate hook to a represented harness.** The
  author may have deleted one on purpose. A missing or stale gate hook
  is named with its remedy — the module and the import line.
- **The gate commands have one home, the engine's.** The SDK's copy is
  machine-written across the `generated/` boundary and held by the seam
  gate, and scaffolded members import it rather than spell a literal.

## Rejected

- **Install editing `harness.ts`.** It edits TypeScript the author may
  have restructured, with machinery temper would carry permanently.
- **Writing the modules but not the import.** An unimported module is the
  unreached member `reached-from` indicts; the remedy would author the
  defect.
- **A registry of retired spellings.** It catches the class one release
  late and keeps a graveyard of strings in the engine.

## Consequences

The generator writes the command constants; if the seam gate covers only
types today, it widens to the constants file. `gate_installed` reports a
stale spelling as well as a missing member.
