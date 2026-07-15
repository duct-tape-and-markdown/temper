# Plan state

- Spec derived through: dff2db2
- Audited through: d2496b6
- Residue swept through: d2496b6
- This tick: INBOX drain (two notes). (1) Requirement.kind variance →
  filed REQUIREMENT-KIND-VARIANCE (open): sdk/src/contract.ts:179 typed
  KindDefinition<never> blocks skill/hook-keyed requirements; widen to
  `string | KindDefinition<any>` mirroring the collection child-kind slot
  (kind.ts:315, embeddedMemberValue), declarations.ts:285 gains the string
  arm. SDK-only, no Rust ripple (RequirementRow.kind already a string).
  Human-ruled 07-15, live demand. (2) 2026-07-15 Claude Code drift register
  → NOT re-derived as inbox entries: the recut 0aa9e62 (John, same day,
  AFTER the note at e8edffa) absorbed its spec-encodable half (five→seven
  kinds, command legacy posture, skill path-scope channel gating, domain
  partition) and its commit body RESOLVED the one modeling question the
  register raised — rejected encoding the `paths` gate as a channel
  construct/algebra; it is the field's documented semantics, carried with
  the field. So no open fork survives. The remaining code reconciliation
  (skill new fields + forbiddenKeys, DOCUMENTED_HOOK_EVENTS re-verify,
  agent tools-loud clause, rules glob-validity, cite refreshes) is a
  DERIVATION of the ratified recut — tracked by the spec cursor (still
  dff2db2, behind 0aa9e62), reference `docs/market-formats.md` "Claude Code
  deep audit", carrying the summarizer-mediated re-fetch-raw-page-before-
  cite obligation. Item 4's AGENTS.md detail (not read; `@AGENTS.md`
  bridge) feeds the standing `(agents-md-builtin-kind)` open fork,
  unchanged. Both notes drained from inbox.
- Queue: REQUIREMENT-KIND-VARIANCE (open, pickable) +
  PACKAGING-CHANNELS-REMAINDER (parked, human release actions). Disjoint —
  no shared file (sdk/** vs .github/**).

Plan continues: yes — spec delta 0aa9e62 (builtins recut) unrouted; the spec
cursor is at dff2db2 and job 2 derives the recut's code reconciliation into
entries next tick (docs/market-formats.md the reference).
