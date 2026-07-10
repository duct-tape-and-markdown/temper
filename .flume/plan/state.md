# Plan state

- Spec derived through: a0fccaf
- Audited through: 2f9e277
- Residue swept through: e5a7077
- This tick: Ship audit. LAYOUT-RELATIONSHIP-EDGES verified on disk at
  fcaf08e: layout_edge_fields (drift.rs:690) derives the edge-field set
  from assembly facts, consumed on both document-read paths (drift.rs:567,
  main.rs:887); relationship-species tests present and green (suite run:
  4 passed); the file's local scaffold retired for tests/common. Gate
  re-test: EMBEDDED-EDGE-TARGETS' blocker shipped and its gap re-verified
  live (assemble_by_kind, now main.rs:1006, still keys only kind-fact-row
  kinds) — flipped to open. Downstream blockedBy chain unchanged
  (file-serialization on main.rs/drift.rs, still true). PACKAGING park
  reason re-verified (no release.yml; root package.json still the private
  flume manifest). Queue anchors re-set after the drift.rs/main.rs churn
  (family 2179, source_deps 1402, read_declarations 2110, narration
  1401/2144/2274/2593/2711, edge-row read main.rs:1184).
- Queue: EMBEDDED-EDGE-TARGETS (open); BUILTIN-CONTRACT-ARRAY-SURGERY
  (blockedBy EMBEDDED-EDGE-TARGETS); LOCK-ROW-REJECT-LOUD (blockedBy
  BUILTIN-CONTRACT-ARRAY-SURGERY); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep trails HEAD (e5a7077 < 2f9e277;
fcaf08e's code commit is unswept, and the six riding debts are due their
per-sweep re-verify).
