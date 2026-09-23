<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->





## The example-harness emit test reads an unbuilt SDK: a flake that reverted CONTAINMENT-INCIDENCE-FAMILY — observed at 36d1ae35

**Observed:** CONTAINMENT-INCIDENCE-FAMILY passed in its worktree, then failed
the afterMerge `cargo test` on trunk with one test,
`emit_program_runs_the_shipped_example_harness` (tests/emit.rs), crashing on
`ENOENT … /home/jwcam/repos/temper/sdk/dist/src/prose.js`. The entry was
reverted and quarantined. It touches no SDK file, and `sdk/dist` was complete
again minutes later (rebuilt 18:33 by the `sdk test` gate).

**Read (why):** the test vendors the SDK into
`examples/base-harness/.temper/node_modules/@dtmd/temper` in the **checkout
itself**, via `common::vendor_sdk` (tests/common/mod.rs:118). That function
returns early when the link already exists, *before* `ensure_sdk_built()`
(:124). The link has existed in the primary checkout since 15:55, so every run
since then has read whatever `sdk/dist` holds, unbuilt. The SDK's `build`
and `test` scripts both start with `rm -rf dist` (sdk/package.json:43-44),
so any run that overlaps or precedes a rebuild sees a missing file. The
worktree passed because its agent had built the SDK.

**Fix direction:** `vendor_sdk` guarantees the build whether or not the link
exists. Better, the test copies the example into a temp dir rather than writing
a `node_modules` link into the source tree; a test that mutates the checkout
is how the stale link persisted. Either way it goes ahead of
CONTAINMENT-INCIDENCE-FAMILY's retry, or the flake can revert it again.
Containment itself needs no change.
