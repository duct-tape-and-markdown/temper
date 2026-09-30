# Plan state

- Spec derived through: f26213a0 — copied forward; the specs delta is empty.
- Audited through: 839589ee — 839589ee..HEAD is unreconciled and next tick's job.
- Residue swept through: 839589ee — same window, same tick.
- Posture swept through: 7d695577 — copied forward; its forward window is
  186 commits wide and waits behind the reconciliation.
- This tick: drained the flat-handler refactor capture — the wire-key fold
  filed, the two false doc claims routed onto the entry that opens the file,
  the branch-retirement half rejected on verification.
- Queue: 13 pending — 1 open, 6 blockedBy, 3 parked, 3 deferred. Pickable: 1 —
  EVERY-AUTHORED-HOOK-SPELLS-ITS-HANDLERS, the hook spine's head and 6
  entries' gate. Open forks: 1. Friction: 2 (both live). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation of 839589ee..HEAD (three ships
across two build ticks), then the posture sweep's wide forward window.
