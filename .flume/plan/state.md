# Plan state

- Spec derived through: 862f0b60 — copied forward: 96e6d497 stays unrouted
  (0072-0073 remain; 0063-0071 are routed), and f26213a0 (0074-0075) behind it.
- Audited through: e253fd44 — copied forward: the 5866aea3..e9573d2e window is unreconciled.
- Residue swept through: e253fd44 — copied forward, same window.
- Posture swept through: 7d695577 — the rotation armed at that sha is closed; it
  re-arms over 7d695577..HEAD once the jobs above are serviced.
- This tick: spec delta, one slice — 0071 filed as three chained entries (SDK
  kind, embedded lock, advisory retirement), every consequence bullet accounted.
- Queue: 18 pending — 5 open, 6 blockedBy, 3 deferred, 4 parked. Pickable: 5 —
  DIRECTIVE-BACKING-RESOLVES-BY-STAT, EMBEDDED-EDGE-TARGET-DEFERS-TO-CHECK,
  LAYOUT-TITLE-IS-THE-DOCUMENTS-OWN-SPAN, LOCK-NAMES-THE-ENGINE-THAT-WROTE-IT
  and MCP-KIND-SHIPS-IN-THE-SDK, disjoint (Rust gate vs SDK emit vs the layout
  reader vs the lock writer vs the provider face). Open forks: 1. Friction: 2
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — 96e6d497 still carries 0072-0073 unrouted
