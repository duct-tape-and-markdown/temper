# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 34bddb94 — the window's two ships verified on disk, both
  regressions green.
- Residue swept through: 34bddb94 — same window, swept with the audit.
- Posture swept through: 7d695577 — copied forward; its forward window is
  220 commits wide.
- This tick: reconciled f09e8171..HEAD — both ships verified, and the sweep
  filed NESTED-MEMBER-FIXTURE-JOINS-THE-ONE-BUILDER over the three bare
  `PayloadMember` literals eb0800fb added after 2ab8a85e's fold.
- Queue: 7 pending — 1 open, 3 parked, 3 deferred. Pickable: 1 —
  NESTED-MEMBER-FIXTURE-JOINS-THE-ONE-BUILDER. Open forks: 1. Friction: 2.
  Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the only live job left is the posture sweep's
open rotation, and the one pickable entry ships first.
