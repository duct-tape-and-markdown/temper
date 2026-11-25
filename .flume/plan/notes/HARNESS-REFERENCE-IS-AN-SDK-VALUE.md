# Harness reference ships as `reference()` from the SDK root

- `sdk/src/reference.ts`: `reference({ satisfies? })` returns a `skill` member
  named `temper`, marked in a WeakSet; `withReference` (run in `emit` before
  `withGuidanceRules`) re-renders its description and body from the program.
  `declarations.ts` now exports `kindsInPlay` and `contractedRequirements` (plus
  the `ContractedRequirement` type) as the seam the reference renders from.
- The index is rendered from the *authored* program, so the `rule` kind appears
  only when the program authors a rule. The synthesized `temper-sdk` and
  guidance rules do not put it in play. The next entries' topic files should
  render from the same view so the index lines and the files agree.
- Committed-only means a kind at an `at` locus with `commitment: "local"` is
  excluded, along with its members, the requirements they publish, and their
  satisfier entries. That covers the dial and `settings-local`.
- The index lines link `kind-<k>.md` / `domain-<r>.md` beside SKILL.md. Those
  files do not exist until the next two entries render them, so the links
  dangle until then.
- The mechanics text is ported from `src/bundle.rs` OPERATE_SKILL with two
  changes: an added `temper emit` bullet (the spec names install/emit/check),
  and the `(specs/distribution.md)` cite is dropped because an adopter has no
  specs tree. The bundle literal still stands until BUNDLE-EMBEDS-THE-HARNESS-REFERENCE.
- The description is not truncated. A harness with many domains can exceed the
  skill listing cap, and that is `check`'s to report.
