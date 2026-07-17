# Plan state

- Spec derived through: 684dfec
- Audited through: 7cf9ff0
- Residue swept through: 7cf9ff0
- This tick: POST-SHIP RECONCILIATION of 585066b..HEAD. One build commit
  shipped (chore(flume) 7cf9ff0): EMBEDDED-EXTENT-CAPTURE (build e8e7a3b) —
  captures a composed embedded member's rendered span at emit so `extent`
  budgets it. AUDIT (on disk): `NestedMemberRow` now carries
  `rendered_lines`/`rendered_chars: Option<usize>` beside `placed_edges`
  (drift.rs:43-49); `main.rs` lifts the captured span (2079-81, hardcoded
  zero gone); `engine.rs` dropped `extent` from the `bodyless` fence (248-256,
  Extent now judged with Indeterminate for an unrendered/layout-source span);
  the extent suite passes including
  `an_extent_clause_bound_to_an_embedded_kind_judges_the_captured_span`.
  Already dropped from pending by the ship (7cf9ff0, -91 lines). Refreshed the
  one stale coexistence note (VERIFIER-TYPED's drift.rs description) naming
  now-shipped EMBEDDED-EXTENT-CAPTURE as "open" -> shipped 7cf9ff0; regions
  stay struct-disjoint (RequirementRow vs NestedMemberRow), line drift left to
  build's `scoped at`. The three open entries re-verified and HOLD as
  unbuilt: TAP-VERB (no src/tap.rs, no `Tap` command), SETTINGS-LOCAL (no
  settings_local/settingsLocal symbol), VERIFIER-TYPED (verified_by/verifiedBy
  still live in compose.rs:100/drift.rs:3048/contract.ts:300). SWEEP:
  EMBEDDED-EXTENT introduced no residue — `renderedExtents` reuses
  `renderMemberBlock` (one home), `Features` extent fields widened as one
  `Option<usize>` type (shared concept, one type), no `_` arm, no TODO; the
  two documented deviations (reused `opt_usize`, migrated gate_fail_loud
  exemplars extent->name-matches-dir) both toward one-job-one-home. Parks
  re-tested on disk and hold: IMPORT-HOP-CAP-CITE (MAX_IMPORT_HOPS still 5 at
  graph.rs:59, no hop-semantics ruling; window's graph.rs edit was the
  Features fixture ripple), PACKAGING (crate 0.1.0, no version tag,
  release.yml:7-9 deferral verbatim).
- Queue: 7 entries — 3 pickable (open): TAP-VERB-EVENT-RECORD,
  SETTINGS-LOCAL-KIND, VERIFIER-TYPED. 2 blockedBy (TELEMETRY-FIELD-STRAND ->
  TAP-VERB; TELEMETRY-HOOK-PROJECTION -> VERIFIER-TYPED); 2 parked
  (IMPORT-HOP-CAP-CITE, PACKAGING). Open set shares drift.rs/main.rs/
  declarations.ts region-disjointly (distinct structs/fns, documented per
  file) — the accepted coexistence pattern; build's singleton serializes picks.

Plan continues: no — inbox empty, no spec delta past 684dfec, window
reconciled and both audit/sweep cursors at HEAD; no live input remains. 3
pickable entries — build takes over.
