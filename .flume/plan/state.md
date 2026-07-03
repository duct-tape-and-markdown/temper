# Plan state

- **Phase:** reconcile. HEAD f534fe9.
- **Last shipped:** EMIT-VERB (`build` 44df9a8 / `chore` f534fe9) — `apply`
  renamed to `emit`; the projection re-emits whole, byte-deterministic and
  double-emit verified, and the three-state merge model is dropped. Verified on
  disk: `Command::Emit` dispatches `drift::emit`/`render_emit` (main.rs:158-380),
  the `--frozen`/`--dry-run` options stand, no `Apply`/`ApplyOutcome::Conflicted`
  survive.
- **This tick:** inbox empty. Reconciled the queue: EMIT-VERB shipped, so
  MANIFEST-EMIT's dangling `blockedBy` flips to `open` — the new chain head.
  Verified MANIFEST-EMIT unbuilt on disk — `AuthorLayer` still only *reads* an
  optional overlay `temper.toml` (compose.rs:26); import writes `lock.toml`, not
  a serialized manifest.
- **Pickable now:** **MANIFEST-EMIT** (`open`). Serialized behind it:
  MANIFEST-GATE-READ → INIT-ONRAMP → EMIT-OWNED-PLACEMENTS (each `blockedBy` the
  prior). Deferred/parked: EXTRACTION-VOCAB-GAPS, AGENT-KIND (no consumer),
  PACKAGING-CHANNELS (creds).
- **What's next:** build ships MANIFEST-EMIT; the next reconcile flips
  MANIFEST-GATE-READ `open`. Human still owes ask (a) (SDK scaffolding) before any
  altitude-rung entry can be filed.

Plan continues: no — queue reconciled, MANIFEST-EMIT is a pickable head, inbox
drained. Hand to build; re-planning would re-emit the same queue.
