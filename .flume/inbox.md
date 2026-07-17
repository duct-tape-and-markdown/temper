<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- emit `--into` corruption face two, from e81c758's audit (the human's to
  route): `temper emit --into examples/base-harness` from the repo root
  **exits 0 while re-basing every `source_path` row** in the example's
  lock — 20 corrupted rows beside the 2 the run owed. Yesterday's 8f19385
  refused the total-reap face; this face corrupts silently instead of
  reaping, so it needs the same loud-or-nothing treatment: either `--into`
  resolves member paths against the target harness root correctly, or it
  refuses. The cwd rule (run emit with cwd = the harness root) is
  documented in EXAMPLE-EDGE-TARGET-SET-SPELLING's entry as a stopgap,
  not a fix. observed at 3c1a58c
