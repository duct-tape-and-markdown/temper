# Plan state

- Spec derived through: a0fccaf
- Audited through: e30e5a6
- Residue swept through: ced4f94
- This tick: Residue sweep of 961c9c2..HEAD (one code commit, 670d54e). One
  gap filed: LAYOUT-RELATIONSHIP-EDGES — a layout host's declared-relationship
  edge slot is silently dropped at check (`layout_unit` main.rs:816 discards
  `LayoutReading::edges`; the slot is excluded from frontmatter, so
  `graph::check` main.rs:692 never sees it) vs representation.md "kind" +
  invariant 6; SDK-expressible today (kind.ts:102/105). Serialized behind
  PROSE-INCLUDE (shared drift.rs/main.rs). Otherwise clean:
  `parse_edge_entries` is the tree's only list parse; the kind_facts test
  consolidation verified on disk; all five riding debts re-verified, stamps
  advanced to ced4f94.
- Queue: PROSE-INCLUDE (open); LAYOUT-RELATIONSHIP-EDGES (blockedBy
  PROSE-INCLUDE); PACKAGING-CHANNELS (parked).

Plan continues: yes — quiet pass: closing disjointness/gate re-check before
hand-off to build.
