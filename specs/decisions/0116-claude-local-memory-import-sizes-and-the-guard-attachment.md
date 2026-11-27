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

## Amendment (2026-11-27)

The first text made `CLAUDE.local.md` a local-locus template inside memory's
locus. A commitment class is a whole file locus's fact (`representation.md`,
"locus"), so a committed locus cannot hold a local template. Derive hit that
and tried to record it. `CLAUDE.local.md` is instead its own kind,
**memory-local**, at the local commitment class, which is to `memory` what
`settings-local` is to `settings`: read in place and gated, never emitted.
Twenty-one kinds ship.

Rejected: **a commitment class per template.** That splits one locus fact
into many, and no other kind needs it.
