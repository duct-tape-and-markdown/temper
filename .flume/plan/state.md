# Plan state

- Spec derived through: 684dfec
- Audited through: 1745634
- Residue swept through: 1745634
- This tick: POST-SHIP RECONCILIATION of 82dcb3e..HEAD. Four build commits
  shipped (chore(flume) 1745634): MANIFEST-SEGMENT-REAP (2a2d583),
  EMBEDDED-EDGE-SOURCE-SEAM (0d84244), TEMPLATE-DISCOVERY-CARVE (dcaf48c),
  GAUNTLET-CORPUS (87b3391). AUDIT (on disk, not the log): EMBEDDED-EDGE shipped
  its body-carried-edge attribution in graph.rs's `mention_reachable`
  (`embedded_hosts_by_source`, main.rs:1917) — NOT in `embedded_member_features`.
  Both `blockedBy: EMBEDDED-EDGE-SOURCE-SEAM` entries → UNBLOCKED to open,
  premises re-verified: EMBEDDED-EXTENT-CAPTURE (main.rs still hardcodes span 0
  at ~2083-84, drift NestedMemberRow lacks a rendered column, engine.rs:256
  still fences Extent bodyless) and TAP-VERB-EVENT-RECORD (no src/tap.rs, no
  `Tap` command). Corrected EMBEDDED-EXTENT's now-false "EMBEDDED-EDGE threads
  row.host into Features here" claim — it never touched that function. Refreshed
  stale coexistence language naming now-shipped MANIFEST/EMBEDDED-EDGE/TEMPLATE
  as in-flight partners across STAR/EMBEDDED-EXTENT/TAP/VERIFIER; line-number
  drift left to build's `scoped at` re-address (the mechanism for exactly that).
  SWEEP: the 4 ships introduced no residue — MANIFEST reused `Manifest::parse`
  (no 2nd parser), EMBEDDED-EDGE local to `mention_reachable` (no other reader
  moves), TEMPLATE-CARVE's consumers self-heal, GAUNTLET test-only. GAUNTLET-
  CORPUS explicitly deferred the STAR-SEGMENT-LOCUS gauntlet cell to STAR's
  entry — added it (tests/gauntlet.rs) per engineering.md 'The gauntlet corpus'.
  TEMPLATE-DISCOVERY-CARVE shipped before the gauntlet existed so its cell was
  never added — accepted debt (its own nested_member.rs test covers
  correctness; gauntlet integration coverage is the marginal miss, not a
  correctness gap). Parks re-tested on disk and hold: IMPORT-HOP-CAP-CITE
  (MAX_IMPORT_HOPS still 5 at graph.rs:59, no hop-semantics ruling), PACKAGING
  (no version tag, release.yml deferral verbatim, `git diff 82dcb3e..HEAD --
  .github/` empty).
- Queue: 9 entries — 5 pickable (open): STAR-SEGMENT-LOCUS, EMBEDDED-EXTENT-
  CAPTURE, TAP-VERB-EVENT-RECORD, SETTINGS-LOCAL-KIND, VERIFIER-TYPED. 2
  blockedBy (TELEMETRY-FIELD-STRAND → TAP-VERB; TELEMETRY-HOOK-PROJECTION →
  VERIFIER-TYPED); 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING). Open set shares
  drift.rs/main.rs/declarations.ts region-disjointly (distinct structs/fns,
  documented per file) — the accepted coexistence pattern; build's singleton
  serializes picks.

Plan continues: no — inbox empty, no spec delta past 684dfec, window reconciled
and both cursors at HEAD; no live input remains. 5 pickable entries — build
takes over.
