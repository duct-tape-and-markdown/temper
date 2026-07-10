# Plan state

- Spec derived through: a0fccaf
- Audited through: eab6979
- Residue swept through: c2a8cae
- This tick: Ship audit — PREDICATE-CONSTRUCTORS (c8b78c5: five constructors
  contract.ts:95-111, ClauseRowError loud-reject compose.rs:162/198) and
  TEST-SCAFFOLD-ONE-HOME (5d0ea5b: tests/common `scaffold`, two suites
  converted; layout_edge_slot.rs's copy rides as scoped) verified on disk.
  LAYOUT-RELATIONSHIP-EDGES opened (blocker shipped); its anchors and
  EMBEDDED-EDGE-TARGETS' re-set after the churn (drift.rs 702→704,
  graph.rs 173→157, declarations.ts 200-202→211-213); PACKAGING-CHANNELS'
  parked reason re-verified (no release.yml; root package.json still the
  private flume manifest).
- Queue: LAYOUT-RELATIONSHIP-EDGES (open); EMBEDDED-EDGE-TARGETS (blockedBy
  LAYOUT-RELATIONSHIP-EDGES); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep (c2a8cae trails HEAD; c8b78c5 and
5d0ea5b are unswept code commits).
