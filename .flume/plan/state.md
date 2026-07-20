# Plan state

- Spec derived through: 946e303 — advanced from caf29fa. This tick's inbox drain routes decision
  0045's Consequences in full: the spec-text bullet (contract.md's own generalization) is already
  ratified in the commit itself; the SDK/lock-fact and engine-delivery bullets are filed as
  GUIDANCE-KIND-SDK-LOCK-FACT and GUIDANCE-KIND-CONTRACT-SCHEMA-EXPLAIN; the built-in-notes bullet is
  GUIDANCE-BUILTIN-NOTES-PROMOTE; the field-declaration half of the SDK bullet is deferred as
  GUIDANCE-FIELD-DECLARATION-CHANNEL (no consumer — none of the six promoted notes target a field).
- Audited through: 4ab6fe2 — unchanged; `git log 4ab6fe2..HEAD -- src/ tests/ sdk/` is empty (946e303 is spec-only).
- Residue swept through: 4ab6fe2 — unchanged, same empty window.
- Posture swept through: sdk/src/declarations.ts (+ its immediate imports assembly.ts/kind.ts/contract.ts/prose.ts/builtins.ts) covered; sdk/src/dial.ts next in rotation. Unchanged this tick.
- This tick: INBOX. Decision 0045's guidance-channel note (observed at 946e303) routed into a
  3-entry chain matching its own (a)/(b)/(c) split — GUIDANCE-KIND-SDK-LOCK-FACT (blockedBy
  DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE, both edit sdk/src/declarations.ts), GUIDANCE-KIND-CONTRACT-SCHEMA-EXPLAIN
  (blockedBy it), GUIDANCE-BUILTIN-NOTES-PROMOTE (blockedBy that) — plus GUIDANCE-FIELD-DECLARATION-CHANNEL,
  deferred. Mid-tick, pending.json gained a second, independent derivation of the same note
  (GUIDANCE-KIND-FIELD-SDK-CHANNEL / GUIDANCE-KIND-FIELD-SCHEMA-EXPLAIN-DELIVERY /
  BUILTINS-STRANDED-NOTES-PROMOTE-TO-GUIDANCE) from outside this session; its builtins.ts promotion
  covered only 3 of the 6 stranded notes (missed `rule`/`memory`/`hook`) and its SDK entry carried
  guidance on `KindOptions` while describing it as erased-like-`render`, inconsistent with its own
  claim that `kindFactRow()` serializes it. This tick's chain supersedes it; all three dropped.
  Also re-fixed the live `notes`-length gate violation on disk (DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE
  had drifted to 509 chars, over the 500 cap — the prior tick's revert cause) — trimmed, same substance.
- Queue: 7 pending — 1 open (DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE), 3 chained blockedBy in the
  guidance work, 1 deferred (guidance, field-declaration half), 2 parked (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks: 2 (multi-harness-projection, lazy-grounds). Friction: 0.
  Amendments: 0. Inbox: 0 notes.

Plan continues: after-build — the only remaining live job is the dial.ts posture rotation, and
DECLARATIONS-ZERO-CONSUMER-EXPORTS-PRUNE is pickable now, unblocking the guidance chain behind it.
