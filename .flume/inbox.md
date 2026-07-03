<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): DIRECTIVE-MEMBERS-ALL-KINDS — the at-import pipeline is
  live end to end EXCEPT collection: `collect_directive_members`
  (src/main.rs:787-794) ranges over skill_units/rule_units/custom_kinds and
  misses the other BUILT-IN kinds' members, so a CLAUDE.md member's extracted
  directives never reach `graph::classify_directives` and an unbacked import
  draws no finding. Same hardcoded-pair legacy CHECK-MEMBERS-ALL-KINDS
  (e92bf62) fixed for clause dispatch — fix collection the same way: range
  over the generic per-kind member map. Repro: harness with CLAUDE.md
  containing `@docs/missing.md`, import + check (or `check --harness`) —
  expect one unbacked-pointer finding on the memory member; today: silence,
  exit 0. Verify the wedge path collects too.
