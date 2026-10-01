# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: 8b8fcdf0 — three ships verified on disk, six non-pickable
  gates re-tested and all still true.
- Residue swept through: 8b8fcdf0 — same window; one gap filed, one staleness
  routed to the entry that rides it.
- Posture swept through: 7d695577 — copied forward; its forward window is
  206 commits wide.
- This tick: reconciled 13b3d9b5..8b8fcdf0 — audit and sweep in one window,
  LEAF-MENTION-ROW-HEADS-ITS-HOST-ADDRESS filed off e9c80335's own deferral.
- Queue: 11 pending — 2 open, 3 blockedBy, 3 parked, 3 deferred. Pickable: 2 —
  ADDRESS-GRAMMAR-READS-ANY-DEPTH, NESTED-CHILD-KEY-ROUND-TRIPS-ITS-PATTERN.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep is the only live job left (its
cursor sits 206 commits behind HEAD) and two pickable entries exist, so the
ready work ships first and the sweep resumes when the wave hands back.
