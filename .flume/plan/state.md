# Plan state

- Spec derived through: 0aa9e62
- Audited through: d2496b6
- Residue swept through: d2496b6
- This tick: SPEC DELTA drain — routed the builtins recut (0aa9e62). Three
  amendments, all checked: (1) five→seven kinds — MOOT, sdk/src/builtins.ts
  already ships `hook`/`mcp-server` kinds (lines 161/231, since 0021); (2)
  channel gating (a skill path scope gates all channels) — filed
  SKILL-PATHS-CHANNEL-GATE (open): the `Skill` interface has no `paths`
  field, so the freshly-ratified gating fact (cite 07-15, verified 2.1.210)
  is unmodeled; add it as a guidance-only field, NOT a registration entry
  ("carried with the field, never a channel entry") and no clause (mirrors
  rule's paths); (3) domain partition — MOOT for code, descriptive model
  prose, no decidable check, reinforces the spine rule code already honors
  (no baked baseline). All three routed → cursor to 0aa9e62. NB the code
  reconciliation the prior tick anticipated (skill forbiddenKeys,
  DOCUMENTED_HOOK_EVENTS re-verify, agent tools clause, rules glob-validity)
  lives in `docs/market-formats.md` (candidate intent, fence-excluded), NOT
  in the recut spec text — not derived; it enters the queue only if ratified
  into specs.
- Queue: REQUIREMENT-KIND-VARIANCE (open) + SKILL-PATHS-CHANNEL-GATE (open) +
  PACKAGING-CHANNELS-REMAINDER (parked). Disjoint — contract.ts/declarations.ts
  vs builtins.ts/builtins.test.ts vs .github/**, no shared file.

Plan continues: no — inbox empty, spec delta drained to 0aa9e62, reconciliation
window (d2496b6..HEAD) has no src/sdk/tests touches; two pickable open entries
remain, so build takes over.
