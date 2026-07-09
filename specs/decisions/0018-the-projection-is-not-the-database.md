# 0018 — the projection is not the database

- **Date:** 2026-07-09 · **Status:** accepted

## Context

An embedded member's data made a round trip through its own rendering: the
SDK held it typed, serialized it into a TOML fence in the host's body at
emit, and the engine re-parsed that fence at gate/check time to recover
what it already knew when it wrote it. 0012 named a "hand-authored mixed
host" as this fence's other reader — but no such host exists for a custom
kind's embedded members: every `CustomKind` the engine constructs outside
tests comes from a lock row, and a lock row only exists because an SDK
program declared the kind first. There is no path to a fence naming a kind
the engine recognizes without the SDK's involvement first. The persona 0012
built a reader for was the SDK-composed case, mid-thought.

## Decision

An embedded member's facts are declared, carried as lock rows the same way
any other declared fact is (mention, satisfies, clause) — never mined from
its own rendering. Its format becomes writer-only and unconstrained:
nothing composes an extractor from it, so it holds no admissibility bar, no
leniency declaration, no logic prohibition — authorial control over the
projection is total. 0012's "hand-authored mixed host is a source the fold
reads" is retracted: that category has no occupant for a custom kind's
embedded members, so nothing reads a projection back for them, full stop —
0013's rule ("a projection is written and never read for meaning") becomes
uniformly true rather than aspirational for this one case. 0013's
template-format admissibility bars (injective render, declared leniency)
keep their domain over file-locus kinds — a file may still be
hand-authored, so still needs a reader — but have nothing left to govern
for embedded ones; `(format-template-spelling)` resolves: there is no
spelling to give a mechanism no kind still needs.

## Rejected

Keeping the fence as a lenient "source-read face" alongside the new
declared path (demote, not delete): no genuine hand-authored occupant
survives to read it for, so a permanent second intake would solve a case
that doesn't exist. Fixing the round trip instead of removing it — real
lens machinery, put-back and merge alignment: 0013 already rejected lenses
on "no such file exists here"; one does now, but the simpler fix is to stop
reading it, not to reconcile two copies of a fact that only ever needed to
exist once.

## Consequences

`representation.md`'s `kind`/`nesting` and `pipeline.md`'s lock/emit
sections recut to the declared shape; `CustomKind::fold_members`'s
nested-member fold retires with no fallback — no caller survives it;
collections gain declared order for free (the lock row family is
array-of-tables, not a sorted map); a leaf's authored prose may carry
mentions like any member's, since nothing needs to recover them from
rendered text anymore.
