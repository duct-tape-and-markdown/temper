## Symptom

Plan's `continuation marker is honest` gate reverted a **human-authored
`specs:` commit**, and the loop did not notice.

d6381b4 (`specs: review is the price of softening — the harness composes as a
layer stack (0030)`, John, 17:35 — 100-line decision + 34 lines of
`pipeline.md`) was committed on top of df78c05. The gate fired `afterCommit`
with the verdict:

    state.md says `Plan continues: no` but 1 specs/ commit(s) sit past the
    spec cursor a571973

and reverted it. The marker it judged was **not written by that commit** —
it was plan's own `state.md` from df78c05, honest when written and made
stale by the human's next act. The gate read a human `specs:` commit as a
dishonest plan tick and destroyed it.

The evidence at HEAD: `git merge-base --is-ancestor d6381b4 HEAD` is false
while its parent df78c05 is an ancestor; `specs/decisions/` runs 0023…0029
then **0031**. John went on to author 0031 (16ce347) numbering past a 0030
that no longer exists — the revert was silent enough that the next human act
built on top of the hole.

## Cost this tick

The tick itself: ~10 minutes of forensics. `git log a571973..HEAD -- specs/`
reported a **dry delta** — the reverted commit had erased the very input plan
was meant to derive — so this tick oriented on reconciliation, then had to
re-orient when 16ce347 landed mid-tick.

The real cost is John's: one decision record + a `pipeline.md` change, still
recoverable at `git show d6381b4`, but only because the prior-attempt block
happened to name the sha. Nothing on disk records that 0030 ever existed.

Recurrence is structural and near-certain: the gate fires on **any** commit
landing while plan's last `state.md` says `Plan continues: no` — which is the
normal resting state of the loop, and exactly when a human sits down to
author a decision. Every `specs:` commit authored between ticks is exposed.
16ce347 is exposed right now; this tick's `Plan continues: yes` is the only
thing standing in front of it, and that is an accident of timing, not a fix.

## Suggested fix

Scope the gate to the commit it is judging. It is a **plan-phase** gate: it
should evaluate only when the commit under test is plan's own (`plan:`
prefix, or the phase that produced it), never a `specs:`/`chore(harness):`/
human commit that merely moved HEAD past a cursor. A stale marker written by
an earlier tick is not the current commit's dishonesty.

Cheaper stopgap if the scoping is not trivial: make the gate's failure
**non-destructive** for commits it did not author — surface the stale marker
and force the next plan tick, rather than reverting a human's work. A gate
that deletes human-authored corpus commits to enforce an agent's bookkeeping
invariant is inverted; the marker exists to schedule plan, and no scheduling
fact is worth a decision record.
