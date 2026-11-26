# 0113 — AGENTS.md's two gates, placement as a declared path, unfiltered degree

- **Date:** 2026-11-26 · **Status:** accepted

## Context

Three plan questions, triaged under John's direction:

- `agents-md-load-facts-the-spec-omits`: the memory docs
  (code.claude.com/docs/en/memory, retrieved 2026-11-26) state three things
  the spec did not. `.claude/AGENTS.md` loads at launch. A subdirectory's
  `AGENTS.md` loads when "that subdirectory has none of the three
  `CLAUDE.md` files of its own". The project-instructions setting can load
  both files.
- `member-placement-path-fact-surface`: 0109 made placement a path fact
  without saying how it is spelled.
- `unfiltered-degree-counts-satisfies`: once a requirement became one graph
  node, an unfiltered degree counted `satisfies` arcs.

## Decision

- **`AGENTS.md` has two gates, each read from the docs** (`builtins.md`,
  memory, and registration). At launch, any `CLAUDE.md` in the working
  directory or above suppresses it. On demand, only the subdirectory's own
  `CLAUDE.md` files do. Where the project-instructions setting loads both,
  neither gate applies. Memory claims `.claude/AGENTS.md` too.
- **Placement is declared where the templates cannot place the name whole**
  (`representation.md`, "member"). It is carried on the member's lock row
  and never parsed out of identity. A field-derived identity keeps its
  path-derived prefix, so a root and a nested same-named style stay two
  members, as `builtins.md` (output-style) requires.
- **An unfiltered incidence bound excludes containment and `satisfies`**
  (`contract.md`, "selection"): relations the selections already judge,
  each counted only where a filter names it. A selection over the
  requirement kind judges like any kind's.

## Rejected

- **"Or above" for the on-demand gate.** The docs say "of its own".
- **A bare field-value identity.** It would make two same-named styles a
  coincident address.
- **Counting `satisfies` unfiltered.** That makes a requirement's satisfier
  degree vacuous.

## Consequences

The shipped absence gate narrows for subdirectories. The member init gains
an optional location, relative to its kind's locus root.
