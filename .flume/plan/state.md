# Plan state

- Spec derived through: 86a4b214 — copied forward; the delta past it is empty.
- Audited through: e9c6820b — copied forward.
- Residue swept through: e9c6820b — copied forward.
- Posture swept through: 7d695577 — copied forward; its forward window is
  202 commits wide.
- This tick: drained the inbox — three notes routed into two open entries,
  both reproduced on disk before scoping.
- Queue: 8 pending — 2 open, 3 parked, 3 deferred. Pickable: 2 —
  LOCUS-STRANGER-FINDING-ASSERTS-ONLY-WHAT-IT-KNOWS and
  LOCK-ENGINE-SKEW-NAMES-BOTH-VERSIONS, file-disjoint and on different
  seams (message text vs. load-fault lowering). LOCUS-STRANGER shares
  src/install.rs only with a deferred entry, never picked. Open forks: 1.
  Friction: 2 (both live). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation of e9c6820b..HEAD, two ships
(26b5798d, d48ce8c7) unaudited and unswept.
