# Plan state

- Spec derived through: a9f7b9e
- Audited through: 224b880
- Residue swept through: 224b880
- This tick: Post-ship reconcile 26862e3..HEAD (224b880) — audit + sweep over
  the one src window (HOOK-SHAPE). AUDIT: HOOK-SHAPE shipped (5fc3e9f nests
  hooks into settings.json's matcher-group array via the sibling registration
  loop, drift.rs:672-693 + extract.rs `hook_matcher_group` 928-1015; 224b880
  removed it from pending), so MANIFEST-WRITE-SETTINGS-RESIDUE unblocks →
  gate open. Re-verified its premises verbatim on disk: EmitResult carries
  declarations/members/permissions/registrations, NO settings-residue family
  (emit.ts:321-344); assembly.settings at assembly.ts:41; build.residue fills
  only from a path-matched container member (drift.rs:708), struct field 841,
  none projects to settings.json — HOOK-SHAPE's hook-nesting left that path
  untouched. Entry cites re-stamped (~693→708, ~826→841). SWEEP: the window's
  only src change opened src/extract.rs (+135); two ride-only riders
  re-verified undischarged (floor-mention comment 196-198; two law-5 fixture
  strings shifted 1223/1258→1340/1375) and their open-questions records
  updated — no new standalone residue. PACKAGING-CHANNELS parked, copied
  forward: window touched no .github/root-package.json, so its condition
  (human release creds + engine-binary workflow) is provably unchanged.
- Queue: RESIDUE (open, pickable), PACKAGING-CHANNELS (parked, human release
  creds). Spec cursor unmoved — no specs commit past a9f7b9e.

Plan continues: no — reconciliation was the last live input; queue holds one
pickable entry (RESIDUE, now open), build takes over.
