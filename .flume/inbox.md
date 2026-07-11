<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `coverage.unmodeled-surface` now lies about `settings.json`: post-76aaa83
  a gate run checks `hook (3)` inferred from that file yet the advisory
  still reads "no kind governs it — temper checks none of its members"
  (probe: `cargo run -- check .temper`, both lines in one run). 0021's
  per-manifest retirement needs its partial-governance spelling: narrow the
  finding to the actually-ungoverned residue segments (permissions, env) or
  retire it for a manifest any kind governs — either way the message must
  state only true things at every intermediate state, per "loud or
  nothing"/invariant 6. The 15a71ac sweep judged the coverage semantics
  sound and MANIFEST-WRITE-SIDE completes the retirement, but a false
  diagnostic cannot ride until then. observed at 15a71ac