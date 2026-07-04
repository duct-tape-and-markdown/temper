<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-04 (session, post-merge of PR #2): the installed CI gate job fails
  `temper: command not found` (exit 127) on every run — the projected
  workflow runs `temper import . --into .temper && temper check` with no
  build/install step on the runner (run 28719951414). Two halves: the
  missing toolchain setup in the projected workflow, and the job still
  speaking the retired import-verb surface — both belong to whatever entry
  re-derives `install`'s CI projection under the six-noun corpus.
