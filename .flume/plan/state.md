# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: f88e96d — unchanged; no window since touched src/, tests/, or sdk/ (only plan commits followed).
- Residue swept through: f88e96d — unchanged, same reason.
- Posture swept through: src/builtin.rs..src/json_splice.rs (prior rotation), plus src/kind.rs covered this tick — mid-rotation. src/layout.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, src/kind.rs — whole file plus its immediate imports (compose::Edge, drift's row/error types, extract::Features, layout::Layout) read. One violation found and filed: `entry_shape_to_label` (948-965) is zero-consumer dead code behind `#[allow(dead_code)]`, grep-verified across src/, tests/, sdk/ with no caller anywhere. Filed KIND-ENTRY-SHAPE-LABEL-PRUNE (per engineering.md, "An export earns its consumer"). Every other pub/pub(crate) surface re-checked has a live external caller (MARKETPLACE_FIELD, SATISFIES_EDGE_FIELD, identity_edge, commitment_from_row/format_from_row/content_from_row/collection_address_from_row all grep-confirmed consumed outside the module). No cohesion, dead-plumbing, or embedded-provider-knowledge violation found otherwise.
- Queue: 3 pending, 1 open (KIND-ENTRY-SHAPE-LABEL-PRUNE, new this tick), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched, not re-tested this tick). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — build ships KIND-ENTRY-SHAPE-LABEL-PRUNE; the posture sweep rotation (still open, frontier non-empty from src/layout.rs onward) resumes when the wave hands back.
