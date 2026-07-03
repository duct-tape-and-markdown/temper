# Plan state

- **Phase:** reconcile. HEAD 3804f1c.
- **Last shipped:** READD-RETIRE — the re-add reverse round-trip retired (`build`
  caa30c4 / `chore` 3804f1c). Chain head drained; no `ReAdd` symbol left in src/
  (only stale prose comments, swept as downstream links rewrite their files).
- **This tick:** inbox empty. Reconciled the floor-migration chain against disk:
  READD-RETIRE shipped, so **flipped LOCK-FRESHNESS-FACTS `blockedBy` → `open`**
  (verified its blast radius is still live — `import_hash`/`last_applied` sit
  exactly at the cited lines in import.rs/drift.rs/frontmatter.rs/document.rs).
  The rest of the chain stays `blockedBy` its still-pending upstream; the
  deferred/parked entries are untouched.
- **Pickable now:** **LOCK-FRESHNESS-FACTS** (chain head). Held behind it:
  EMIT-VERB → MANIFEST-EMIT → MANIFEST-GATE-READ → INIT-ONRAMP →
  EMIT-OWNED-PLACEMENTS (each `blockedBy` the prior). Deferred/parked:
  EXTRACTION-VOCAB-GAPS, AGENT-KIND (no consumer), PACKAGING-CHANNELS (creds).
- **What's next:** build ships LOCK-FRESHNESS-FACTS; the next reconcile flips
  EMIT-VERB `open`. Human still owes ask (a) (SDK scaffolding) before any
  altitude-rung entry can be filed.

Plan continues: no — inbox drained, the floor chain has a pickable head
(LOCK-FRESHNESS-FACTS), everything else is correctly serialized or human-gated.
Hand to build; re-planning would re-emit the same queue.
