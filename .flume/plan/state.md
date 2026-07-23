# Plan state

- Spec derived through: aee005d — advanced from c2c7365. The one queued
  commit (pipeline.md Drift EOL-blind clause) routed: already fully
  derived last tick into DRIFT-COMPARE-EOL-CANONICALIZE (filed e2b9d1f
  from inbox item 10, same ratification). Re-verified against Drift
  186-205 and the entry on disk: per-cite, 4-site scope, and notes all
  match the clause exactly — no residual scope, no new entry needed.
- Audited through: c38c101 — unchanged this tick.
- Residue swept through: c38c101 — unchanged this tick.
- Posture swept through: mid-rotation, at src/graph.rs — src/hash.rs next
  in the c9d11d5 rotation's frontier, untouched this tick.
- This tick: SPEC DELTA. Routed aee005d — no new entry (see above); cursor
  advanced. See commit body.
- Queue: 3 pending — 1 open (DRIFT-COMPARE-EOL-CANONICALIZE), 1 parked, 1
  deferred. Open forks: 2, unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — audited/residue-swept window (c38c101..HEAD) has
unreconciled src/ commits (b5c5a5d and 5 more); post-ship reconciliation
is next tick's live job.
