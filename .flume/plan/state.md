# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 32a4c9c — unchanged, no new src/tests/sdk commits since (`git log
  32a4c9c..HEAD -- src/ tests/ sdk/` empty).
- Residue swept through: 32a4c9c — unchanged, same reason.
- Posture swept through: sdk/src/declarations.ts (+ assembly.ts/kind.ts/contract.ts/prose.ts/
  builtins.ts) covered, sdk/src/claude-code.ts and sdk/src/dial.ts covered (earlier this
  rotation), and sdk/src/emit.ts (+ needs.ts) covered too — mid-rotation, unchanged this tick.
  sdk/src/index.ts next in rotation (tree order; the only sdk/src/ module the phrase delta
  2e2b32a still owes).
- This tick: INBOX, four notes drained. IMPORT-HOP-CAP-CITE resolved by hand (14719f2, cite
  now reads four hops) — dropped. Consumer-format-constants finding ruled/parked in
  docs/ledger.md, nothing to route — dropped. EMIT-NEEDS-ZERO-CONSUMER-EXPORTS-PRUNE was
  mis-scoped (human-ruled, 14719f2): `ResolveOptions`/`capability`/`Capability` are re-exported
  from sdk/src/index.ts (91, 21-22) — public API, not zero-consumer; only `edgePlacements`/
  `renderedExtents` are internal. Rewrote to that scope (prepared branch 63cbefe un-exports all
  four, must not merge as-is). Filed the underlying defect — "An export earns its consumer"
  ignores public re-export as a consuming edge — as an amendment, not an entry:
  `.flume/amendments/plan-export-consumer-reexport-carveout.md`.
- Queue: 3 pending — 1 open (EMIT-NEEDS-ZERO-CONSUMER-EXPORTS-PRUNE, rescoped), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked (PACKAGING-CHANNELS-REMAINDER). Open forks: 2
  (multi-harness-projection, lazy-grounds), unchanged. Friction: 0. Amendments: 1 (new, awaiting
  human ratification). Inbox: 0 notes (drained).

Plan continues: after-build — the posture rotation is the only remaining live job and a pickable
entry exists (EMIT-NEEDS-ZERO-CONSUMER-EXPORTS-PRUNE, rescoped), so the wave ships it before the
sweep resumes.
