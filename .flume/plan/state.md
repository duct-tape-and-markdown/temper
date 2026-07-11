# Plan state

- Spec derived through: a9f7b9e
- Audited through: f075f8d
- Residue swept through: f075f8d
- This tick: Post-ship reconciliation, window 5f27db2..HEAD (COVERAGE-PARTIAL-
  GOVERNANCE 3dd6b52, MANIFEST-WRITE-SDK-ERASURE 8cc0561). AUDIT: both verified
  on disk. 3dd6b52 narrowed coverage.unmodeled-surface — a partially-governed
  manifest now names only its ungoverned residue (coverage_note.rs
  unmodeled_residue split). 8cc0561 is SDK-side only: fields-only hook/mcp-server
  members now carry typed fields (shape:"fields") and emit erases each into a
  RegistrationFact on EmitResult (name/collectionAddress/fields) — no seam/engine
  change, one-sided until the write face consumes it. GATE RE-TEST: MANIFEST-
  WRITE-EMIT-FACE was blockedBy SDK-ERASURE → SDK-ERASURE shipped → flipped
  EMIT-FACE to open. BUNDLE/GUARD/COVERAGE-RETIRE stay blockedBy EMIT-FACE
  (unshipped); PACKAGING-CHANNELS stays parked. SWEEP: no new residue — shipped
  code is well-cited. Three doc-comment riders on files 8cc0561 opened
  (builtins.ts 392/432/469 PACKAGE.md cites; kind.ts fact-3 "layout" narration
  4/16/106/108; kind.ts "posture 3" shifted 252→254) each opened-and-left per
  reconciliation-not-opening — re-verified on disk at f075f8d, undischarged.
  Both cursors → f075f8d.
- Queue: EMIT-FACE (open) → BUNDLE/GUARD/COVERAGE-RETIRE (blockedBy EMIT-FACE) →
  PACKAGING-CHANNELS (parked). One open head; all blockedBy tags resolve.

Plan continues: no — window reconciled to f075f8d, inbox/spec-delta empty; one
open pickable entry (EMIT-FACE), build takes over.
