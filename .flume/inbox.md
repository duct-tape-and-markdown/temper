<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Decision 0022 (`specs/decisions/0022-glob-validity-joins-the-vocabulary.md`,
  human-ruled 07-15) resolves the `(builtins-coverage-predicates)` fork:
  admit a **glob-validity predicate family** (globs parse under `globset`,
  brace-aware), first consumers the `rule` and `skill` default contracts
  over `paths`; `tools-must-resolve` rejected permanently on invariant 2
  (recorded in 0022 — do not re-file). Work: `Predicate` enum variant +
  schema surface in `src/contract.rs`, the two default-contract clauses in
  `sdk/src/builtins.ts` with fresh raw cites, frozen-lock re-derive +
  tests. The two deferred skill shape predicates are NOT in scope (0022 is
  explicit they need their own design). observed at dc43554
