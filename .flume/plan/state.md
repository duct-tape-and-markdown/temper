# Plan state

- Spec derived through: 6a04322
- Audited through: f3a5356
- Residue swept through: 08550e5
- This tick: Ship audit (job 3). a538a76 (KIND-CONTENT-FACT) verified on
  disk: KindFactRow.content Option<LayoutRow> at drift.rs:1118 with TOML
  round-trip (content_table/content_from_table); CustomKind.content;
  SDK fact 6 at kind.ts:105 lowered by declarations.ts contentRow;
  generated LayoutRow/LayoutRegionRow bindings present. Both riding exit
  clauses fired — kind.ts's 8 stale spec-path cites cut (grep: zero hits),
  kind.rs's two false inertness comments corrected — both bullets deleted.
  New finding: kind.rs's format-field doc comment (~66) is a third false
  inertness claim (builtin_kind.rs declares it 4×, main.rs:1013 consumes
  it) — logged as a riding bullet, fix named in LAYOUT-READER's kind.rs
  description. LAYOUT-READER unblocked → open. PACKAGING-CHANNELS park
  re-verified (workflows/ still only temper.yml; root package.json still
  the private flume manifest).
- Queue: LAYOUT-READER (open) → LAYOUT-PROSE-IMPORT (blockedBy — shared
  main.rs/drift.rs surfaces); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep (cursor 08550e5 trails HEAD).
