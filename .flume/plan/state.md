# Plan state

- Spec derived through: 6a04322
- Audited through: f3a5356
- Residue swept through: e9d05f6
- This tick: Residue sweep (job 4). Delta 08550e5..e9d05f6 is one build
  commit, a538a76 (KIND-CONTENT-FACT). No structural residue: the SDK's
  contentRow lowers into the generated LayoutRow/LayoutRegionRow bindings
  (seam discipline held); drift.rs's content codec follows the file's
  per-row inline-table pattern; fixtures extended in tests/common. Two
  comment-staleness finds, both riding: kind.rs:39 "five facts" struct doc
  (widened onto the existing kind.rs ride — LAYOUT-READER's description
  now names both); kind.ts's pre-0019 "layout" fact-3 naming, colliding
  with the exported Layout type (new bullet — rides the next kind.ts
  opener). Standing debts re-verified on disk (session_start.rs
  128/133/146; builtins.ts 308/348/385 — a538a76 touched neither).
- Queue: LAYOUT-READER (open) → LAYOUT-PROSE-IMPORT (blockedBy — shared
  main.rs/drift.rs surfaces); PACKAGING-CHANNELS (parked).

Plan continues: yes — quiet closing pass (all inputs current after this
sweep).
