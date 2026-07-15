# Plan state

- Spec derived through: f67303c
- Audited through: 226199b
- Residue swept through: 226199b
- This tick: reconciled the c0bbf3b..HEAD ship window — one build commit,
  LAYOUT-OVERLAY-CHECK-GAP (73c54bf), the only window commit touching
  src/tests (bf609e6 is the prior plan tick, 226199b the chore ship). AUDIT:
  LAYOUT-OVERLAY shipped and is already off pending (build removed it) —
  verified on disk: `CustomKind::overlay_content` + shared `layout_from_row`
  lift (kind.rs:715/878), the `overlay_content` call + `overlaid.content`
  dispatch (main.rs:885/981). No queued entry's work is done → no drops.
  Re-tested the held gates: the fork `(lock-upgrade-migration-posture)` is
  still OPEN, so SATISFIES-LABEL-QUALIFY + EMIT-INTO-REROOT-REAP (open but
  dependsOnForks) and LOCK-SPELLING-REAP (blockedBy SATISFIES + same fork) all
  stay held; PACKAGING parked. LAYOUT-OVERLAY was the one pickable entry — none
  remains. SWEEP: LAYOUT-OVERLAY unified `content_from_row` and
  `overlay_content` on one `layout_from_row` lift (commit body names the share)
  — consolidation, not a second layout reader; no new residue. The window added
  +2 lines above the satisfies fold in main.rs (hunks at 854/882/989):
  refreshed SATISFIES's `row.member == unit.id` loop citation 1017/1020→
  1019/1022 (EMIT-INTO's main.rs:134,308-316 sit above the shift, unmoved;
  drift.rs + declarations.ts cites untouched). No rider names a file this window
  opened by pinned line (kind.rs's `15-kinds.md` fixture strings at
  1301/1310/1406 are body text, not a line-pinned cite).
- Queue: no pickable entry. SATISFIES-LABEL-QUALIFY + EMIT-INTO-REROOT-REAP
  (open, dependsOnForks `(lock-upgrade-migration-posture)`), LOCK-SPELLING-REAP
  (blockedBy SATISFIES + same fork), PACKAGING-CHANNELS-REMAINDER (parked) all
  held. Disjoint.

Plan continues: no — window reconciled, both cursors to HEAD (226199b); inbox
empty, spec cursor f67303c is specs/ HEAD (no delta). The whole queue is held
by the lock-upgrade fork or parked — no pickable entry; the loop hibernates
until the fork resolves through the inbox. (NB the SessionStart reporter shows
the `.temper` dogfood gate red — two unfilled requirements; harness territory,
a `chore(harness)` fix outside plan's writable paths.)
