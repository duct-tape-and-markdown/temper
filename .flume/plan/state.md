# Plan state

- Spec derived through: a0fccaf
- Audited through: 8575dd6
- Residue swept through: df667e4
- This tick: Inbox drain — routed both notes. (1) prose.ts NUL/`\x01`
  sentinels re-verified still literal on disk (grep exit=1 without `-a`)
  → filed PROSE-SENTINEL-ESCAPE (open); the standing "rides next-open"
  exit was inert (no queued entry opens prose.ts), so promoted standalone
  and re-pointed the open-questions sweep-mechanics NB. (2) `impact`
  re-verified a deliberately-unified internal strand of `explain`
  (main.rs clap has one read variant, read.rs:190 "the one read verb"),
  while the model still names it a peer verb → registered fork
  `(impact-read-verb)`; spec-vs-code collision surfaced, not filled
  (a session doesn't write specs; the code unification looks intentional).
- Queue: PROSE-SENTINEL-ESCAPE (open, pickable) · PACKAGING-CHANNELS
  (parked). Disjoint — sdk/src/prose.ts vs release infra (package.json,
  release.yml) share no path.

Plan continues: yes — post-ship reconciliation is live below inbox:
543f2fd bumped sdk to 0.1.0 past both cursors (8575dd6 / df667e4),
staling PACKAGING-CHANNELS' "still at 0.0.5 pending publish" note.
