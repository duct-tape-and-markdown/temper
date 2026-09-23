# Plan state

- Spec derived through: fcf719f4 — unchanged, copied forward; the spec delta
  is drained.
- Audited through: 9993fc0d — the five build commits past 59857dd5 verified on
  disk; every shipped entry was already off the queue.
- Residue swept through: 9993fc0d — two gaps filed: the example harness's
  ungated committed lock, and `degree`'s address dropping its field filter.
- Posture swept through: tests/tap.rs next — unchanged, copied forward.
- This tick: post-ship reconciliation over 59857dd5..HEAD — audit and sweep in
  one window, two entries filed, no cursor split.
- Queue: 27 pending — 1 open, 18 blockedBy, 3 deferred, 5 parked. Pickable: 1.
  Open forks: 12. Friction: 0. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the window is closed and the queue has a pickable
entry; the posture rotation (tests/tap.rs) resumes when the wave hands back.
