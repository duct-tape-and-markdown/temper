# Rule globs agree with its skill's paths: shipped as a sibling predicate

Shipped `trigger-agrees(scopeField, gateField)`, a new predicate and not a flag on
`mentionReachable`. It is the converse containment check: the gate's globs must sit
inside the source's scope. It is judged in the same pass as `graph::mention_reachable`
and calls the same `uncontained` witness with its arguments swapped, so no second glob
comparer exists. The rule default contract carries it as an advisory. The skill
contract does not: the entry scoped this to rules.

Observations:
- Incomparable glob sets (each covers paths the other misses) draw two advisories, one
  per half: `mention-reachable` and `trigger-agrees`. "Broader" draws only the forward
  one and "narrower" only the converse, so "differs ⇒ one advisory" holds for the
  nested cases but not for incomparable ones. If plan wants one finding there, the
  fix is to merge the two halves into one predicate. I did not invent that.
- `the_rule_floor_is_silent_when_the_scope_is_a_structural_subset_of_the_gate` (rule
  `database/x/**/*.sql`, skill `**/*.sql`) now draws a `trigger-agrees` advisory by
  design. The test only greps for `mention-reachable`, so it stays green.
- A predicate addition touches about 20 exhaustive matches across src/ and tests/,
  plus the `VOCABULARY` slot count and the `floor_quads` column read in
  lock_declaration_rows.rs. That is the cost of the closed algebra, but nothing lists
  those sites for the next addition.
