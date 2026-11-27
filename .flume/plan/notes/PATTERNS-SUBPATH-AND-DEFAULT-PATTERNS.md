# PATTERNS-SUBPATH-AND-DEFAULT-PATTERNS

Shipped. `sdk/src/starter.ts` is now `sdk/src/patterns.ts` at the `./patterns`
export; `starter()` is `defaultPatterns()`. The composed types are renamed
`Starter` -> `DefaultPatterns` and `StarterOptions` -> `DefaultPatternsOptions`.
The spec names only the function, so build picked these names. PLAYBOOK-SHAPE
should keep them or rename them on purpose.

Human halves outside the fence:
- the dogfood `.temper/harness.ts` still imports `starter` from
  `@dtmd/temper/starter`. It breaks against this SDK until someone edits it
  and re-emits.
- the release cut owes a migration-note line for the rename.
- `examples/base-harness/.temper/memory/CLAUDE.ts` (and its projection
  `examples/base-harness/CLAUDE.md`) still calls the example "A starter
  harness". That is memory content, so I left it; the README's opening line
  now says "A base harness".

Test-title matching: the runner's nextest filter joins a line's words into
a regex, so `defaultPatterns` has to appear in the fn name as the
contiguous `defaultpatterns`, not `default_patterns`. A camelCase identifier
in a `tests[]` line produces a slightly awkward snake name. Plan can avoid
this by writing such titles as separate words.
