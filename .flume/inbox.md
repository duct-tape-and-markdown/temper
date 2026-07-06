<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

## The SDK front door is UNBLOCKED (John, 2026-07-05) — package published, layout ratified

The gate that held the whole SDK-primary chain is discharged:

- **`@dtmd/temper@0.0.1` is live on npm** (published 2026-07-05, public,
  verified by anonymous `npm view`). sdk/package.json carries the publish-
  ready manifest (6e433f4). 0.0.x are stake/dev publishes; 0.1.0 is reserved
  for the coordinated v0.1 release that pins the engine binary.
- **`(sdk-package-layout)` is RESOLVED in the corpus** (PR #4, merged
  4a3790d): one package — root exports the six-noun core, the first-party
  provider face exports from the `claude-code` subpath
  (`@dtmd/temper/claude-code`). 50-distribution carries the Decision; every
  corpus spelling followed.

Plan derives the chain from the corpus. Fileable threads it should weigh:

- The SDK-side recut to match the ratified layout: `sdk/package.json` gains
  the exports map (`.` and `./claude-code`); the provider kinds/floors move
  behind the subpath face (today `builtins.ts` exports from the root index,
  and its header comment still says `@temper/claude-code` — stale spelling).
- `emit` as the host repo's real lock producer and the gate rewrite (`check`
  reads the committed lock + walks each kind's `governs` locus; the copy-tree
  scratch import falls) — scope recorded under `(inplace-lock-producer)`'s
  DATUM in open-questions, including the demolition remainder it unlocks.
- `init`'s re-shape to SDK-program scaffolding is no longer name-blocked
  (the specifier it writes is `@dtmd/temper`).

Still human-gated, unchanged: the release workflow / engine-binary channel
(per-platform packages, marketplace, signing — PACKAGING-CHANNELS), and the
USPTO screen before launch.
