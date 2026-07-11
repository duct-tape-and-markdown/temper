# Plan state

- Spec derived through: a9f7b9e
- Audited through: 8e974ee
- Residue swept through: 8e974ee
- This tick: Post-ship reconciliation, window 7fd76de..HEAD (SEAM 6d90c43,
  BUNDLE 756d32b). AUDIT: MANIFEST-WRITE-SEAM landed both halves on disk — the
  RegistrationRow family crosses the seam (declarations.ts registrationRows →
  Payload/Declarations.registrations), and drift::emit routes each host manifest
  whole through json_manifest::write_manifest (emit_manifest + ManifestBuild,
  drift.rs:815/906): registrations on the lock as identity+address, a container
  member's opaque residue carried, owned so re-emit never reaps it. That is
  exactly the blocker both parked entries named ("a manifest genuinely
  REPRESENTED — members in the lock, container an opaque-field-carrying member").
  GATE RE-TEST: MANIFEST-WRITE-GUARD + MANIFEST-WRITE-COVERAGE-RETIRE (both
  blockedBy SEAM) → flipped open; premises re-verified intact on disk —
  install.rs's guard still greps only .claude/ (GUARD_PATH_MATCH:128,
  GUARD_MESSAGE:121); coverage_note.rs's None arm (202) still conflates
  empty-residue with empty-checked. Files-disjoint (install.rs vs
  coverage_note.rs), so both are open heads. MANIFEST-WRITE-BUNDLE (756d32b)
  verified: bundle.rs now routes through write_manifest (bundle.rs:268, "one
  encoder") — its second-serde_json-encoder residue consolidated. SWEEP: window
  touched only declarations.ts/emit.ts/generated + drift.rs/bundle.rs + four
  tests; no new second encoder (emit.ts registrationFacts now maps from
  registrationRows, single source), no retired vocab in touched files, and no
  open-questions "rides X" rider names any file this window touched. Both cursors
  → 8e974ee.
- Queue: GUARD (open) + COVERAGE-RETIRE (open, disjoint) pickable →
  PACKAGING-CHANNELS (parked, human release creds). Two open files-disjoint heads.

Plan continues: no — window reconciled to 8e974ee, inbox/spec-delta empty; two
open files-disjoint entries (GUARD, COVERAGE-RETIRE) pickable, build takes over.
