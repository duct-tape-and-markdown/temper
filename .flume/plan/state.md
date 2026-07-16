# Plan state

- Spec derived through: cc5a9b3
- Audited through: d97a704
- Residue swept through: d97a704
- This tick: POST-SHIP RECONCILIATION — the cac023a..d97a704 window, one build
  commit (794678f, TEMPLATE-FILE-CHILD-FACT). Audit: verified on disk that the
  declared fact shipped across all three layers — `KindFacts.templates`
  (`sdk/src/kind.ts:158`, fact 7), the `TemplateRow` seam binding,
  `templatesFor`'s two-loci derivation (`declarations.ts:187`), and the Rust
  `Template` + `overlay_templates` read-back (`src/kind.rs:567/680`). The
  entry's own chore already dropped it from the queue, so the audit's live
  action was the gate behind it: **EMBEDDED-FORMAT-TARGET-FACTS unblocks**
  (`blockedBy` → `open`), cites re-derived against the moved file
  (`ResolvedEmbeddedMemberValue` 300→323, `KindFacts.edgeFields` 126→146, the
  `render` hook 159→160; `EdgeField`:43 and emit.ts's 104/158 unmoved).
  PACKAGING-CHANNELS-REMAINDER re-tested on all three park conditions at HEAD
  and holds (`git tag -l 'v*'` empty, crate 0.1.0 vs npm 0.0.7, release.yml:7-9
  still defers darwin). Sweep: the window introduced no new residue — the entry
  declared its riders and carried two. The **pre-0019 "layout" fact-name record
  is discharged and deleted**: fact 3 respelled to "projection", the count moved
  to seven, and the sanctioned `Layout` content type no longer collides in-file.
  The `kind.ts:257` "posture 3" half of the pre-recut record is discharged too,
  narrowing that record to `prose.ts` alone. Undischarged, re-verified unmoved:
  builtins.ts:565/611, session_start.rs:128/133/146, contract.rs:475,
  compose.rs:233, prose.ts's narration, and emit.test.ts's `renderMemberFence`
  cite (937→980, riding EMBEDDED-FORMAT-TARGET-FACTS). `(nested-file-child)`
  **narrowed, not closed**: collision (1) is now half-answered — a kind can
  declare a file-child layer, but nothing composes or resolves one — so the fork
  still owes the composition surface, the resolution rule, and the overlap's
  legality. Four accreted rider records compacted against the file's own
  anti-accumulation rule; every surviving claim disk-verified at HEAD.
- Queue: 1 pickable (EMBEDDED-FORMAT-TARGET-FACTS, `open`);
  FORMAT-OMITS-EDGE-CLAUSE serializes behind it (a semantic gate, not a file
  overlap: the target facts are its placement observation point);
  PACKAGING-CHANNELS-REMAINDER
  parked (John's Apple notarizing + the v0.1 lockstep tag). Fork board:
  `(nested-file-child)` narrowed, still blocking the supporting-doc adoption;
  `(guidance-climb)` blocks nothing.

Plan continues: no — reconciliation was this tick's one job and both cursors
advance to d97a704. No input below it is live: the window is closed, the inbox
and refactor captures are empty. **Build takes over:
EMBEDDED-FORMAT-TARGET-FACTS is pickable.**
