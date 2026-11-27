# 0116 — CLAUDE.local.md, import sizes, and the guard attachment's command

- **Date:** 2026-11-27 · **Status:** accepted

## Context

The loop raised two plan questions. Both were class-covered, and the build's recommendation was sound in each.

## Decision

- **Memory claims `CLAUDE.local.md`** (`builtins.md`, memory; 0090, 0101).
  - The docs load it beside its directory's `CLAUDE.md` and treat it the same way.
  - It is a local-locus template, because it is personal and kept out of version control.
  - An import target that no kind claims is sized through the reference graph at the root, so the size advisory reaches every file the runtime cuts.
- **A playbook's guard attachment runs temper's own `guard`** (`distribution.md`).
  - The `guard` verb reads the edit's path against the area the lock records.
  - A `PreToolUse` matcher selects by tool name only, so the path test lives in the command. Keeping that test in the engine keeps the area in the lock, and keeps the command rendered by temper, as every gate hook's is.

## Rejected

- **Counting members only for the size advisory.** That misses files the runtime cuts.
- **An emitted guard script as a `tool` member.** That ships logic outside the engine.

## Consequences

Two questions are retired. The `guard` verb grows an area mode, fed from the lock.
