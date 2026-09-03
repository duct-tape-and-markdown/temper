<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at d3848196 (interactive review of 4d7e8127, GH #33 fix) — the
  new `is_scope_contained_in_gate` (`src/graph.rs`) decides subset by
  generating witness paths from the scope glob and matching them against
  the compiled gate set. Matching rides `globset`; the witness generator is
  a hand-rolled segment rewrite that concretizes `*`→`file`, `?`→`x`, and
  strips brackets, and never expands brace alternation, so a scope like
  `**/*.{sql,md}` yields the literal witness `file.{sql,md}` and reports
  uncontained against a gate `**/*.sql` (false positive survives for brace
  globs) while a char-class scope `[!a]*.sql` concretizes to `!a*.sql`
  (unsound witness). Low severity, posture-sweep grade: either widen the
  witness generator to the glob vocabulary `globset` accepts (braces, `!`
  classes) with a table-driven test, or document the supported subset in
  the finding text. Not a release blocker.
