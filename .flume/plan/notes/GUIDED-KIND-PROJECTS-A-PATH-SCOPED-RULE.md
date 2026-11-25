# GUIDED-KIND-PROJECTS-A-PATH-SCOPED-RULE

Shipped SDK-only (`withGuidanceRules`, `sdk/src/declarations.ts`; `emit` and
`compileDeclarations` both read the augmented roster).

- Placement: `rule:temper-guidance-<kind>` at the in-play `rule` kind's locus
  (built-in: `.claude/rules/temper-guidance-<kind>.md`). An authored rule at that
  name refuses at emit. A subdirectory (`.claude/rules/temper/`) was the cleaner
  ownership signal, but a member name cannot carry `/`, and a second kind over
  `.claude/rules/temper/*.md` collides with `rule`'s `**/*.md` (0085), so the
  reserved prefix plus refusal is the placement.
- "Shipped" is facts identity against every `claude-code` export plus `dial`;
  a kind with `relocates` set projects nothing yet (RELOCATION-DECLARES-GUIDANCE).
- Embedded kinds: paths = union of the file globs of every host that templates or
  admits it (recursively); an unhosted one projects no rule (absent `paths` would
  load unconditionally).
- Observed: `paths` cannot express a locus `exclude`, so an `at` kind with one
  over-scopes its rule (advisory, tolerated). A program that never declared `rule`
  now puts `rule` in play once a guided kind exists, so `check` starts governing
  `.claude/rules/` there.
- Dogfood human half: drop the hand rules in `.temper` this replaces, re-emit.
