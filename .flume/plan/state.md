# Plan state

- Spec derived through: f26213a0 — copied forward; the specs delta is empty.
- Audited through: dca24238 — three ships verified on disk this tick.
- Residue swept through: dca24238 — swept in the same motion.
- Posture swept through: 7d695577 — copied forward; the rotation is now the
  only live plan job, and its forward window is 185 commits wide.
- This tick: reconciled 039eb92b..HEAD — all three ships verified on disk, no
  entry dropped, every stale gate re-tested still true, and EMPTY-CONTRACT's
  ripple closed by folding the example lock's byte-compare (tests/it/emit.rs)
  into its fence.
- Queue: 16 pending — 2 open, 8 blockedBy, 3 parked, 3 deferred. Pickable: 2 —
  EMPTY-CONTRACT-IS-A-DECLARED-CONTRACT and NO-TEST-JUDGES-A-RETIRED-FINDING-CLASS,
  file-disjoint. Open forks: 1. Friction: 3 (human channel; the snapshot-glob
  capture is discharged by b7b55457). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep, behind two pickable entries
