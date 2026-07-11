# Plan state

- Spec derived through: a9f7b9e
- Audited through: 26862e3
- Residue swept through: 26862e3
- This tick: Inbox drain — routed the two dogfood-found manifest-write defects
  (both observed at fd3d2ec; fd3d2ec..HEAD touched only .flume/inbox.md, so
  both premises hold verbatim at HEAD d927edf). Verified each on disk before
  scoping. (1) MANIFEST-WRITE-HOOK-SHAPE — drift.rs:665 maps a hook
  registration's fields to a FLAT `hooks.<Event>={command,matcher,type}` entry,
  but the documented + read-face shape is `[{matcher,hooks:[{type,command}]}]`
  (array of matcher groups); round-trip broken, Claude Code ignores the flat
  form. per representation.md "Reach". (2) MANIFEST-WRITE-SETTINGS-RESIDUE —
  assembly.settings (assembly.ts:41) has NO seam family (EmitResult carries
  declarations/members/permissions/registrations, not settings residue), and
  drift.rs:693 fills build.residue only from a path-matched container member —
  none projects to settings.json — so authored settings are shed silently
  (invariant 6). per pipeline.md "Emit". Both edit src/drift.rs, so serialized:
  HOOK-SHAPE open, RESIDUE blockedBy it (pending-entry disjoint rule).
  PACKAGING-CHANNELS copied forward verbatim.
- Queue: HOOK-SHAPE (open, pickable), MANIFEST-WRITE-SETTINGS-RESIDUE
  (blockedBy HOOK-SHAPE), PACKAGING-CHANNELS (parked, human release creds).
  Cursors unmoved — inbox was the job; 26862e3..HEAD carries no src/tests/sdk
  change to reconcile.

Plan continues: no — inbox drained, spec-delta empty, 26862e3..HEAD has no
src/tests/sdk change; one pickable entry filed (HOOK-SHAPE), build takes over.
