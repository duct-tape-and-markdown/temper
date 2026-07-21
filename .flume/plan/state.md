# Plan state

- Spec derived through: c9d11d5 — routed in full, 0 new entries; see commit body.
- Audited through: 35e700b — window 0d91b96..35e700b: one src/-touching
  commit (e6282dd, WHEN-ROOT-GUARD-STRAY-COLON-PREFIX's build tick).
  Verified on disk: engine.rs:1283 pushes `msg` directly (was
  `format!("{}: {msg}", "")`), the mod-tests assertion at 2813 tightened to
  exact-match; `rg` finds no sibling `format!("{}: {` stray-prefix site in
  src/; `cargo test --lib` 298/298 green. Ship commit 35e700b already
  dropped the entry from pending.json. metrics.jsonl's build tick (37
  turns, 167s, 7519 output tokens) shows no oversized-split signal.
- Residue swept through: 35e700b — same window: a two-line fix introduces
  no new symbol, retirement, or vocabulary; no residue found, 0 entries
  filed.
- Posture swept through: mid-rotation, at src/frontmatter.rs — filed
  FRONTMATTER-CLOSING-DELIMITER-ZERO-CONSUMER-PRUNE; `src/fs_util.rs` next
  in the c9d11d5 re-arm rotation's frontier.
- This tick: POSTURE SWEEP src/frontmatter.rs (+ immediate imports
  document.rs's `Satisfies`, kind.rs's `CustomKind`/`UnitShape`) — filed
  FRONTMATTER-CLOSING-DELIMITER-ZERO-CONSUMER-PRUNE; see commit body.
- Queue: 3 pending — 1 open (FRONTMATTER-CLOSING-DELIMITER-ZERO-CONSUMER-PRUNE),
  1 deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked
  (PACKAGING-CHANNELS-REMAINDER); 0 open questions unresolved by this
  queue. Open forks: 2, unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: after-build — the posture rotation is the only remaining
live job and a pickable entry now exists
(FRONTMATTER-CLOSING-DELIMITER-ZERO-CONSUMER-PRUNE), so build ships it
before the sweep resumes at fs_util.rs.
