# Plan state

- Spec derived through: f26213a0 — copied forward; the specs delta is empty.
- Audited through: bf6e421d — copied forward; bf6e421d..HEAD (two ships) is
  next tick's window.
- Residue swept through: bf6e421d — copied forward, same window.
- Posture swept through: 7d695577 — copied forward; its forward window is
  186 commits wide.
- This tick: drained the HOOK-AUTHORS fence capture — the entry re-scoped with
  its breakage path and two ripple consumers, its staleness half split into a
  blocked entry, capture deleted.
- Queue: 13 pending — 1 open, 6 blockedBy, 3 parked, 3 deferred. Pickable: 1 —
  HOOK-AUTHORS-ITS-HANDLERS, head of the now-7-deep hook spine and the whole
  queue's gate. Open forks: 1. Friction: 2 (both live). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation of bf6e421d..HEAD, the two hook
ships (b158bd53, 6f8d2eab) neither motion has covered.
