# Plan state

- Spec derived through: c9d11d5 — routed in full, 0 new entries; see commit body.
- Audited through: 0d91b96 — window 126c264..0d91b96: one src/-touching commit
  (69c41c6, COMPOSE-ZERO-CONSUMER-VISIBILITY-PRUNE's build tick). Verified on
  disk: `edge_fields_by_kind`/`embedded_member_features` are now plain `fn`,
  `rg` over src/, tests/, main.rs finds no external caller, `cargo test
  compose` is 11/11 green. Ship commit 0d91b96 already dropped the entry from
  pending.json. metrics.jsonl's build tick (16 turns, 77s, 3507 output
  tokens) shows no oversized-split signal.
- Residue swept through: 0d91b96 — same window: a six-line visibility
  narrowing introduces no new symbol, retirement, or vocabulary; no residue
  found, 0 entries filed.
- Posture swept through: mid-rotation, at src/engine.rs — filed
  WHEN-ROOT-GUARD-STRAY-COLON-PREFIX; `src/extract.rs` next in the
  c9d11d5 re-arm rotation's frontier.
- This tick: POSTURE SWEEP src/engine.rs (neighborhood: engine.rs +
  immediate imports check/contract/extract) — filed
  WHEN-ROOT-GUARD-STRAY-COLON-PREFIX, a verified defect; see commit body.
- Queue: 3 pending — 1 open (WHEN-ROOT-GUARD-STRAY-COLON-PREFIX), 1
  deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked
  (PACKAGING-CHANNELS-REMAINDER); 0 open questions unresolved by this
  queue. Open forks: 2, unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: after-build — the posture rotation is open (extract.rs
onward unswept) but a pickable entry now exists
(WHEN-ROOT-GUARD-STRAY-COLON-PREFIX); build ships it first and the sweep
resumes once the wave hands back.
