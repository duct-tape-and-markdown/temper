# 0067 — `check` names a re-rooted harness

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(re-rooted-harness-disclosure)` fork. `check --harness <dir>`
resolves a directory holding a `lock.toml` as a workspace and walks its
parent — deliberately, so a relocated workspace gates the corpus beside
it — but nothing in the run named the root it resolved. A field fanout
walked a 12.8k-entry `/tmp` for 400s per spawn and reported only
`checked 0 members`. `authoring.md` enumerates what `check` announces,
and a resolved root was not on it. Ruled by John 2026-09-29 on the
session's recommendation.

## Decision

**`check` announces the harness root it resolved when that root differs
from the path it was given.** The line fires only on divergence, so the
common run stays quiet. No verdict and no capability changes.

## Rejected

- **Narrowing the workspace branch to a path named `.temper`.**
  `emit --into` already sanctions a relocated workspace.
- **Refusing a workspace spelling at `--harness`.** It makes one flag
  disagree with the positional about which harness a path names.

## Consequences

One announcement line on divergence, pinned by a test that re-roots a
run and one that does not.
