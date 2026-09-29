# Plan state

- Spec derived through: 862f0b60 — copied forward: 96e6d497 stays unrouted
  (0064, 0067-0073 remain; 0063 and this tick's 0065/0066 are routed).
- Audited through: e253fd44 — copied forward: the 5866aea3..e9573d2e window is unreconciled.
- Residue swept through: e253fd44 — copied forward, same window.
- Posture swept through: 7d695577 — the rotation armed at that sha is closed; it
  re-arms over 7d695577..HEAD once the jobs above are serviced.
- This tick: spec delta, one slice — 0065 filed as one entry, 0066 verified moot
  on disk with its one stale comment riding that entry.
- Queue: 10 pending — 3 open, 3 deferred, 4 parked. Pickable: 3 —
  DIRECTIVE-BACKING-RESOLVES-BY-STAT, EMBEDDED-EDGE-TARGET-DEFERS-TO-CHECK and
  LAYOUT-TITLE-IS-THE-DOCUMENTS-OWN-SPAN, disjoint (Rust gate vs SDK emit vs the
  layout reader). Open forks: 3. Friction: 2 (human channel). Amendments: 0.
  Refactor: 0. Inbox: 0.

Plan continues: yes — 96e6d497 still carries 0064 and 0067-0073 unrouted
