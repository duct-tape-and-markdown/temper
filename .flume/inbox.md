<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

## `(explain-target-disambiguation)` — RESOLVED (John, 2026-07-04, interactive session)

**Ruling: bare target + collision error.** `explain <target>` takes one
positional. The engine resolves the string across all target namespaces
(member, requirement, leaf-grain address, neighborhood):

- exactly one hit → explain it;
- multiple hits → **error**, listing each match's qualified spelling
  (`member:foo`, `requirement:foo`, …) for the retry;
- qualified prefixes are always accepted as an explicit spelling.

Never guesses, never silently prefers one namespace. Chosen over
always-qualified (ceremony on the ~95% unambiguous calls) and
explain-all-matches (a collision silently doubles the output). The ethos is
git's ref disambiguation: cheap in the common case, explicit on ambiguity.

Plan: resolve the fork in open-questions.md, drop the `dependsOnForks` hold on
EXPLAIN-UNIFY, and encode the resolution mechanism into the entry. If the
20-surface.md "one read verb" Decision wants a one-line encode of the
mechanism, route that back as a human corpus delta rather than editing specs.

## MANIFEST-MACHINERY-RETIRE bail — session concurs (2026-07-04)

The voluntary bail in `prior-attempts/manifest-machinery-retire.json` is
sound: the entry is mis-sequenced (no lock producer survives the cut; the
committed lock carries zero declaration rows; temper's own repo has no SDK
assembly producer; RequirementRow lacks set-scope facets + per-member
`satisfies`). Re-scope per its recommendation — a predecessor entry along the
lines of GATE-READS-LOCK-ASSEMBLY, then the retire becomes a clean deletion.
The bailing agent's proposed in-repo producer question (SDK `harness.ts` for
temper's own dogfood vs committed lock rows) overlaps the session's pending
fence-side migration — if plan needs the dogfood assembly authored SDK-side
to proceed, say so in state.md and the session will hand-author it as the
`chore(harness)` half.
