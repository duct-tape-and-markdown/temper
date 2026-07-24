# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: b1b385d — advanced from d2ed4e8.
- Residue swept through: b1b385d — advanced from d2ed4e8.
- Posture swept through: mid-rotation, at src/json_splice.rs — unchanged
  this tick; src/kind.rs next in the c9d11d5 rotation's frontier.
- This tick: POST-SHIP RECONCILIATION, window d2ed4e8..b1b385d — one
  commit (1862273) touched tests/, already fully resolved outside
  build (human fixture deletion + open-question drain); audit and
  sweep found no pending entry to drop and no further residue (full
  DATUM in the commit body).
- Queue: 2 pending — 0 open, 1 parked, 1 deferred. Open forks: 2,
  unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — no pickable entries exist to hand build, so the
posture sweep's open rotation drives itself; next tick resumes at
src/kind.rs.
