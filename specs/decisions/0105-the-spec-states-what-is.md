# 0105 — the spec states what is

- **Date:** 2026-11-26 · **Status:** accepted

## Context

A census of 448 normative claims across the model, `builtins.md` and
`distribution.md` found 310 held by a test or clause, 87 held only by
prose, 36 false and 15 unclear. The false claims fell into three groups:

- **pending:** rulings the code does not carry yet, written in the present
  tense;
- **never built:** designs the code never had, such as formats as
  overridable template literals, layer slot data, a per-member mode, and
  the requirement as a kind;
- **drifted:** claims true once or stated loosely.

`spec-system.md` already said that body text states what is and never
cites decisions, and the corpus broke both rules. John, 2026-11-25: "we
aren't exactly pinning rules about our project, we're maintaining a
decision ledger disguised as a spec."

## Decision

**A claim is true of the code, or marked** (`process/spec-system.md`):

- A ruling not yet built is written *(pending)* at its claim, and the
  release cut that ships it removes the mark, at the cut's reconcile step.
- A design the code never had is cut, or restated as what the code does.
- Each claim's holder is a test, a clause or review, and a review-only
  claim is the queue for a pin.
- Body text cites no decision.

This pass applies it: the format bullet, the root member's mode, the
layers opening, determinism, read-back and kind identity are restated as
what the code does, and the pending claims are marked.

## Rejected

- **Keeping aspiration with a "future" label.** An unbuilt design with no
  entry has no path to the code. It is cut and re-proposed when someone
  needs it.
- **Making every claim a typed invariant now.** That is the target, a
  temper-typed spec with holders that `check` reads, but it needs the kinds
  first. The marks and the census hold the line until then.

## Consequences

The four hard-wired checks become clauses, because intent's "the only fixed
checks are well-formedness" binds. The install report's contract half, the
requirement as a kind, and the script-verifier edge are entries. The
review-only claims with cheap tests file as pin-only entries.
