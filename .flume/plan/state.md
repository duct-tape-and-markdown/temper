# Plan state

- Spec derived through: c2c7365 — unchanged; one commit past cursor
  (aee005d, pipeline.md Drift EOL-blind clause) queued for next tick's
  spec-delta job.
- Audited through: c38c101 — unchanged this tick.
- Residue swept through: c38c101 — unchanged this tick.
- Posture swept through: mid-rotation, at src/graph.rs — src/hash.rs next
  in the c9d11d5 rotation's frontier, untouched this tick.
- This tick: INBOX. Drained item 9 (accepted debt — withdrawn, low/optional
  remainder shelved pending a fresh ask) and item 10 (filed
  DRIFT-COMPARE-EOL-CANONICALIZE, per pipeline.md Drift). See commit body.
- Queue: 3 pending — 1 open (DRIFT-COMPARE-EOL-CANONICALIZE), 1 parked, 1
  deferred. Open forks: 2, unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — spec delta unrouted (aee005d, one commit past
cursor) is next tick's live job.
