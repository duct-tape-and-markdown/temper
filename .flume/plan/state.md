# Plan state

- Spec derived through: 6a04322
- Audited through: 5819ac5
- Residue swept through: 1284cdf
- This tick: Residue sweep (job 4) over e9d05f6..1284cdf — one code
  commit, e3982b5 (LAYOUT-READER). Structurally clean but one residue
  class: the reader's tests landed as near-verbatim twin suites
  (src/kind.rs:1491-1601 unit tests vs tests/layout_kind.rs, same
  fixture, same public `Layout::read`) — filed LAYOUT-READER-TEST-DEDUP
  (engineering.md, "One job, one home"). Heading tree/preamble ride the
  one `collect_heads` scan; `slugify` is the single slug home; no
  unsanctioned vocabulary. All four standing staleness debts re-verified
  on disk (stamps updated), all still riding.
- Queue: LAYOUT-PROSE-IMPORT (open); LAYOUT-READER-TEST-DEDUP (open,
  disjoint files); PACKAGING-CHANNELS (parked).

Plan continues: yes — all inputs current; quiet pass (job 5) closes.
