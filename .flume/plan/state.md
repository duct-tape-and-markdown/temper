# Plan state

- Spec derived through: 6a04322
- Audited through: b8f0746
- Residue swept through: fcdbe52
- This tick: Residue sweep (job 4) — window ec3d112..fcdbe52 (code commits
  36e0556, f36c192). One structural class found and filed:
  TEST-ROW-FIXTURES-ONE-HOME (per engineering.md "One job, one home" —
  bare RequirementRow/ClauseRow/KindFactRow test literals and per-file
  builders beside tests/common; 36e0556's prose column demonstrated the
  ripple). `means` vocabulary clean (English/narration only); the seam
  lockstep growth is already routed by SEAM-BINDINGS-GENERATED; both
  retired-trees debts re-verified on disk (bullet refreshed);
  tests/install.rs:15,382 universals now ride the new entry. The
  build-filed friction capture (harness.ts stale `means:` key) is moot at
  HEAD — 8d741ba shipped its fix; left for the human sweep to delete.
- Queue: FRONTMATTER-MALFORMED-LOUD, SEAM-BINDINGS-GENERATED (open,
  disjoint) → TEST-ROW-FIXTURES-ONE-HOME → KIND-CONTENT-FACT →
  LAYOUT-READER → LAYOUT-PROSE-IMPORT (linear blockedBy chain over the
  shared drift.rs/declarations.ts/lock_declaration_rows.rs/main.rs
  surfaces); PACKAGING-CHANNELS (parked, carried verbatim).

Plan continues: yes — all inputs current after this sweep; one quiet
closing pass (queue disjointness, gate reasons, state re-derivation)
then hand off to build.
