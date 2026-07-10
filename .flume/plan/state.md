# Plan state

- Spec derived through: a0fccaf
- Audited through: eab6979
- Residue swept through: e5a7077
- This tick: Residue sweep over c8b78c5 + 5d0ea5b. 5d0ea5b is pure
  consolidation (no residue; install.rs:189 checked — a different job, not a
  scaffold copy). c8b78c5 clean in itself but its named scope cut and the
  premise it retired both file: BUILTIN-CONTRACT-ARRAY-SURGERY (effective's
  severity-flip layer vs builtins.md "no layering rules") and
  LOCK-ROW-REJECT-LOUD (from_table degrade-to-absent vs invariant 6), both
  serialized onto the main.rs chain per the disjoint rule. All six riding
  debts re-verified on disk; prose.ts found binary-detected (literal \x00/\x01
  sentinels — grep silently skips it), noted on its record.
- Queue: LAYOUT-RELATIONSHIP-EDGES (open); EMBEDDED-EDGE-TARGETS (blockedBy
  LAYOUT-RELATIONSHIP-EDGES); BUILTIN-CONTRACT-ARRAY-SURGERY (blockedBy
  EMBEDDED-EDGE-TARGETS); LOCK-ROW-REJECT-LOUD (blockedBy
  BUILTIN-CONTRACT-ARRAY-SURGERY); PACKAGING-CHANNELS (parked).

Plan continues: yes — all inputs current; quiet closing pass next.
