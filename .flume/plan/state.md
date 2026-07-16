# Plan state

- Spec derived through: 82c816e
- Audited through: cac023a
- Residue swept through: cac023a
- This tick: RECONCILE the a2f4a1c..cac023a window — one src-touching commit
  (0b2da21, COMPOSED-BODY-ADMISSION). **Audit:** verified on disk, not the
  log — `withinHosts` is gone tree-wide (0 matches across 146 files), the
  host-free `Locus` reads `{ kind: "embedded" }` (kind.ts:57), and admission
  moved corpus-side as `harness({ admit })` (assembly.ts:30 `Admission`,
  keyed by kind value like `expect`). The entry left pending by the flume
  chore, so nothing to drop — but its landing made one gate stale:
  SKILL-NESTED-REFERENCE-DOCS was `blockedBy` it and is now **open**. The
  ship moved the very column that entry grows (`templates` is now derived
  from admission, per host kind), so its premise was re-tested rather than
  assumed: it holds, and disjointly — representation.md keeps the file-child
  template kind-side ("kind") while admission governs the composed body
  ("nesting"), and 0b2da21 went further, refusing to `admit` a non-embedded
  kind at all. That disjointness is now written into the entry's notes: build
  must not route skill's file children through `admit`. All three
  entries re-scoped against HEAD — every anchor re-verified on disk
  (`KindFacts` 102, `templatesFor` 179, `skill` 142, `Template` 565 +
  92/635/675, `ResolvedEmbeddedMemberValue` 300, `Predicate` 81,
  `admissibility` 97, emit.ts 104/158), several having drifted from their
  scoping-time line numbers. Park re-tested: PACKAGING-CHANNELS-REMAINDER
  still true (no version tag — only era tags; crate 0.1.0 vs npm 0.0.7;
  release.yml:7 still defers darwin). **Sweep:** no new residue — the commit
  retired its symbol whole and coined nothing the corpus lacks (`admit` /
  admission is representation.md's own word, "nesting"). Three riders that
  named COMPOSED-BODY-ADMISSION as carrier re-tested and all **undischarged**
  by the reconciliation-not-opening precedent: it opened `sdk/src/kind.ts` and
  rewrote the `Locus` doc *fresh* (49-59) while leaving the fact-3 "layout"
  narration (4/16, 106/108 → 109/111) and "posture 3" (254 → 257); it opened
  `sdk/test/emit.test.ts` and left the `renderMemberFence` cite (907 → 937).
  Re-pointed: kind.ts's two riders → SKILL-NESTED-REFERENCE-DOCS,
  emit.test.ts's → EMBEDDED-FORMAT-TARGET-FACTS (the one queued entry opening
  it), builtins.ts's two PACKAGE.md cites (565/611, unmoved) →
  SKILL-NESTED-REFERENCE-DOCS. session_start.rs's `+++` fixtures (128/133/146)
  keep no carrier — no queued entry opens that file.
- Queue: 1 pickable (SKILL-NESTED-REFERENCE-DOCS, gate opened this tick);
  EMBEDDED-FORMAT-TARGET-FACTS → FORMAT-OMITS-EDGE-CLAUSE serialize behind it
  (shared `sdk/src/kind.ts`); PACKAGING-CHANNELS-REMAINDER parked (John's
  Apple notarizing + the v0.1 lockstep tag), gate re-tested on disk at
  cac023a. Fork board unchanged — `(guidance-climb)` blocks nothing (0025
  sequences it after the machinery).

Plan continues: no — the window is reconciled and both cursors sit at HEAD.
Every input above is dry: the inbox is empty, `.flume/refactor/` holds only
its README, and the spec delta is empty (82c816e is still the last `specs:`
commit — nothing past the cursor). **Build takes over:
SKILL-NESTED-REFERENCE-DOCS is pickable.**
