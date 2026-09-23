# 0059 — the guard asks the contract

- **Date:** 2026-09-23 · **Status:** accepted

## Context

The `(guard-locus-binding-clause-gated)` fork (plan's sweep of b0665cae).
Since 0054, an undeclared document at a governed locus is a **clause** on
`check`: with no `locus-declared` clause bound, there is no finding
(`tests/acceptance.rs:736`: "a stranger at a governed locus is not a fact
the tool pushes unasked"). `temper guard` pushes it unasked anyway.
`guarded_loci` (`main.rs:685`) builds its locus set from kind facts alone and
reads no clause row, so a write of an undeclared document at any governed
locus binds, and `block` denies it. The corpus said nothing about what the
guard binds; `distribution.md`'s "Per tool call" bullet stated only the
mode vocabulary.

The field case (a platform consumer, 09-23) is latent, not live. Its
`source` kind governs `code/*` inside each page's program: byte copies of
the ASP and SQL files it models, not the live `src/` files, which sit outside
every program root. The guard is not installed there. Both would have to
change for it to bite: install the guard, and every refreshed `code/` copy
and every new file under `tests/` binds, warning each time or denied under
`block`, while `check` reports the same files under a clause the consumer
filters out. Ruled by John 2026-09-23.

## Decision

**The guard asks the contract.** Whether an undeclared document at a governed
locus is a problem is the contract's call; what happens about it is the
enforcement mode's. `guarded_loci` binds the undeclared-document case only
where a `locus-declared` clause is bound. Where one is bound, the declared
mode decides exactly as before. An author scopes the concern by binding the
clause where they mean it, or by dropping it from the root contract, and
the guard and `check` agree either way.

## Rejected

- **Map the clause's severity onto the guard verdict** (`advisory` to
  warn, `required` to the mode): it merges how heavy a finding is with where
  a finding is enforced, and the corpus states no such mapping.
- **Declare the guard a write-time boundary independent of the
  contract**: it writes down a gate that cries wolf and gets switched off
  (`distribution.md`, "Session start"), at the one placement that can deny.

## Consequences

`distribution.md` "Per tool call" states what the guard binds. Plan derives
the entry: `guarded_loci` reads the locus-declared binding and a test pins
both arms. The `(guard-locus-binding-clause-gated)` record deletes. It rides
0.0.20 if it ships in time; the consumer's live case arrives only after the
`input()` path fix, when it drops the `code/` copies.
