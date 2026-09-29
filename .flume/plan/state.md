# Plan state

- Spec derived through: f26213a0 — 96e6d497's last decision (0073) is routed, so
  the cursor clears it and lands on f26213a0, already routed last tick. The
  specs delta is empty.
- Audited through: e253fd44 — copied forward: the 5866aea3..e9573d2e window is unreconciled.
- Residue swept through: e253fd44 — copied forward, same window.
- Posture swept through: 7d695577 — the rotation armed at that sha is closed; it
  re-arms over 7d695577..HEAD once the jobs above are serviced.
- This tick: spec delta, last slice — 0073 derived as two chained entries onto
  the hook chain's install.rs tail.
- Queue: 25 pending — 5 open, 14 blockedBy, 3 deferred, 3 parked. Pickable: 5 —
  DIRECTIVE-BACKING-RESOLVES-BY-STAT, EMBEDDED-EDGE-TARGET-DEFERS-TO-CHECK,
  LAYOUT-TITLE-IS-THE-DOCUMENTS-OWN-SPAN, LOCK-NAMES-THE-ENGINE-THAT-WROTE-IT
  and MCP-KIND-SHIPS-IN-THE-SDK, disjoint (Rust gate vs SDK emit vs the layout
  reader vs the lock writer vs the provider face). Open forks: 1. Friction: 2
  (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: yes — the 5866aea3..e9573d2e window's audit and residue sweep
