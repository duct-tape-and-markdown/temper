# Plan state

- Spec derived through: 6a04322
- Audited through: ab73a60
- Residue swept through: a932bb0
- This tick: Residue sweep (job 4) over fcdbe52..a932bb0 (code commits
  4f5df8d, 36a7662). One class found and filed: SEAM-PAYLOAD-TYPED —
  36a7662 wired the declaration rows to the generated seam home but
  generated/Payload{,Member} have zero SDK consumers; emit.ts still
  hand-restates PayloadMember and encodeSeam is untyped, so the member
  half of the seam still drifts silently; declarationsToJson's
  declarations-only pipe parses as no engine shape. 4f5df8d clean (one
  frontmatter parse surface, one FrontmatterError home). Both
  retired-trees debts re-verified unmoved; Cargo.toml's stale schemars
  cite (src/schema/interchange.rs) logged as new riding debt.
- Queue: TEST-ROW-FIXTURES-ONE-HOME (open) → SEAM-PAYLOAD-TYPED →
  KIND-CONTENT-FACT → LAYOUT-READER → LAYOUT-PROSE-IMPORT (linear
  blockedBy chain — shared drift.rs/declarations.ts/emit.test.ts/
  lock_declaration_rows.rs surfaces); PACKAGING-CHANNELS (parked,
  reason re-verified at ab73a60).

Plan continues: yes — quiet pass (closing verification over the
re-sequenced queue).
