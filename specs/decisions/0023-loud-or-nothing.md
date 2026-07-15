# 0023 — loud or nothing

- **Date:** 2026-07-09 · **Status:** accepted

## Context

Fail-loud was stated six times in the corpus and ruled zero times: kind
narrowing "a finding, never a silent exclusion" (contract.md), a drifted
ownerless file "never a silent delete", nondeterminism "a loud emit
failure", the emit refusing clause, drift "never silently reconciled",
install "no half-scaffolded state" (pipeline.md). Each line was written
when its surface was specified; nothing bound the next surface. The gap
this predicts shipped: EMBEDDED-KIND-RENDER-HOOK (3c6f50b) gave a kind's
`render` hook the raw embedded value, bypassing mention resolution — a
dangling mention that refuses loudly on every other path silently
stringifies through the hook. The hook's local sanction ("writer-only,
unconstrained — no admissibility bar", representation.md) is true of its
rendering having no reader, but read alone it licensed skipping the
refusing clause one file over. Two locally-true sentences; the misleading
one won because the binding one was per-surface.

## Decision

Intent gains a sixth invariant, the generator the six lines are
consequences of: a failure temper can detect is an error message at
author-time; no path silently degrades, deletes, reconciles, or emits over
an unresolved input. The per-surface refusal clauses stay as the binding
spec text at their surfaces; the invariant makes "where does this path
refuse?" a mandatory question for every new surface, the way "is this
decidable?" already is. A surface without a refusal clause is a gap to
surface, not a license.

## Rejected alternatives

- **Patch the instance only** — extend pipeline.md's refusing clause to
  enumerate the render hook. Fixes the known path, not the class; the next
  surface starts unbound again.
- **Leave it as problem framing** — intent's problem statement ("the author
  learns of the failure from the agent's behavior, not from an error
  message") already implies it. Implication is what the render hook slipped
  through; the invariant list is what spec deltas are checked against.
- **Objection recorded against adding:** a seventh statement of a
  six-times-stated rule is drift surface, and the corpus holds one home per
  rule. Answered: the six are consequences at their surfaces, not
  restatements — the missing piece was the rule they derive from, and a
  generator plus its instances is one rule in one home.
