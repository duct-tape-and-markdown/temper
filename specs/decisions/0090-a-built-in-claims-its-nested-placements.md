# 0090 — a built-in claims its nested placements

- **Date:** 2026-11-25 · **Status:** accepted

## Context

The `nested-command-and-skill-placements` question. Current Claude Code
documentation (code.claude.com/docs/en/skills and /sub-agents, retrieved
2026-11-25) loads three placements the built-ins do not claim: a command
in a subdirectory of `.claude/commands/` (named by its path, `/` as `:`:
`/frontend:component`), a skill under a subdirectory's own
`.claude/skills/` (`/apps/web:deploy`, once Claude works there), and an
agent in a subfolder of the recursively scanned `.claude/agents/`.

0084 rules that a built-in claims what the harness loads. The obstacle is
identity: `/` is the nesting separator (0049). Ruled by John 2026-11-25:
"if claude code supports that, we should."

## Decision

**Each built-in claims its documented nested placements** (`builtins.md`,
"The shipped kinds"). Identity stays inside the one address grammar,
using the `:` that 0074 already admits within a name:

- a nested command is its invocation name, `command:frontend:component`;
- a nested skill prefixes its directory name with its subdirectory's
  segments, `skill:apps:web:deploy`, and the kind renders the invocation
  `/apps/web:deploy` from the placement;
- an agent keeps its `name`-field identity.

A built-in's relocation takes an authored `exclude`, appended to the base
exclusions its template still reaches (0087). That lets 0085's remedy
reach a built-in, and it is how a fixture tree holding harness files is
left ungoverned.

## Rejected

- **`/` inside a name.** It breaks 0049's segment count: `skill:apps/web`
  would read as a nested address.
- **An escape for `/`** (`apps%2Fweb`). An encoding nobody reads, in an
  address every finding prints.
- **Keep the top-level loci and name the nested ones unmodeled.** That
  leaves documents the harness loads ungoverned, which is the gap 0084
  closes.

## Consequences

Not claimed, because the documentation does not state them: skills
nested deeper inside `.claude/skills/`, a subdirectory's own
`.claude/agents/`, and its own `.claude/rules/`. A nested-harness
directory (its own `.temper`) stays fenced from discovery. A mention of a
nested skill renders its invocation spelling. Stem-only command identity
retires, so two `x.md` files in different subdirectories are two
members.
