# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: 021c01d — advanced from c38c101. Window's 7 build
  commits (b5c5a5d..d8f85a1) each map to an already-drained pending
  entry (their own `chore(flume): ship` commits removed them); no stale
  gate condition in pending.json depended on this window. Metrics
  glanced: no oversized tick; FRONTMATTER-LOAD-FAULT-AS-DIAGNOSTIC needed
  2 revert/re-attempts before landing but shipped in-band. See commit body.
- Residue swept through: 021c01d — advanced from c38c101. d843ffb's
  command-clause removal checked for residue: none (extraction primitives
  are a distinct, still-accurate concept from the removed default-contract
  clauses; `display::plural` still has live callers after the gate-installed
  count→path-list change). Ride-only orphans in drift.rs (x2), install.rs,
  and compose.rs re-verified against this window's 4 touching commits —
  none reached their cited regions; open-questions.md line numbers and
  shipped-entry lists refreshed to 021c01d. See commit body.
- Posture swept through: mid-rotation, at src/graph.rs — src/hash.rs next
  in the c9d11d5 rotation's frontier, untouched this tick.
- This tick: POST-SHIP RECONCILIATION over c38c101..HEAD. Audit clean,
  no entries dropped; sweep found no fileable residue, refreshed 4
  ride-only orphan records. Both cursors advanced to 021c01d.
- Queue: 3 pending — 1 open (DRIFT-COMPARE-EOL-CANONICALIZE), 1 parked, 1
  deferred. Open forks: 2, unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: after-build — DRIFT-COMPARE-EOL-CANONICALIZE is pickable;
the open posture rotation (src/hash.rs) resumes once the wave hands back.
