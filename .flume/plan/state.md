# Plan state

- Spec derived through: 862f0b60 — copied forward: the 96e6d497 delta is unrouted.
- Audited through: e253fd44 — copied forward: the 5866aea3..e9573d2e window is unreconciled.
- Residue swept through: e253fd44 — copied forward, same window.
- Posture swept through: 7d695577 — the rotation armed at that sha is closed; it
  re-arms over 7d695577..HEAD once the jobs above are serviced.
- This tick: spec delta, one slice — 0063 routed as two open forks, no entries filed.
- Queue: 9 pending — 2 open, 3 deferred, 4 parked. Pickable: 2 —
  DIRECTIVE-BACKING-RESOLVES-BY-STAT and EMBEDDED-EDGE-TARGET-DEFERS-TO-CHECK,
  disjoint (Rust gate vs SDK emit). Open forks: 3. Friction: 2 (human channel).
  Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the spec delta (96e6d497, decisions 0064-0073) is unrouted
