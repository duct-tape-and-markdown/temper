# Plan state

- Spec derived through: 684dfec
- Audited through: 82dcb3e
- Residue swept through: 82dcb3e
- This tick: POST-SHIP RECONCILIATION of 0349821..HEAD. One code commit in
  window: 4bdb796 (EXTENT-PREDICATE — 9a18907 was specs+.temper harness only,
  the rest plan/chore/specs). AUDIT: verified on disk EXTENT shipped whole —
  `extent(unit,bound)` live in `src/engine.rs`, `rendered_lines`/`rendered_chars`
  intrinsics, `max_lines` retired (surviving mentions are the retirement
  tests + explanatory comments only). Both `blockedBy: EXTENT-PREDICATE`
  entries pointed at an absent shipped tag → UNBLOCKED to `open`:
  SETTINGS-LOCAL-KIND (premise held: no `settings-local` kind in src/sdk) and
  VERIFIER-TYPED (premise held: `verified_by: Option<String>` still untyped at
  compose.rs:100 / contract.ts:300). Refreshed both entries' stale "serialized
  behind EXTENT" language to name the current region-disjoint coexistence
  partners on drift.rs/main.rs/graph.rs/declarations.ts. SWEEP: EXTENT
  introduced no residue — `body_lines`/`LineCount` kept as intentional orphans
  (0035 names no LineCount retirement, per the commit body), refactor already
  drained (82dcb3e → EMBEDDED-EXTENT-CAPTURE). Ride-only: VERIFIER-TYPED now
  opens src/roster.rs, so it carries the orphan `10-contracts.md` cite-cut
  (~475); open-questions record re-routed onto it.
- Queue: 13 entries — 7 pickable (gate:open): MANIFEST-SEGMENT-REAP,
  EMBEDDED-EDGE-SOURCE-SEAM, TEMPLATE-DISCOVERY-CARVE, STAR-SEGMENT-LOCUS,
  GAUNTLET-CORPUS, + SETTINGS-LOCAL-KIND, VERIFIER-TYPED (both unblocked). 4
  blockedBy (EMBEDDED-EXTENT-CAPTURE/TAP-VERB → EMBEDDED-EDGE;
  TELEMETRY-FIELD-STRAND → TAP-VERB; TELEMETRY-HOOK-PROJECTION → VERIFIER-TYPED);
  2 parked (IMPORT-HOP-CAP-CITE, PACKAGING). Open set region-disjoint.

Plan continues: no — inbox empty, no spec delta past 684dfec, EXTENT window
reconciled and both cursors at HEAD; no live input remains. 7 pickable
entries — build takes over.
