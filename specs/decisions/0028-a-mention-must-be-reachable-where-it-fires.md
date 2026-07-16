# 0028 — a mention must be reachable where it fires

- **Date:** 2026-07-16 · **Status:** accepted

## Context

The `(mention-gate-containment)` fork: a skill's `paths` removes it from
every invocation channel until a matching file is read, so a rule→skill
mention fires only if the rule's own scope falls inside the target skill's
gate — an invariant held by nothing, failing silently. A five-cell headless
probe (Claude Code 2.1.211, haiku, 2026-07-16; conclusions here, transcripts
uncommitted per the surfaces-state-rules discipline) hardened the fork's
premise and killed the session's own counter-position:

- Invoking a gated, unlisted skill by name **hard-errors** (`Unknown
  skill`) — the gate makes a skill uninvocable, not merely undescribed.
- Both broken cells end with the harness misinforming the user: the model
  reports the skill "doesn't exist" / "is not available".
- An always-on rule mentioning a gated skill is the same hard failure
  whenever the session never reads a gate-matching file — and the same
  configuration works when it does, so no static check can split the two.
- A mention is not a duplicate trigger: with the gate open, the listing
  alone drove invocation 1/2; the mentioning rule drove it 2/2. The gate
  provides availability, the rule provides obligation.

Session-argued, human-ruled 2026-07-16.

## Decision

The vocabulary admits **`mention-reachable`**: over each selected member,
every mention edge whose target member registers by `paths-match` must be
reachable where the mention fires. Two diagnoses, one invariant:

- **scoped source, gate not contained** — the source's scope globs are not
  literally contained in the target's gate globs: the mention can fire
  where the target cannot be invoked;
- **unscoped source, gated target** — the mention is actionable only
  inside the target's gate; scope the source to the gate, or ungate the
  target.

The predicate is generic — source scope field and target gate field are
arguments, like `glob-valid`'s field — and hard-codes no kind. Its
**declared leniency**: containment is literal (every source glob appears
verbatim in the target's gate), because true glob-set containment is
undecidable; it false-fires on a semantically contained narrower glob
(`src/**/*.ts` inside `src/**`). That leniency is why the shipped severity
is **advisory** — a check that can be wrong must not block (invariant 2,
invariant 5). The `rule` default contract adopts the clause over `paths` →
skill `paths`, so a bare harness is covered with zero opt-in, and a corpus
that hits the false alarm retunes or drops one clause in its own contract.

## Rejected

- **Engine-side always-on check** (route-resolution class): same coverage,
  no off switch — a known false alarm with no remedy trains authors to
  ignore the gate (invariant 2's "cries wolf" failure). Checks with
  declared leniency are clauses.
- **Opt-in-only vocabulary** (no default-contract adoption): the author who
  makes this mistake is the author who doesn't know the gate nuance and
  will never opt in.
- **No language change, guidance only**: the failure is silent at runtime
  and invisible at check time; guidance doesn't hold invariants, gates do.
- **Two predicates, one per diagnosis**: independently tunable, but the
  vocabulary is a closed set where every word is permanent spend, and the
  need is speculative — a corpus's coarse remedy (drop or soften the one
  clause) exists today. Splitting later is compatible; un-minting is not.
- **Not flagging unscoped→gated** (the session's opening position — "the
  mention is just early; the gate opens en route"): killed by the probe.
  The sad path is a hard failure identical to the contained case's, the
  happy and sad paths are one configuration, and the fix costs nothing —
  a scoped rule drives invocation as reliably as an unscoped one.

## Consequences

The engine's `Predicate` enum grows one variant with its schema surface;
the `rule` default contract gains the clause at advisory with a fresh raw
cite (the hard-error and gating behavior are probe-verified 2.1.211,
2026-07-16 — re-verify against live docs when the clause's own `cite` is
encoded, per `builtins.md`); the frozen built-in lock re-derives.
`model/contract.md`'s "obligation-free" mention sentence is amended in the
same commit — a shipped clause now ranges over mentions, though none
demands one exist. The fork record deletes with this record's commit; plan
derives the build entry.
