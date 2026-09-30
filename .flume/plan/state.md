# Plan state

- Spec derived through: 86a4b214 — copied forward; the delta past it is empty.
- Audited through: 040e2d1c — e9c6820b..HEAD reconciled, both motions.
- Residue swept through: 040e2d1c — one gap filed, one candidate cleared.
- Posture swept through: 7d695577 — copied forward; its forward window is
  202 commits wide.
- This tick: reconciled e9c6820b..HEAD — two ships verified on disk, and the
  lock's last two hand-rolled readers filed as one fold.
- Queue: 9 pending — 2 open, 1 blockedBy, 3 parked, 3 deferred. Pickable: 2 —
  LOCUS-STRANGER-FINDING-ASSERTS-ONLY-WHAT-IT-KNOWS and
  LOCK-ENGINE-SKEW-NAMES-BOTH-VERSIONS, file-disjoint and on different
  seams (message text vs. load-fault lowering); both ripples reconciled as
  noise this tick. SOURCE-DEP-LOCK-READS-HAVE-ONE-HOME waits on LOCUS-STRANGER
  over src/drift.rs. Open forks: 1. Friction: 2 (both live). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep is the only live plan job and
two entries are pickable, so the wave ships first.
