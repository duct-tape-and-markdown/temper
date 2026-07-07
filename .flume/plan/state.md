# Plan state

- Spec derived through: 5945405
- Audited through: c7595d3
- Residue swept through: 8aeb64e
- This tick: Drained inbox — routed field report 3 (T9-T13) against HEAD
  (f8b3d7f), *after* REQUIREMENT-GATE which the report (18dca38) predates.
  Verified each on disk: T9 -> new COVERAGE-KIND-AWARE (only coverage_note's
  built-in-only kind set survives; the gate now runs+counts custom floors and
  install places their notes via kind-generic emit_owned_targets). T10 -> new
  MENTION-EDGE-LANDS (`n` resolution-checked at emit.ts but no declaration-row
  family, so it reaches no lock/graph edge). T12 -> new DISCOVERY-SKIPS-SURFACE
  (discoverable_paths excludes only .git; .temper/ is committed and walked).
  T11 reconciled — harness_diagnostics now delegates to gate(), so session-start
  reads the same lock joins; no entry. T13 -> registered `(genre-fence-format)`
  fork (write face); its concrete claims are stale (graph.rs nested-empty is
  test code, from_kind_fact_row has a production caller now, read fold exists in
  kind.rs). Cursors unmoved — inbox job, not audit.
- Queue: 9 — 3 open/disjoint (HELP-TEXT-RECUT main.rs, COVERAGE-KIND-AWARE
  coverage_note.rs, DISCOVERY-SKIPS-SURFACE import.rs), 5 blocked (install.rs
  chain SCAFFOLD-OUTPUT-VALID→PATH-SEP-NORMALIZE→GUARD-OWNPATH; EXPLAIN-RESOLVER
  on the shipped REQUIREMENT-GATE; MENTION-EDGE-LANDS on HELP-TEXT-RECUT for
  shared main.rs), PACKAGING-CHANNELS parked.

Plan continues: yes — ship audit is live (c7595d3..HEAD carries the
REQUIREMENT-GATE + WIN/BUNDLE src+sdk+tests commits): it advances the Audited
cursor and unblocks SCAFFOLD-OUTPUT-VALID + EXPLAIN-RESOLVER, whose blockers
(WIN-INSTALL-SPAWN, REQUIREMENT-GATE) shipped this window.
