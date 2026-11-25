# Emit refuses two members at one projection path

The refusal lives in `member_path_index` (src/drift.rs), the one pass that derives
every member's path before any write: a second insert at a normalized path raises
`DriftError::ProjectionPathCollision` naming both whole addresses and the path.
It covers local-locus, layout and container members too (all derive a path there),
and the whole suite stayed green, so no tree relied on sharing one.

A registration-only manifest path shared with its one container member is the
sanctioned case and is untouched (registrations are not members). A nested-memory
adopter now gets this refusal instead of the silent win the
INSTALL-LIFTS-A-NESTED-MEMORY-AT-ITS-PLACEMENT park saw; that park's placement
question is still open.
