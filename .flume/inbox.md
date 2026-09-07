<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


- observed at a5101a9d (build NESTED-MEMBER-DUPLICATE-KEY-ADMISSIBILITY's
  friction capture `.flume/friction/build-hook-kind-collision-test-stalls-
  under-fanout.md`, diagnosis verified by the session) — **`check
  --harness <dir>` walks OUTSIDE the harness root it was given.** The
  hook_kind collision fixture spawns `temper check --harness
  /tmp/collection-address-collision<rand>`; sampling the child's open fds
  shows it descending `/tmp/check-cost<rand>/…`, `/tmp/admit-dangling-
  verifier<rand>/…` — every other suite's fixture tree, none under the
  root it was handed. With /tmp at 9 GB / 37k entries the test ran 400s+
  per spawn and stalled every fanout wave's afterMerge gate. Product
  defect: discovery's root must be the harness root, never a parent
  derived from it (the root-prefix derivation class 07-18 fixed for the
  worktree base); `specs/model/adoption.md` discovery walk. Acceptance: a
  fixture rooted directly under a populated parent walks only itself
  (count entries visited); a regression test that plants a sibling dir
  beside the harness and asserts it is never opened. Separately (harness
  work, not product): `tests/common::tmpdir` persists every fixture with
  `.keep()` at /tmp's top level, so debris grows monotonically — give
  fixtures one per-run parent under `tempfile` so the session can sweep
  it; the session cleared 9 GB of it on 09-07.
