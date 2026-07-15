# Plan state

- Spec derived through: 0aa9e62
- Audited through: 8ccd01d
- Residue swept through: 8ccd01d
- This tick: POST-SHIP RECONCILIATION of d2496b6..HEAD. Two build commits in
  window, both audited on disk against their diffs: (1) SKILL-PATHS-CHANNEL-GATE
  (2c26759) — skill `paths` added as a guidance-only field (builtins.ts:63), no
  clause, no registration entry; test asserts `skill.facts.registration`
  unchanged (still user-invoked + description-trigger). Matches builtins.md
  ("a declared field may also gate the member's channels outright … carried
  with the field, never a channel entry"). (2) REQUIREMENT-KIND-VARIANCE
  (0ee0c25) — `Requirement.kind` widened `KindDefinition<never>` →
  `string | KindDefinition<any>`; `requirementRows` handles the string arm;
  `RequirementRow.kind` stays a string so the frozen-lock seam is unmoved
  (contract.md, "requirement" — kind carries identity for coverage). Both
  already dropped from pending by build. SWEEP: no new residue — the one
  file this window opened that carries a rider (builtins.ts, opened by
  2c26759) left the three retired-`packages/` doc-comment cites unreconciled;
  staleness exception, so it rides (not filed), cites bumped +12
  (392/432/469→404/444/481) in the kept-on-purpose record. Both cursors → HEAD.
- Queue: PACKAGING-CHANNELS-REMAINDER (parked — John's Apple notarizing +
  v0.1 launch tag). Sole entry; not pickable.

Plan continues: no — inbox empty, spec delta drained to 0aa9e62, reconcile
window closed to HEAD; the one remaining entry is parked, nothing pickable, so
the loop hibernates.
