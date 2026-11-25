# 0095 — review is a verifier

- **Date:** 2026-11-25 · **Status:** accepted

## Context

A requirement delegates the behavior temper cannot decide to a verifier.
Two species ship: script and telemetry. Many real invariants are held by
neither. Cascade's "Comments say only what types can't" is held by "the
read": a reviewer, or the agent as it writes. With no species for that,
"held by review" and "nobody holds this" are the same empty slot, and a
gap model that flags the empty slot flags every honest review. Intent
already names the reader: the verifier invariant 2 delegates to "is,
first, the author itself" (invariant 8). Ruled by John 2026-11-25: review
"is a consequence of the requirement model".

## Decision

**A review is a third verifier species** (`contract.md`, "requirement"):
the reader whose judgment holds the requirement, named in prose. Like the
other species it is declared, never performed, and its verdict never
enters `check`. A requirement with no verifier is **unheld**. `explain`
and the emitted reference (0093) name it. Making unheld a finding is a
clause the author attaches, never a default: no check may demand
declaration density (invariant 1).

## Rejected

- **Leave it to the consumer's vocabulary** (Cascade's `holder` field).
  Whether a requirement is held at all is a fact of the requirement model.
  Only who the reader is belongs to the consumer.
- **Unheld as a shipped finding.** It would demand a declaration on every
  requirement.
- **A review that names a person or role as data.** Authority and
  ownership are the consumer's (0094). The reader is prose.

## Consequences

The SDK gains `review(prose)` beside `script()` and `telemetry()`, and the
lock gains a verifier species row value. The reference and `explain`
render "held by review: …" and "unheld" distinctly.
