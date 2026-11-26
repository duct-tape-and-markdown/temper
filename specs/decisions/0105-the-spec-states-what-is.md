# 0105 — the spec states the product's intent

- **Date:** 2026-11-26 · **Status:** accepted

## Context

A census of 448 normative claims across the model, `builtins.md` and
`distribution.md` found 310 held by a test or clause, 87 held only by
prose, 36 the code does not meet, and 15 unclear. John, 2026-11-25: "we
aren't exactly pinning rules about our project, we're maintaining a
decision ledger disguised as a spec." A first pass restated unmet claims as
what the code does and marked rulings *(pending)*. John corrected it the
same day: "we're triaging by keeping the spec area as a representation of
the product; whether the code represents that is the loop's job." This
record is amended to that ruling, before derivation.

## Decision

**The spec states the product's intent, and conformance is the loop's job**
(`process/spec-system.md`):

- A claim the code does not meet is an entry for the loop, never an edit
  to the claim and never a mark in its text.
- A claim is cut only by a ruling that it is no longer intent. This pass
  cuts one: formats as overridable template literals with an admissibility
  bar, superseded by the closed format set, layouts and render hooks.
- Each claim's holder is a test, a clause or review, and a review-only claim
  is the queue for a pin.
- Body text cites no decision.

## Rejected

- **Restating unmet claims as the code's behavior.** It deletes intent and
  turns the spec into a description of the code.
- ***(pending)* marks in the text.** They put conformance status into the
  spec and need a hand reconcile. Gaps belong in the queue.

## Consequences

Plan needs a reconcile mode: derive reads spec diffs from a cursor, so an
old claim the code drifted from is never re-checked. The census is that
check run once, and the loop should run it. The unmet claims that remain
intent are entries: the fixed checks become clauses, a per-member mode,
layer slot data, loud program nondeterminism, typed kind identity in edges,
the typecheck before the program, the unrepresented modeline, the install
report's contract, `package.json` never rewritten, the script-verifier
edge, and the no-lock guard (0103).
