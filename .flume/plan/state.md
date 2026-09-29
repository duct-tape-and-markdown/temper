# Plan state

- Spec derived through: f26213a0 — copied forward; the specs delta is empty.
- Audited through: e9573d2e — the 5866aea3..e9573d2e window is reconciled, and
  nothing past it touches `src/`, `tests/` or `sdk/`.
- Residue swept through: e9573d2e — the same window, swept beside the audit.
- Posture swept through: 7d695577 — the rotation re-arms now that reconciliation is
  serviced; its forward window is 171 commits.
- This tick: post-ship reconciliation — the window's three shipped entries verified on
  disk, four queue entries corrected against it, none dropped and none newly derivable.
- Queue: 25 pending — 5 open, 14 blockedBy, 3 deferred, 3 parked. Pickable: 5 —
  DIRECTIVE-BACKING-RESOLVES-BY-STAT, EMBEDDED-EDGE-TARGET-DEFERS-TO-CHECK,
  LAYOUT-TITLE-IS-THE-DOCUMENTS-OWN-SPAN, LOCK-NAMES-THE-ENGINE-THAT-WROTE-IT and
  MCP-KIND-SHIPS-IN-THE-SDK — file-disjoint, verified (the gate read vs the SDK emit
  vs the layout reader vs the lock writer vs the provider face). Open forks: 1.
  Friction: 2 (human channel). Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture sweep over 7d695577..HEAD
