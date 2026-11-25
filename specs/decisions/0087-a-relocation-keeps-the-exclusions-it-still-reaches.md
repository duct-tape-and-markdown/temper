# 0087 — a relocation keeps the exclusions it still reaches

- **Date:** 2026-11-25 · **Status:** accepted

## Context

The `relocated-locus-exclusions` question. `relocate(memory, { governs })`
copies memory's whole `exclude` list (0084: `.claude/agents`,
`.claude/commands`, `.claude/rules`, `.claude/skills`) onto the new
template. Under `docs/**/CLAUDE.md`, none of those paths is matched or
reached, so 0081's load check refuses every one, and relocating memory off
`.` fails to load. The engine's overlay comment says a relocation carries
only its own exclusions, so the SDK and the engine disagree. Ruled by the
session under John's standing direction: act where the solution is clear.

## Decision

**A relocated built-in keeps each base exclusion its new template still
matches or reaches beneath, and drops the rest** (`builtins.md`, opening).
An exclusion records another loader's ownership (0084). That ownership
holds wherever the moved template still reaches the folder, and it has no
meaning where the template no longer does.

## Rejected

- **Drop every base exclusion.** A same-reach relocation (`.` +
  `**/CLAUDE.md`) would then claim `.claude/rules/CLAUDE.md` again,
  undoing 0084's answer.
- **Keep them all.** 0081 refuses an exclusion the template cannot reach.
  That refusal is right: an unreachable exclusion states nothing.
- **An authored `exclude` face on `KindRelocation` now.** It may come
  later. The derived default is needed first, so it cannot replace it.

## Consequences

The SDK's `relocatedLocus` filters the base's exclusions by the 0081
reach test, and the engine's overlay agrees. A test relocates memory both
off `.` and onto the same reach.
