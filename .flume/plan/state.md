# Plan state

- Spec derived through: 6b80e24
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: drift done — cli-engine next
- This tick: POSTURE SWEEP. Jobs 1-3 re-confirmed quiet at HEAD c2eca2d:
  inbox empty, no refactor-captures, `git log 6b80e24..HEAD -- specs/` and
  `git log 60faee0..HEAD -- src/ tests/ sdk/` both empty. Rotation was
  absent, so it opened at its first subsystem: `drift` (`src/drift.rs`
  alone, 4474 lines). Read the whole file against every section of
  engineering.md as it reads this tick. "One job, one home": the row-walk
  triplication is already the open DRIFT-LOCK-ROW-WALK-CONSOLIDATION entry
  — re-confirmed unmoved (drift.rs untouched since b8fc7ca). Found a fresh
  instance one layer up: `emit()` (fn 901) reads and parses the workspace's
  `lock.toml` twice in one run — the reap-diff loop (1197) via
  `read_prior_provenance` (1918) and the layer-drop check (1244) via
  `read_declarations` (3398) — two independent opens of the same file for
  two different row families (provenance columns vs the `[declaration]`
  sub-table), never shared. Filed DRIFT-EMIT-LOCK-PARSE-HOIST, `per`
  engineering.md's "Cost scale is hoisted, and pinned by count",
  `blockedBy` DRIFT-LOCK-ROW-WALK-CONSOLIDATION (shares `src/drift.rs`;
  its `walk_lock_rows` doc-based extractor is the shape this entry's
  provenance-row side should follow, not re-derive independently).
  Checked every `DriftError`/`LockRowError` variant is constructed
  (none unconstructable) and every `pub` export
  (place/ApplyOutcome/layout_imports/includes/*_stale/emit_owned_targets/
  config_stale/project_bytes/render_emit/layout_edge_fields/
  read_layout_document/LayoutDocumentRows) resolves to a caller in
  install.rs/main.rs/bundle.rs/import.rs or their tests — no dead
  plumbing, no orphaned exports.
- Queue: 5 pending — 2 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; disjoint files, no serialization
  needed), 1 blockedBy DRIFT-LOCK-ROW-WALK-CONSOLIDATION
  (DRIFT-EMIT-LOCK-PARSE-HOIST; shares src/drift.rs, correctly serialized),
  2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks: (multi-harness-projection),
  (lazy-grounds).

Plan continues: yes — posture sweep continues to the roster's next
subsystem, `cli-engine` (main+engine).
