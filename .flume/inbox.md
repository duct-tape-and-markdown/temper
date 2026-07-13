<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- PACKAGING-CHANNELS partially shipped in-session (John's ruling, 07-11):
  channel 2's linux-x64 + win32-x64 first cut is live — release.yml builds
  and idempotently publishes @dtmd/temper-{linux-x64,win32-x64} then the
  SDK; @dtmd/temper@0.0.7 carries the launcher (sdk/bin/temper.js) and
  exact-pinned optionalDependencies; verified by a no-cargo scratch-dir
  `npm install` + `npx temper check --harness`. Deliberate deviation from
  the entry's filed shape: the launcher + optionalDependencies live on the
  SDK package per the spec's own text ("pinned by the SDK",
  specs/distribution.md, "What ships") — the root package.json stays the
  private flume manifest untouched; the entry's edit-file claim is stale.
  The entry as filed is SUPERSEDED, not re-scopable (John, 07-11): both
  its file claims are dead — `.github/workflows/release.yml` now exists on
  disk (so the `new` claim fails the refs gate the moment the entry
  opens), and the root-package.json launcher was ruled out by the spec
  text. Retire it and file the remainder fresh: darwin binaries (Apple
  notarizing, on John), channel 3 (plugin + marketplace.json), standalone
  tag assets (workflow path exists, unexercised until a tag), and the
  binary self-reporting its crate version (0.1.0) while npm carries
  0.0.x — lockstep lands at the v0.1 tag. observed at 56012d0

- BASE-HARNESS DOGFOOD, three product findings (observed at 4e111af; all
  reproduced in examples/base-harness):
  1. The SDK-phase fill check cannot see layout-derived `satisfies`: a
     `required: true` requirement whose fills live in layout documents'
     edge sections refuses at `emit(program)` (sdk/src/emit.ts) before the
     engine ever reads the documents. The two-phase fill check needs to
     defer (or delegate) to the engine when the requirement's kind is
     layout-content. (The example no longer carries the workaround — its
     doc kinds went composed, fills program-side — but the gap stands for
     any layout-fill corpus; repro: give a layout kind a `satisfies`
     region and require the requirement.)
  2. `temper install --yes --dry-run` on an already-represented harness
     previews `reaped` for every live projection while the same report
     lists them `unchanged` — contradictory, and if the real run reaps,
     destructive. Not run for real; needs a fixture.
  3. Root-harness discovery does not fence nested governed roots: the
     repo's own gate now counts examples/base-harness/CLAUDE.md as a
     second `memory` member (any-depth `**/CLAUDE.md` glob; no finding,
     but cross-contamination). Should discovery stop at a directory
     carrying its own `.temper/lock.toml`?
