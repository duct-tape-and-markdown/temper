# Plan state

- Spec derived through: a0fccaf
- Audited through: e30e5a6
- Residue swept through: ced4f94
- This tick: Inbox drained — the 0020 edge-slot remainder note (37e4834,
  observed e30e5a6) routed into the already-filed LAYOUT-RELATIONSHIP-EDGES:
  same gap, re-verified (no code commits e30e5a6..HEAD; `layout_unit`
  main.rs:816 still drops `LayoutReading::edges`). The note's proposed
  member-authored edge-row lock family is the recorded rejected alternative —
  relationship edges resolve live off frontmatter `Features` (graph.rs), a
  layout document is a source read live at check, so it folds the same way
  lock-free; lock rows exist only where a reference can't be re-read from a
  source (composed prose: mentions, PROSE-INCLUDE's includes). Customer
  evidence (a counterpart corpus's Depends-on sections) carried into the
  entry's notes.
- Queue: PROSE-INCLUDE (open); LAYOUT-RELATIONSHIP-EDGES (blockedBy
  PROSE-INCLUDE); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep cursor (ced4f94) trails HEAD; only plan
and chore(flume) commits since, so a trivial advance is expected.
