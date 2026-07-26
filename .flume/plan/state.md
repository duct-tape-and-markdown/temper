# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: bb27d6ab — advanced from 9c2db5f4; window
  9c2db5f4..bb27d6ab touched sdk/src exactly once (7258ad7a). Verified on
  disk: SettingsLocal now types autoMemoryEnabled/autoMemoryDirectory,
  fields-only with a doc-comment cite, matching the entry precisely; `pnpm
  --dir sdk test` green (141/141). Ship commit (bb27d6ab) already removed
  the entry from pending.json; metrics.jsonl logs a clean shipped tick, no
  revert.
- Residue swept through: bb27d6ab — same window; no retirement named, no
  stray vocabulary, nothing to file.
- Posture swept through: cbdde828 mid-rotation — unchanged; tests/install.rs
  remains the open frontier.
- This tick: RECONCILE — audited+swept 9c2db5f4..bb27d6ab. Verified
  SETTINGS-LOCAL-AUTO-MEMORY-KEYS shipped clean against its acceptance;
  nothing else in the window; nothing further to file.
- Queue: 3 pending — 0 open, 1 parked, 2 deferred. Open forks: 3 (+1
  committed-settings-kind). Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — the posture rotation is still open (tests/install.rs
frontier) and the queue holds no pickable (open) entries, so plan drives the
sweep itself next tick.
