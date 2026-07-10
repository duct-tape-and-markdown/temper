# Plan state

- Spec derived through: a0fccaf
- Audited through: eab6979
- Residue swept through: e5a7077
- This tick: Quiet closing pass. eab6979..HEAD is .flume/plan-only — no code
  moved, so every cursor's premise stands. All queued anchors re-verified on
  disk (layout_unit main.rs:818, edge_field_slots kind.rs:609, drift.rs:704
  single-slot set, compose::effective compose.rs:110, assemble_by_kind
  main.rs:985, family/filter_map drift.rs:2160-2164, local scaffold
  layout_edge_slot.rs:61, tolerant edge-row read main.rs:1163). PACKAGING
  park reason still true (no release.yml; root package.json still the
  private flume manifest). Queue disjoint: the four-entry chain serializes
  its main.rs/drift.rs overlaps via blockedBy; PACKAGING touches neither.
  Inbox empty, no refactor captures.
- Queue: LAYOUT-RELATIONSHIP-EDGES (open); EMBEDDED-EDGE-TARGETS (blockedBy
  LAYOUT-RELATIONSHIP-EDGES); BUILTIN-CONTRACT-ARRAY-SURGERY (blockedBy
  EMBEDDED-EDGE-TARGETS); LOCK-ROW-REJECT-LOUD (blockedBy
  BUILTIN-CONTRACT-ARRAY-SURGERY); PACKAGING-CHANNELS (parked).

Plan continues: no — all inputs current; LAYOUT-RELATIONSHIP-EDGES is
pickable, build takes over.
