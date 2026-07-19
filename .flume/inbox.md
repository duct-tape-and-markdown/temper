<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## RULED (0043): modules declare their subsystem in their own header

Decision 0043 (`specs/decisions/0043-the-map-declares-subsystems-the-tree-declares-membership.md`)
moves module→subsystem membership out of the architecture page's
enumeration and into the modules: every `src/*.rs` module doc gains a
`//! subsystem: <name>` line, every `sdk/src/*.ts` module header the
same as a comment, naming one subsystem from
`specs/process/architecture.md`'s vocabulary (engine: foundation,
model, formats, pipeline, judges, provider, verbs; SDK per its codemap
section). Assignments per the page's current enumeration — this entry
is mechanical, no re-homing judgment. `lib`, `main`, `test_support`,
and `sdk/src/generated/` are in scope like any module (`generated/`
via its emitter's template if hand-edits would be overwritten —
build's call). One entry, one wave.
