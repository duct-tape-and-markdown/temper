# Plan state

- Spec derived through: 86a4b214 — copied forward; the delta past it is empty.
- Audited through: e9c6820b — four ships verified on disk, every gate re-tested.
- Residue swept through: e9c6820b — one duplicate-surface gap filed.
- Posture swept through: 7d695577 — copied forward; its forward window is
  198 commits wide.
- This tick: reconciled 2cbe66ac..HEAD — four ships and the 0.0.21 wave
  verified, one residue entry filed, one deferred entry's cites refreshed.
- Queue: 8 pending — 2 open, 3 parked, 3 deferred. Pickable: 2 —
  ENGINE-AND-SDK-CARRY-ONE-VERSION and LOCK-READ-COUNTS-HAVE-ONE-HOME,
  mutually disjoint; LOCK-READ shares src/install.rs only with a deferred
  entry, never picked. Open forks: 1. Friction: 2 (both live). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep is the one remaining live
input and two pickable entries ship first.
