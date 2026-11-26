# SDK root-hosted address folds into memberAddress

- `ROOT_HOSTED_KINDS` moved from `builtins.ts` to `member-address.ts` (string keys
  `tool`, `requirement`; `builtins.ts` re-exports it): the grammar module may not import
  the kinds without a cycle. The keys are now spelled twice (here and the kinds' `name`);
  no test pins their agreement.
- `parseNestedAddress` now reads a bare `root` host, so the writer round-trips: the
  engine's `host_identity` already did (d7312cff). Only tests call the SDK reader.
- `emit.ts`'s `bareFileChildTargets` now also enters root-hosted members under bare
  `tool:<name>`, because the member table keys a tool by `root/tool/<name>` now, and the
  bare edge spelling would otherwise stop resolving.
- Side effect: a tool's `mentionOf`, `satisfies`, `mode`, `include`, and `input` rows now
  key by `root/tool/<name>`. No committed lock carries one, so both locks stay
  byte-identical (built-in lock test and base-harness emit test green).
- Requirement rows still spell `nestedAddress(ROOT_ADDRESS, requirement.key, named)`
  by hand in `declarations.ts`. Those are requirement entries, not `Member`s, so
  `memberAddress` cannot take them.
