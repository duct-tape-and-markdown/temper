# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 638f051 — window 32a4c9c..638f051 (14719f2, 263ebec) reconciled this tick.
- Residue swept through: 638f051 — same window, same tick.
- Posture swept through: sdk/src/declarations.ts (+ assembly.ts/kind.ts/contract.ts/prose.ts/
  builtins.ts) covered, sdk/src/claude-code.ts and sdk/src/dial.ts covered (earlier this
  rotation), and sdk/src/emit.ts (+ needs.ts) covered too — mid-rotation, unchanged this tick.
  sdk/src/index.ts next in rotation (tree order; the only sdk/src/ module the phrase delta
  2e2b32a still owes).
- This tick: POST-SHIP RECONCILIATION over 32a4c9c..638f051. Audit: 263ebec un-exported exactly
  `edgePlacements`/`renderedExtents` from sdk/src/emit.ts, matching EMIT-NEEDS-ZERO-CONSUMER-
  EXPORTS-PRUNE's rescoped entry precisely — `ResolveOptions` and `capability`/`Capability`
  verified still exported and still re-exported at sdk/src/index.ts:91/21-22; merge dropped the
  entry cleanly (metrics.jsonl: 111368ms, 23 turns, no revert), pending.json already lacks it.
  14719f2 (IMPORT-HOP-CAP-CITE) was already dropped by hand last tick — re-confirmed, nothing
  further. Sweep: no residue — remaining `edgePlacements`/`renderedExtents` mentions are doc-
  comment cites naming the (still-existing, now-internal) functions, not stale. Re-tested both
  standing gates: PACKAGING-CHANNELS-REMAINDER's parked condition holds (no v0.1 tag, crate
  still 0.1.0, `git diff a23269d..HEAD -- .github/` empty, darwin deferral still stated
  verbatim); GUIDANCE-FIELD-DECLARATION-CHANNEL's deferred condition holds (window touches
  neither kind.ts nor any field-guidance surface).
- Queue: 2 pending — 0 open, 1 deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked
  (PACKAGING-CHANNELS-REMAINDER). Open forks: 2 (multi-harness-projection, lazy-grounds),
  unchanged. Friction: 0. Amendments: 1 (plan-export-consumer-reexport-carveout, still awaiting
  human ratification). Inbox: 0 notes.

Plan continues: yes — the posture rotation (sdk/src/index.ts next) is the only live job and the
queue has no pickable entry to hand the wave, so plan drives the sweep itself next tick.
