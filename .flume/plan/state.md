# Plan state

- Spec derived through: a9f7b9e
- Audited through: 99a79ec
- Residue swept through: 99a79ec
- This tick: Post-ship reconcile cefe608..HEAD — the three queued dogfood
  entries shipped (EMBED-RENDER-FENCE-FREE f2d73da, MENTION-EMBEDDED-TARGETS
  cf3d2e2, DRIFT-REAP-PATH-NORMALIZE e7b859a) and are already out of
  pending.json. Audit: EMBED-RENDER-FENCE-FREE was EMBED-FILL-DEFER's blocker
  — verified shipped on disk (sdk/src/emit.ts renderMemberBlock, fence-free
  render path), so EMBED-FILL-DEFER flips blockedBy→open; its emit.ts refs
  re-derived to the current `refuseBrokenSource` (was mis-cited `checkFills`;
  200/202-213/226-231/205-210) and drift.rs layout-satisfies merge (~755/881,
  was 672). PACKAGING-CHANNELS-REMAINDER stays parked — window touched no
  release infra. Sweep: window opened prose.ts (cf3d2e2, +9) — the pre-recut
  vocabulary rider's line numbers re-derived (law5 5/92/193, posture
  77/107/109/136/185), narration left undischarged, no fresh stale vocab.
  f2d73da's `renderMemberFence`→`renderMemberBlock` rename left one stale
  test-comment cite (emit.test.ts:853) — new comment-staleness rider filed.
  No other rider file touched; no structural residue. Both cursors → HEAD.
- Queue: EMBED-FILL-DEFER (open — unblocked, pickable, sdk/src/emit.ts);
  PACKAGING-CHANNELS-REMAINDER (parked, human release actions).

Plan continues: no — inbox empty, spec cursor current (no specs commits past
a9f7b9e), reconciliation window drained. Build has one open pickable entry
(EMBED-FILL-DEFER).
