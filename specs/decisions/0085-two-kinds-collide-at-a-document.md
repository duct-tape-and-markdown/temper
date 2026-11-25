# 0085 — two kinds collide at a document

- **Date:** 2026-11-25 · **Status:** accepted

## Context

GOVERNS-COLLISION-COMPARES-GOVERNED-PATHS built a static overlap check
over two loci's templates, and it parked. Built-in `memory`
(`**/CLAUDE.md`) overlaps every custom markdown kind: a `docs/*.md` kind
shares `docs/CLAUDE.md` with it whether or not that file exists. The
adopter had no remedy for the finding, because an exclusion names literal
paths and names narrow by clause, never by pattern (0081). The runtime
fact is settled: Claude Code loads a subdirectory `CLAUDE.md` as memory
(code.claude.com/docs/en/memory, retrieved 2026-11-24), so a real
`docs/CLAUDE.md` is memory, and 0084 says the kind whose loader the docs
name owns it. Ruled by the session under John's standing direction
(2026-11-25): act where the solution is clear.

## Decision

**"Two kinds never govern one path" is judged at each discovered
document** (`representation.md`, "locus"). A document that two kinds'
loci both reach is a finding that names both kinds, and it is answered by
an exclusion on the kind that does not own the path. A path no document
occupies collides with nothing. The built-ins keep their own claims
disjoint through their exclusions (0084).

## Rejected

- **Static template overlap.** It fires on hypothetical paths, and for
  `**/CLAUDE.md` against any `*.md` kind there is no spelling that
  answers it.
- **Memory yields to any custom locus.** That silently misstates a file
  Claude Code loads as memory, which 0084 forbids.
- **A filename-level exclude** (`exclude: "CLAUDE.md"` anywhere). It is a
  pattern under a literal-path rule, a second way to spell narrowing.

## Consequences

The collision entry re-scopes from template comparison to the discovered
document set, and its fixtures carry no extra exclusions. The finding's
remedy names the literal exclude. Admissibility keeps the string check it
already has for two kinds whose root and glob are identical.
