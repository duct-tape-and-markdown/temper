# Plan state

- Spec derived through: a0fccaf
- Audited through: 008a995
- Residue swept through: 503de24
- This tick: Ship audit — EMBEDDED-EDGE-TARGETS verified on disk (ca1e413:
  `assemble_by_kind` main.rs:1008 takes the embedded corpus,
  `embedded_features_by_kind` main.rs:1045, `embedded_member_features`
  main.rs:1080; tests/graph.rs `embedded_edge_targets` module covers
  resolve-by-identity, dangling→route finding, unmodeled→admissibility
  finding). BUILTIN-CONTRACT-ARRAY-SURGERY unblocked (blockedBy was
  file-serialization only) and re-anchored (gate 590→591; 292/299,
  compose.rs:110/108, contract.rs:6, schema.rs:6, acceptance.rs:36 hold).
  LOCK-ROW-REJECT-LOUD re-anchored (edge read main.rs:1184→1269
  `edges_from_declarations`; layout_edge_fields filter_map at drift.rs:694;
  drift.rs/kind.rs otherwise untouched). PACKAGING-CHANNELS parked reason
  re-verified (no release.yml, root package.json still private flume
  manifest, SDK 0.0.5).
- Queue: BUILTIN-CONTRACT-ARRAY-SURGERY (open); LOCK-ROW-REJECT-LOUD
  (blockedBy BUILTIN-CONTRACT-ARRAY-SURGERY); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep (cursor 503de24 trails HEAD; ca1e413 is
unswept code).
