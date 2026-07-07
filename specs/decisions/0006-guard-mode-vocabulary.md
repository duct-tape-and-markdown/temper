# 0006 — the guard's mode vocabulary

- **Date:** 2026-07-07 · **Status:** accepted

## Context

The mode recut (0005) landed the enforcement-mode type on the root member
but scoped the values out: the engine enum stayed two-valued
(`{Shared, Surface}`) while the corpus named `note / warn / block` with
`note` undefined and the default unpinned.

## Decision

Three values, split by where the finding goes: **block** denies the tool
call; **warn** allows it and surfaces the finding in-band, into the live
context; **note** allows it and records the finding out-of-band only — the
next report, never the session. The default is **warn**.

## Rejected

`note` as the default — out-of-band-only is functionally silent at the
moment of action, against the invariant that failure is never silently
passed. `note` as a dimmer `warn` (same channel, softer text) — two
adjacent tiers on one channel blur; the channel split is the distinction.

## Consequences

`{Shared, Surface}` recuts to `{note, warn, block}` across its engine
homes; the shipped default maps `Shared`→`warn`, preserving today's
behavior. Session start is untouched — it is `check`'s reporter, not the
guard.
