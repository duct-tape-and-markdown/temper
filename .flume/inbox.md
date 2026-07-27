<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## 0037's requirement-grain verdict surface never shipped — observed at c3f2a8a9

0037 ruled the verdict question: judgment of a telemetry verifier is
"reading the field record," verdict surfaces are **read verbs**, and no
verifier's result enters check's exit code ("Verdicts in the gate"
rejected, invariant 2). What shipped is member-grain only: the field
strand attaches to a member target in explain (src/read.rs:305-321,
src/telemetry.rs). The **requirements walk narrates no verifier at all**
— satisfier sets and coverage state, never the declared events, never a
log join. So the question the dogfood's own `context-arrives` prose
declares ("judged by reading the field record, never by check") has no
reader: judging it today means grepping tap.jsonl by hand, which is
exactly how both worktree defects in this arc were found.

Gap, concretely: a requirement target carrying telemetry(events) should
narrate, per declared event, the log's counts with denominators —
records, distinct members, distinct sessions — and name the declared
members with zero records, under the field strand's evidence-narrated-
never-judged banner. The zero-hit list is the "dead weight, trigger
quality, placement audit" answer 0037's Context names as the point, and
the declared-but-never-loads signal docs/horizons.md's observed-graph
strand wants. 0046's absence clause composes: empty log, stated absence.

Second, softer: 0037's Consequences also promise "eval selection rides
the impact strand — a member edit names the requirements whose verifiers
went stale." No trace in src/read.rs. Same family, separable entry.

No new decision needed — this is reconciling code against 0037 as ruled;
the narration's exact shape (counts with denominators, zero-hit naming)
is the one design choice, derivable from 0037's Context.
