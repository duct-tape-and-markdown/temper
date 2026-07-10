# Plan state

- Spec derived through: a0fccaf
- Audited through: d209e0a
- Residue swept through: f6a4af5
- This tick: Residue sweep (job 4). One code commit past the cursor,
  d4207b8 (LAYOUT-READER-TEST-DEDUP): pure subtraction — three reader
  tests + twin intent_layout()/INTENT_DOC fixtures deleted from
  src/kind.rs, the uniquely-held unadmitted-heading refusal moved into
  tests/layout_kind.rs. No twin remains (fixtures grep to layout_kind.rs
  only); the kept unit suites (kind.rs overlay/lift, extract.rs
  pub(crate) internals) are distinct jobs, not mirrors. Nothing filed.
  All five riding debts re-verified on disk at f6a4af5, none of their
  files opened; stamps updated in open-questions.
- Queue: LAYOUT-PROSE-IMPORT (open); LAYOUT-EDGE-SLOT (blockedBy
  LAYOUT-PROSE-IMPORT); PACKAGING-CHANNELS (parked).

Plan continues: yes — quiet pass remains (closing check, then hand off
to build).
