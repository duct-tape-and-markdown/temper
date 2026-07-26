# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 9c2db5f4 — unchanged; window 9c2db5f4..82f9c9b9 (current
  HEAD) touches no src/tests/sdk/src (harness/docs/flume-only commits) —
  nothing to reconcile.
- Residue swept through: 9c2db5f4 — unchanged, same empty window.
- Posture swept through: cbdde828 mid-rotation — unchanged; tests/install.rs
  remains the open frontier.
- This tick: INBOX — routed all 3 lines (observed at 543c9f1/0f44dbb). Filed
  SETTINGS-LOCAL-AUTO-MEMORY-KEYS (open, per specs/builtins.md "The shipped
  kinds"): type autoMemoryEnabled/autoMemoryDirectory on the SettingsLocal
  interface. Registered open fork (committed-settings-kind): the same note's
  claim about the committed .claude/settings.json half has no governing kind
  to type against — a human ruling, not a build entry. Verified-moot: the
  note's agent-kind `memory`-field claim was already false at filing (typed
  by e76934e3, which shipped before observed-sha 543c9f1) — no entry. Drained
  the MEMORY.md-documents note (accepted debt, no action — identity and
  projection both already ruled out) and the WITHDRAWN tap stub per its own
  instruction.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3 (+1
  committed-settings-kind). Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — the posture rotation is still open
(tests/install.rs left in the frontier), but SETTINGS-LOCAL-AUTO-MEMORY-KEYS
is now pickable, so build ships it first and the sweep resumes after.
