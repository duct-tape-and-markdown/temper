# Parked: no clause can be conditioned on a load moment

The entry predicts only builtins.ts and lock edits, but the vocabulary has no
way to say "where this member loads on demand". The pin (root CLAUDE.md past
40,000 characters draws nothing) and the tests (apps/web/CLAUDE.md and an
unscoped apps/web/.claude/rules/x.md draw the advisory) differ only in
placement. Here is what I checked:

- `when` guards are `enum`/`type` over a field (src/contract.rs `When`;
  contract.md: "one predicate from the scalar-decidable set over an
  addressable field"). Memory has no fields, and an unscoped nested rule has
  no `paths`.
- Selectors are by kind, opt-in or incidence. None selects by load moment, so
  a root-bound clause can't narrow either.
- `on-read-beneath` and `member_home` exist (src/gate.rs, src/kind.rs), but only
  `explain` and reachability read them. No predicate does.
- `rule` registers only `paths-match(paths)`, so a nested unscoped rule's
  on-demand load is not a registration fact.

0115 says the advisories read the registration fact, but nothing says how a
clause spells that. There are two consistent answers, and each is a language
change:

1. (recommended) A guard form over the load moment, e.g.
   `when(loads("on-demand"), [extent("characters", 40_000)])`. This keeps
   conditioning in the vocabulary's one conditioning construct. It needs
   contract.md's guard sentence widened past "over an addressable field".
2. An `extent` argument that bounds only on-demand members. Rejected: it
   hides a guard inside one predicate.

Either way `rule` also has to carry `on-read-beneath` (or the engine has to
derive a nested rule's home) before its load moment is a fact a clause can
read. The rule's `paths` guard would then be one of two on-demand causes.

The sibling SIZE-ADVISORY-AT-LOCAL-MEMORY-AND-UNCLAIMED-IMPORTS is blocked on
this entry and shares the gap.
