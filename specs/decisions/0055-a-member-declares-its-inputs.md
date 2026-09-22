# 0055 — a member declares its inputs

- **Date:** 2026-09-22 · **Status:** accepted

## Context

A consumer modeled a legacy web app over a committed code snapshot and
needed to know when the code under a modeled claim changed. The only
fingerprint on offer is an include's: `include()` splices the file into a
projection so the lock can hash it. Observed on 0.0.18: five pins
projected 209,554 bytes, 208,622 of them copied code, to buy five hashes.
The drift finding then says "re-emit to reconcile", which blesses the new
bytes without anyone re-checking the claim. Horizon `(declared-input)`;
ratified by the session on John's delegation, 2026-09-22.

## Decision

**A member declares inputs**: files its claims rest on, each a path
resolved relative to the declaring module (as an include is), each
fingerprinted by the lock in the same source-dependency row an include
writes, and none moving a byte into the projection. When an input's
bytes move, `fresh` (0054) judges the member: the finding names the
dependent member and the input, and its remedy is to re-verify the
member's claims against the input, then re-emit. Temper never judges
whether the claim still holds (invariant 8); it routes the author to the
place to look. The grain is the file.

## Rejected

- **A fingerprint on the ground member**: a hash on the code file names
  no dependent, so the finding cannot say whose claim is at risk; and a
  committed, never-emitted member is `(external-commitment)`'s kernel
  question, not this one.
- **An include as the pin**: correct but ruinous, since the projection
  carries a copy of everything it rests on.
- **An input as an edge**: edges run between members, and an input's
  target is a file that need not be one.
- **Line-range inputs**: a range hash false-fires on every edit above it,
  and a range found by matching content is mining (invariant 1).
  Sub-file grain is decidable only between declared tags, which is
  `(code-seam-joins)`'s code kind.

## Consequences

`authoring.md` names inputs beside mention and include; `pipeline.md`
"Drift" gives an input's finding its remedy. Plan derives: the SDK's
`inputs` on a member and its constructor, the lock row (the include
family's shape, no splice), and the remedy text. It rides after 0054,
which makes `fresh` the clause that judges it.
