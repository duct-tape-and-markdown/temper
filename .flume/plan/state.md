# Plan state

- Spec derived through: 86a4b214 — copied forward; 9626a1a6's parity slice is
  already encoded by this tick's entries, so job 2 next tick confirms and
  advances rather than deriving.
- Audited through: 13b3d9b5 — copied forward; four ships land past it.
- Residue swept through: 13b3d9b5 — copied forward, same window.
- Posture swept through: 7d695577 — copied forward; its forward window is
  206 commits wide.
- This tick: drained the inbox (2 notes) and the address-grammar refactor
  capture into a four-entry chain — two address readers filed, the fold/splice
  inverse filed, the depth entry re-scoped onto all three as parents.
- Queue: 10 pending — 2 open, 2 blockedBy, 3 parked, 3 deferred. Pickable: 2 —
  ADDRESS-GRAMMAR-READS-ANY-DEPTH, NESTED-CHILD-KEY-ROUND-TRIPS-ITS-PATTERN.
  Open forks: 1. Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the spec delta (9626a1a6) is unrouted and four ships sit
past the audit cursor; both are queue-shaping, so neither waits on the sweep.
