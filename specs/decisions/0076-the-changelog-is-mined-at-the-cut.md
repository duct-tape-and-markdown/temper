# 0076 — the changelog is mined at the cut

- **Date:** 2026-09-30 · **Status:** accepted

## Context

The 0.0.21 cut found the `[Unreleased]` section carrying two of roughly
thirty shipped changes, several of them breaks an adopter meets (hook
addresses gained the matcher, `hook()` takes handler groups, an empty
contract is declared). Nothing wrote the changelog between cuts: build
ticks never touched it and the session only noted its own work. flume,
built on the same pipeline, settled this: a versioning policy in its spec,
a miner over `build:` commits with a `BREAKING:` body marker, a migration
note per breaking minor, and a short cut recipe in the harness. Ruled by
John 2026-09-30: emulate it as the standard.

## Decision

**The changelog is a release artifact mined from git at the cut**
(`distribution.md`, "Versioning"). `scripts/build-changelog.mjs` drafts
`[Unreleased]` from `build:` commits since the last recorded version; a
`BREAKING:` body line routes an entry under `### Breaking`; the build
prompt asks for that line. A cut with breaks ships
`docs/MIGRATING-<version>.md`. The cut recipe is the `release` rule.

## Rejected

- **A per-commit changelog gate.** Every entry touching one shared file
  serializes fanout (flume measured wave width falling from 3.94 to 1.20
  over 50 replayed queues), and presence proves nothing about content.
- **Session-written entries only.** Today's gap is that failure: the
  session cannot see thirty build ticks' breaks without mining them.

## Consequences

- Build commits carry `BREAKING:` from the next tick on; the 0.0.21 draft
  is curated by reading bodies, since none carry it yet.
- The `release` rule and CLAUDE.md carry the recipe, including re-emitting
  every committed lock after the version bump (0069's stamp moves with it).
