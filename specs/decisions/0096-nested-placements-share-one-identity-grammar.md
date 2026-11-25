# 0096 — nested placements share one identity grammar

- **Date:** 2026-11-25 · **Status:** accepted

## Context

Three plan questions followed from 0090:

- `nested-memory-identity`: a nested `CLAUDE.md` folds to
  `apps-web-CLAUDE`, which emit cannot invert, so every memory member is
  placed at the root;
- `nested-skill-frontmatter-name`: emit wrote a nested skill's identity
  `apps:web:deploy` into its frontmatter `name`, which the skill's own
  `name-matches-dir` and `allowed_chars` clauses refuse;
- `nested-skill-dir-claude-md`: memory's any-depth reach claims a
  `CLAUDE.md` inside a nested skill's directory, while the root case is
  the skill's (0084), and an exclusion is a literal path that cannot
  reach every nested `.claude/skills`.

Ruled by the session under John's standing direction: each follows from
rulings already made.

## Decision

- **A nested memory takes 0090's grammar**, `memory:apps:web:CLAUDE`. It
  is invertible because `:` is reserved in a path segment's identity, so
  emit places it where it was found (`builtins.md`, memory).
- **A nested skill's frontmatter `name` is the leaf** (`deploy`), and
  its identity is the placement prefix plus that leaf. The directory and
  the name keep matching, as the skill contract requires.
- **A built-in host's template claim owns the files under its unit at
  any depth**, over memory's reach. 0084's rule (the documented folder
  owns its files) is generalized by kind, not spelled as an exclusion
  pattern.

## Rejected

- **An authored placement field for memory.** It is a second spelling of
  what the placement already says.
- **The identity in frontmatter.** It breaks `name-matches-dir`, and it
  writes temper's address into Claude Code's format.
- **A `**/`-led exclusion grammar.** 0081 keeps exclusions literal, and
  the host-claim rule needs no pattern.

## Consequences

`INSTALL-LIFTS-A-NESTED-MEMORY-AT-ITS-PLACEMENT` unblocks. 0084's
literal memory exclusions are now derivable from the host claims, which
is a fold candidate.
