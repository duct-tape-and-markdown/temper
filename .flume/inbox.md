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
  Remaining in the entry: darwin binaries (Apple notarizing, on John),
  channel 3 (plugin + marketplace.json), standalone tag assets (workflow
  path exists, unexercised until a tag), and the binary self-reporting
  its crate version (0.1.0) while npm carries 0.0.x — lockstep lands at
  the v0.1 tag. Re-scope the entry to the remainder. observed at 4f55036
