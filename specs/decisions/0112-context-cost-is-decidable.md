# 0112 — context cost is decidable

- **Date:** 2026-11-26 · **Status:** accepted

## Context

The playbook review surfaced costs no harness author can see:
- every model-invocable skill's description rides every request, an entry
  is cut at 1,536 characters, and the listing holds about 1% of the window,
  with the least-used descriptions dropped first;
- an on-demand rule past the per-file limit is truncated;
- a rule's globs and its skill's `paths` can drift apart;
- one constructor fanning out to several files hides what it costs.

These are documented runtime facts (code.claude.com/docs/en/skills and
/memory, retrieved 2026-11-26), and the built-ins already state each
surface's load timing. Ruled by John on the review's result.

## Decision

**Context cost is decidable, and the shipped contracts judge it**
(`builtins.md`):
- an advisory on the skill listing against its budget;
- an advisory on each rule against the size limit;
- an advisory on a rule's globs against its skill's `paths` where both
  trigger one area.

`explain` states, per member, what loads it, when, and roughly what it
costs.

## Rejected

- **Leaving cost to the author's judgment.** The facts are documented and
  the arithmetic is decidable, so it is a clause, not taste.

## Consequences

The numbers live with each clause's cite and move when the docs move. The
emitted reference can summarize a harness's always-on cost.
