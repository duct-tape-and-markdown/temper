# 0079 — the guard is check at a narrower trigger

- **Date:** 2026-11-24 · **Status:** accepted

## Context

GH #66: after a shell call, the guard reports layout documents as
undeclared when the lock declares them, and `check` agrees they are
declared. A trace on 2026-11-24 found the guard builds its own member
list (`guarded_loci`, `main.rs`) with exclusions `check`'s site
classifier (`gate.rs`) does not make. It walks the tree without
`check`'s ignore rules. Its `fresh` half skips the include, import and
input staleness that `check` judges. 0059 and 0060 settled *what* the
guard binds and *when* it runs, but not that it judges the way `check`
does. Each gap was fixed as reported, and each fix left the next one in
place. Ruled by John 2026-11-24 on the
session's recommendation.

## Decision

**The guard is `check` at a narrower trigger, never a second reader**
(`distribution.md`, "Per tool call"). It judges with `check`'s judges
over `check`'s member enumeration. Every finding it raises is one
`check` would raise on the tree it judges, and any clause it binds it
judges whole. That subset relation is decidable, so a test can pin it
over the gauntlet: on any tree, the guard's findings are a subset of
`check`'s.

## Rejected

- **Patch #66 alone** by teaching `guarded_loci` the `layout_source`
  rows. That closes the one reported gap and leaves the second
  enumeration in place, still missing the ignore rules and the
  source-dependency staleness.
- **The guard runs the full `check`.** 0060 rejected a per-call `check`:
  every clause runs on every call, and its whole-contract findings land
  in a live session. The guard keeps its narrower binding. Only the
  judging is shared.

## Consequences

Plan derives one entry: the guard's locus and freshness answers come
from `check`'s site classification and drift judges, `guarded_loci`'s
private kind filter retires, and a gauntlet test pins the subset
relation at both edges.
