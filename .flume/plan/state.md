# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 32a4c9c — unchanged, no new src/tests/sdk commits since (`git log
  32a4c9c..HEAD -- src/ tests/ sdk/` empty).
- Residue swept through: 32a4c9c — unchanged, same reason.
- Posture swept through: sdk/src/declarations.ts (+ assembly.ts/kind.ts/contract.ts/prose.ts/
  builtins.ts) covered, sdk/src/claude-code.ts and sdk/src/dial.ts covered (earlier this
  rotation), and now sdk/src/emit.ts (+ its immediate import needs.ts, newly covered this tick)
  covered too — mid-rotation. sdk/src/index.ts next in rotation (tree order; the only sdk/src/
  module the phrase delta 2e2b32a still owes — dial.ts/emit.ts/needs.ts now covered close the
  rest of the alphabetical remainder — then the tests/ tree).
- This tick: POSTURE SWEEP, sdk/src/emit.ts neighborhood (assembly.ts/kind.ts/prose.ts/
  declarations.ts already covered, re-read for context only; needs.ts not yet covered, read and
  judged this tick). Filed EMIT-NEEDS-ZERO-CONSUMER-EXPORTS-PRUNE (open): `ResolveOptions`/
  `edgePlacements`/`renderedExtents` (emit.ts) called only inside `emit()`'s own `compile()` —
  same shape as DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE (de-export, don't delete); `capability`
  (needs.ts) has zero call sites anywhere, grep-verified over sdk/src, sdk/test, src/, tests/,
  .temper/ — deleted outright. `bash`/`permissionUnion`/`Capability`/`RegistrationFact`/
  `SettingsResidue`/`EmitResult` all keep live external callers. No other neighborhood finding.
- Queue: 4 pending — 1 open (EMIT-NEEDS-ZERO-CONSUMER-EXPORTS-PRUNE), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-
  REMAINDER). Open forks: 2 (multi-harness-projection, lazy-grounds), unchanged. Friction: 0.
  Amendments: 0. Inbox: 0 notes.

Plan continues: after-build — the posture rotation is the only remaining live job and a pickable
entry now exists (EMIT-NEEDS-ZERO-CONSUMER-EXPORTS-PRUNE), so the wave ships it before the sweep
resumes.
