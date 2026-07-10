# Plan state

- Spec derived through: a9f7b9e
- Audited through: 3d13eb4
- Residue swept through: 3d13eb4
- This tick: Post-ship reconciliation, window 747f8cc..HEAD. Its one
  src/tests/sdk commit is cd1ca29 — MANIFEST-KIND-MODEL: the fields-only
  `Content::Fields` shape + `CollectionAddress`/`CollectionKeyPath` kind
  fact, verified on disk in `src/kind.rs` (not the log). Audit: the entry
  shipped and is already dropped from pending; its downstream blocker
  cleared, so MANIFEST-ADAPTER-READ flips `blockedBy MANIFEST-KIND-MODEL`
  → `open` — its refs resolve (extract.rs/import.rs on disk, json_manifest.rs
  correctly absent). MANIFEST-WRITE-SIDE + PACKAGING-CHANNELS stay parked
  (phase-1 not yet shipped; cd1ca29 touched no release/packaging surface).
  Sweep: cd1ca29 is corpus-sanctioned (0021, representation.md "Reach") —
  no new residue. It opened `sdk/src/kind.ts` and left two doc-comment
  riders (the "layout" fact-3 narration, the "posture 3" cite) while shifting
  their lines; both updated in open-questions per the reconciliation-not-
  opening precedent (staleness rides the file, never a standalone entry).
  The remaining riders name files cd1ca29 never touched — carried forward,
  stamps bumped to 3d13eb4. Both cursors advance.
- Queue: MANIFEST-ADAPTER-READ (open, next) → HOOK-KIND → MCP-SERVER-KIND
  (blockedBy chain) → MANIFEST-WRITE-SIDE (parked, phase 2) →
  PACKAGING-CHANNELS (parked). Disjoint: only the head is `open`.

Plan continues: no — inbox empty, no specs delta past a9f7b9e,
reconciliation done and both audit/residue cursors at HEAD. No live plan
input remains; MANIFEST-ADAPTER-READ is `open` for build to pick up.
</content>
