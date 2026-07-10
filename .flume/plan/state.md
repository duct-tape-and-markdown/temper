# Plan state

- Spec derived through: 6a04322
- Audited through: ab73a60
- Residue swept through: fcdbe52
- This tick: Ship audit (job 3) — 4f5df8d/FRONTMATTER-MALFORMED-LOUD and
  36a7662/SEAM-BINDINGS-GENERATED verified on disk (FrontmatterError's
  malformed diagnostic + loud tests in src/frontmatter.rs and
  tests/gate_fail_loud.rs; sdk/src/generated/ the one binding home,
  declarations.ts importing it, tests/seam_bindings_current.rs the
  byte-compare gate). TEST-ROW-FIXTURES-ONE-HOME's blockedBy resolved →
  flipped open; its cited line anchors re-verified (neither build commit
  touched its files). Moved anchors restamped: drift.rs KindFactRow
  1080→1085 (KIND-CONTENT-FACT), reap_or_report_orphan →592
  (LAYOUT-READER). PACKAGING-CHANNELS parked reason re-verified at
  ab73a60 (no release.yml; root package.json still the private flume
  manifest). No riding-debt file was opened by either commit — all
  open-question routings stand.
- Queue: TEST-ROW-FIXTURES-ONE-HOME (open) → KIND-CONTENT-FACT →
  LAYOUT-READER → LAYOUT-PROSE-IMPORT (linear blockedBy chain over the
  shared drift.rs/declarations.ts/lock_declaration_rows.rs/main.rs
  surfaces); PACKAGING-CHANNELS (parked, carried with a refreshed
  re-verify stamp).

Plan continues: yes — residue sweep: fcdbe52 trails HEAD ab73a60 with two
unswept code commits (4f5df8d, 36a7662).
