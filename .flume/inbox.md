<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->








- observed at 63491f1b (fork ruling, John 2026-09-29: `(directive-backing-set-scope)`)
  — the backing set stays raw disk: `contract.md` "edge" already says a
  path-resolved edge reads raw disk, never the discovery view, so folding
  onto the local discovery flavor would prune `.git/` and sub-corpus
  files and forge findings. The cost cut is resolution by stat: the
  directive half is a membership test on each cited `@import` target, and
  the whole-set walk runs only where a root `reachable` clause binds. A
  directory or symlink target must be spelled against today's
  `is_file()` walk. The `(lazy-grounds)` objection does not apply: that
  horizon is about materializing ground members, this is ordinary path
  resolution. The count pin (c4a99757) moves to 0 where no root
  `reachable` clause binds.
- observed at 63491f1b (fork ruling, John 2026-09-29:
  `(embedded-edge-dangling-judgment)`) — `pipeline.md` "Refusing" already
  answers it: a dangling edge refuses at emit within the program's own
  universe, and a target in a declared kind whose members are not composed
  values defers to `check`. The render-selection candidate (refuse only
  when a hook selected the target) is rejected as a second rule for one
  question. Verify `edgeTargetFacts` refuses by that universe rule and
  file only the gap.
