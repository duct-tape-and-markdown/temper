# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: 5eb7177 — window b1b385d..5eb7177 reconciled;
  GLOB-SEMANTICS-TESTS-RELOCATE-TO-GLOB-RS verified shipped and already
  dropped from pending.json.
- Residue swept through: 5eb7177 — same window; one finding, routed into
  open-questions.md (full DATUM in the commit body), no pending-entry work.
- Posture swept through: mid-rotation, at src/kind.rs — unchanged this
  tick (job 3 outranked job 4); src/layout.rs next in the c9d11d5
  rotation's frontier.
- This tick: POST-SHIP RECONCILIATION b1b385d..5eb7177 — clean audit, one
  orphan discharge routed (full DATUM in the commit body).
- Queue: 2 pending — 1 parked, 1 deferred, 0 open. Open forks: 2,
  unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — the posture rotation is still open (frontier
non-empty: src/layout.rs onward) with no pickable entry in the queue
right now, so plan drives the sweep itself next tick.
