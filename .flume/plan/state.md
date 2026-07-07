# Plan state

- Spec derived through: 5945405
- Audited through: 2b82ebd
- Residue swept through: 2b82ebd
- This tick: Spec delta f4189c3, second slice — 0010 line-endings-are-layout →
  EMIT-LF-NORMALIZE (blockedBy EMIT-REAP-ORPHANS: shares src/drift.rs emit +
  tests/emit.rs). Emit normalizes every projection to LF (CRLF/lone CR → LF)
  before the double-emit compare, hash, and write, so byte-reproducibility holds
  cross-platform. 0008 (settings format-preserving) / 0009 (module-relative paths)
  / 0011 (documented-capability vocabulary) still unrouted — f4189c3 NOT fully
  routed, spec cursor stays at 5945405. Audited/Residue cursors copied forward
  verbatim (not serviced this tick).
- Queue: 4 — EMIT-REAP-ORPHANS open (drift.rs), EMIT-LF-NORMALIZE blockedBy
  EMIT-REAP-ORPHANS (same drift.rs+emit.rs), GUARD-OWNPATH blockedBy
  PATH-SEP-NORMALIZE (blocker SHIPPED 2efd00b — ship audit unblocks it),
  PACKAGING-CHANNELS parked (human release creds). Disjoint chains: drift.rs /
  install.rs+kind.rs / package.json.

Plan continues: yes — spec delta still live (f4189c3 rulings 0008/0009/0011
unrouted); ship audit also trailing (PATH-SEP-NORMALIZE shipped past
Audited-through 2b82ebd).
