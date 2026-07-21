# Plan state

- Spec derived through: c9d11d5 — routed in full, 0 new entries; see commit body.
- Audited through: 99b88a7 — window 8dd1436..99b88a7: one src/-touching
  commit (5e9d1fe, GRAPH-IMPORT-HOP-CAP-TO-PROVIDER-FACE's build tick);
  verified clean, see commit body.
- Residue swept through: 99b88a7 — same window: a pure literal relocation
  (pub const + import), no retirement/demolition/stale vocabulary; 0 filed.
- Posture swept through: mid-rotation, at src/graph.rs — filed
  GRAPH-IMPORT-HOP-CAP-TO-PROVIDER-FACE (now shipped); src/hash.rs next in
  the c9d11d5 re-arm rotation's frontier.
- This tick: POST-SHIP RECONCILIATION over 8dd1436..99b88a7 — verified
  MAX_IMPORT_HOPS lives in builtin_kind.rs beside CLAUDE_ROOT with its
  citation intact, graph.rs imports it with no local definition, the
  hop-cap test and clippy/fmt stay green, pending.json already dropped
  the shipped entry in the same commit; residue motion found nothing to
  file; see commit body.
- Queue: 2 pending — 1 deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1
  parked (PACKAGING-CHANNELS-REMAINDER); both gate reasons re-checked,
  unchanged, neither's condition touched by this window. Open forks: 2,
  unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — no pickable entry in queue (both parked/deferred),
so the open posture rotation resumes at src/hash.rs next tick.
