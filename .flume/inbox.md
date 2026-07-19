<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## RULED (0043): lib.rs registry groups its mod lines by subsystem

Decision 0043 (`specs/decisions/0043-the-map-declares-subsystems-the-tree-declares-membership.md`)
makes `src/lib.rs`'s `pub mod` registry the home of module→subsystem
membership: reorder the mod lines into seven groups, one section
comment each, in the page's vocabulary order (foundation, model,
formats, pipeline, judges, provider, verbs — `lib` and `test_support`
sit under verbs; `main` is the binary, not in the registry),
alphabetical within a group. Assignments per
`specs/process/architecture.md`'s current enumeration — mechanical,
no re-homing judgment, no per-module edits, no SDK half. Observed at
f34e676. One entry, one wave.
