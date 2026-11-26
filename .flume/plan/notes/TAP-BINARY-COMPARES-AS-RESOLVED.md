# TAP-BINARY-COMPARES-AS-RESOLVED

Seam shape chosen: a tap row's handler array carries one handler per distinct *raw*
binary spelling at its (event, matcher), each naming the first declaration stating it
under a new reserved key `temper:tap-declaration` (beside `temper:tap-binary`). The
engine (`tap_rendered_handlers`, src/drift.rs) resolves each, drops later handlers that
agree, and refuses a disagreement with a new `DriftError::TapBinaryDisagreement`
naming both declarations. No seam version bump: the key lives inside opaque handler JSON.

The old SDK refusal named the *event* twice, not the declarations (it used the event
loop variable); the declaration name is now the requirement's key. A member-level
`requires` key is unqualified by its member, so two members each declaring `x` would
name `x` twice in a refusal -- a possible follow-up if 0100's command-reference rework
touches this.
