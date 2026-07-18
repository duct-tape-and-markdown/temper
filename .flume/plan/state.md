# Plan state

- Spec derived through: 7739b91
- Audited through: d1af9a5
- Residue swept through: d1af9a5
- This tick: RECONCILED 68fae5c..HEAD (post-ship). Window shipped three build
  entries: DISCOVERY-WALK-SHARE (16c1716), VERIFIER-TYPED (2fc2291),
  COVERAGE-SEGMENT-PRESENCE (536dd48) — all already dropped from pending by
  build. AUDIT (verified live on disk): the verifier is now a species-tagged
  union (`Verifier.Script`|`Verifier.Telemetry`, generated Verifier.ts;
  `requirement.verifier` on RequirementRow); no `verifiedBy` symbol survives;
  consumers hold exhaustive 2-variant matches (roster.rs 146/155/168, drift.rs
  3961/3965 — no `_` arm). DISCOVERY-WALK-SHARE consolidated to one
  `discoverable_paths` walk (import.rs:485, local_governs flavor);
  COVERAGE-SEGMENT-PRESENCE to one present-keys classifier (coverage_note.rs).
  Stale-gate re-test: VERIFIER-TYPED shipped, so its three dependents —
  KNOWN-MARKETPLACE-KIND, TELEMETRY-FIELD-STRAND, TELEMETRY-HOOK-PROJECTION —
  re-gated blockedBy->OPEN (anchors confirmed present; pairwise file-disjoint,
  safe to fan out). KNOWN-MARKETPLACE-EDGE stays blockedBy KNOWN-MARKETPLACE-KIND.
  Both parks re-tested at HEAD and hold: MAX_IMPORT_HOPS still 5 + 2026-07-02
  cite unmoved (graph.rs 55-59); no v0.1 tag, crate 0.1.0, `.github/` empty in
  window. SWEEP: no residue — the verifier retype leaves no partial enumeration
  or duplicate surface; requirement_roster tests green. The roster.rs cite
  rider (`10-contracts.md`, ride-only class) discharged at 2fc2291 as predicted
  — open-questions record trimmed to zero live orphans. Both cursors -> HEAD.
- Queue: 6 entries — 3 pickable OPEN (KNOWN-MARKETPLACE-KIND,
  TELEMETRY-FIELD-STRAND, TELEMETRY-HOOK-PROJECTION — pairwise file-disjoint);
  1 blockedBy (KNOWN-MARKETPLACE-EDGE->KIND); 2 parked (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER).

Plan continues: no — window reconciled through HEAD, inbox + spec-delta empty;
3 pickable open entries (KNOWN-MARKETPLACE-KIND, TELEMETRY-FIELD-STRAND,
TELEMETRY-HOOK-PROJECTION), build takes over.
