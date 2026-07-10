# Plan state

- Spec derived through: 6a04322
- Audited through: 5f88258
- Residue swept through: ec3d112
- This tick: Inbox drain (job 1) — the ts-rs seam note routed as
  SEAM-BINDINGS-GENERATED (open, per engineering.md "Libraries before
  hand-rolls"); gap re-verified at HEAD 6115217 (ts-rs pinned unadopted,
  no sdk/src/generated, declarations.ts hand-restates 13 row shapes).
  Sequencing ask honored: KIND-CONTENT-FACT re-pointed blockedBy →
  SEAM-BINDINGS-GENERATED. Forced reconciliation: FRONTMATTER-MALFORMED-LOUD's
  blocker tag dangled (ship chore 6115217 removed REQUIREMENT-PROSE-PERSISTS);
  verified shipped on disk (drift.rs RequirementRow.prose) and flipped open.
- Queue: FRONTMATTER-MALFORMED-LOUD, SEAM-BINDINGS-GENERATED (both open,
  disjoint) → KIND-CONTENT-FACT → LAYOUT-READER → LAYOUT-PROSE-IMPORT
  (linear blockedBy chain over the shared drift.rs/declarations.ts/main.rs
  surfaces); PACKAGING-CHANNELS (parked, carried verbatim).

Plan continues: yes — ship audit is live: 36e0556 and f36c192 touch
src/tests/sdk past the audit cursor 5f88258 (exit-clause bullets in
open-questions await their ship-audit verification there).
