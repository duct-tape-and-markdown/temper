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
- Posture swept through: cbdde828 mid-rotation — install.rs/frontmatter.rs/
  placement.rs/tests/install.rs now all covered; sdk/src/builtins.ts
  (re-touched by 7258ad7a since this rotation's cbdde828 baseline) remains
  the open frontier.
- This tick: POSTURE SWEEP tests/install.rs neighborhood — filed
  GUARD-DRIVER-TRIPLICATE-CONSOLIDATE (details in commit body).
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3 (+1
  committed-settings-kind). Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — the new open entry ships first; the sweep
resumes over sdk/src/builtins.ts once the wave hands back.
