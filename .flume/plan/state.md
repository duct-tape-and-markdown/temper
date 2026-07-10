# Plan state

- Spec derived through: 6a04322
- Audited through: 5819ac5
- Residue swept through: e9d05f6
- This tick: Ship audit (job 3). LAYOUT-READER verified on disk:
  kind.rs:245 `Layout::read` with loud `LayoutError` refusals;
  main.rs:851 layout dispatch in `resolve_kind_units`; drift.rs:467-536
  layout rows derived into `nested_member` + layout sources
  reap-protected; install.rs:1078 lift exclusion; six tests in
  tests/layout_kind.rs. All three riding doc-staleness exits fired
  (kind.rs CustomKind + format docs, install.rs module doc) — both
  bullets deleted from open-questions. LAYOUT-PROSE-IMPORT unblocked
  (blockedBy → open); PACKAGING-CHANNELS park re-verified at 5819ac5
  (no release.yml; root package.json still the private flume manifest).
- Queue: LAYOUT-PROSE-IMPORT (open); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep trails HEAD (e9d05f6 < 5819ac5).
