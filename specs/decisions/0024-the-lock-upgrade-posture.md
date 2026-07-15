# 0024 — the lock upgrade posture: read robustly, refuse at the cliff

- **Date:** 2026-07-15 · **Status:** accepted

## Context

Three live incidents asked what an upgraded engine owes a committed lock an
older engine wrote, and the corpus said only "tool-written whole, never
patched" (`model/pipeline.md`, "The lock"): a post-normalization engine
joining old `./`-spelled `source_path` rows against new-spelled owned paths
mass-reaped every live projection silently; pre-fix bare `satisfies` labels
had no stated migration once the wire carried `kind:name`; and an `--into`
re-root of an adopted harness turned the whole projection tree ownerless
and reaped it. A fourth, kin: a harness green pre-0018 re-emitted with its
~57 embedded members silently gone — an upgrade that dropped a declared
layer without a word. Registered as `(lock-upgrade-migration-posture)`
(with `(member-fence-dead-text)`'s loss repro folded in); ruled 2026-07-15.

## Decision

One posture, two halves, every instance hangs off it:

- **Read robustly, rewrite canonically.** Joins over lock rows normalize
  spellings on both sides at read time; a bare label an older engine wrote
  qualifies against the corpus where unambiguous. The lock file itself is
  never patched — the next emit rewrites it whole in canonical form, so old
  spellings heal without a migration step. A true collision remains a
  malformed lock, refused loud (0021's standing posture).
- **Refuse at the cliff.** No upgrade is silent about scale: a reap wave
  that would delete every live projection while emitting nothing, and a
  re-read that drops a whole declared layer the committed lock still
  carries, each refuses with the finding stated. A genuine full teardown is
  an explicit flag the author spells, never a side effect of an upgrade or
  a re-rooted `--into`.

## Rejected

- **A versioned lock with migration steps** — machinery for a problem the
  robust read dissolves; the lock already carries no version negotiation
  and emit already rewrites it whole.
- **Per-incident ad-hoc fixes** — three patches that each re-derive the
  same question and leave the fourth incident open.
- **PR #20's bare total-reap refusal** — right instinct, no spelled escape:
  it would also trip a legitimate teardown with no way through. Adopted
  with the explicit flag.
- **A prose scan for retired member-fence syntax** (the
  `(member-fence-dead-text)` proposal) — scanning body prose for temper's
  own old syntax is invariant 1's matching-is-mining, and any document
  quoting a fence false-positives. The loss it aimed at is the cliff
  refusal's job: the layer's disappearance is loud because the lock still
  declares it, never because prose was mined.

## Consequences

`drift.rs`'s orphan sweep joins on normalized paths; the satisfies-label
qualification lands engine-side with the malformed-collision refusal
intact; emit gains the cliff refusals and the teardown flag. The three
pending entries that declared `dependsOnForks` on this fork unblock.
