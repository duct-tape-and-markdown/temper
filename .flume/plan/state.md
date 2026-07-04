# Plan state

- **Phase:** reconcile. HEAD 118af33.
- **Last shipped:** two human `chore`s landed since the last plan tick —
  `chore(harness)` 4bd4813 (wave-end confirmation pass: `temper.toml` regenerated
  via `emit`, now 17 `[[member]]` tables; `.temper/lock.toml` re-stamped) and
  `chore(flume)` 118af33 (re-armed the dogfood self-check gate + **wired the `sdk
  test` gate into chain.ts**, resolving `(sdk-build-gate)`). No `specs:` commit
  since d4a6999.
- **This tick:** `(sdk-build-gate)` RESOLVED (verified: `sdkGate` in build's
  `gates` array, `pnpm --dir sdk test` afterMerge), so the **SDK altitude wave
  un-parks**. Flipped SDK-EMIT-BYTE-PARITY parked→`open` and dropped its fork;
  cleared the same fork from SDK-BODY-RESOLUTION / SDK-PROJECTION-LOCK (they stay
  `blockedBy` in series over shared emit.ts). Marked the accepted dogfood debt
  DISCHARGED (temper.toml regenerated). Inbox empty. Deferred/parked entries hold
  on disk: `BUILTIN_KINDS = ["skill","rule"]` (no agent kind), `Primitive::Field
  { key }` still flat-reads, PACKAGING-CHANNELS still needs human creds.
- **Pickable now:** **SDK-EMIT-BYTE-PARITY** (`open`) — bring `sdk/src/emit.ts`
  to byte-parity with the Rust `toml_edit` manifest output; the gate now validates
  it. Its two successors are `blockedBy` in series (shared emit.ts, not
  parallel-safe). Nothing else is pickable: EXTRACTION-VOCAB-GAPS / AGENT-KIND
  deferred (no consumer), PACKAGING-CHANNELS parked on human creds.
- **What's next:** build ships SDK-EMIT-BYTE-PARITY; a following plan reconcile
  un-blocks SDK-BODY-RESOLUTION, then SDK-PROJECTION-LOCK. Human forks still open:
  PACKAGING-CHANNELS creds, `(display-rule-emit-face)` scope for the rung-3 pilot.

Plan continues: no — queue reconciled, inbox empty, and one `open` entry
(SDK-EMIT-BYTE-PARITY) is now pickable. Hand to build; building is how the queue
drains.
