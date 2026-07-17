# Plan state

- Spec derived through: 7739b91
- Audited through: 7cf9ff0
- Residue swept through: 7cf9ff0
- This tick: DRAINED INBOX (4 lines). Two centercode field defects → open
  entries: DISCOVERY-WALK-SHARE (the ignore-honoring walk recomputes per kind
  + per nested host, ~40s at 16k-file scale — compute once per local-governs
  flavor and share; engineering.md 'One job, one home') and
  COVERAGE-SEGMENT-PRESENCE (the unmodeled-surface advisory classifies the
  static segment registry not the manifest's real keys — asserts absent
  segments, omits present residue; classify present keys; representation.md
  'Reach'). Decision 0039 (ruled 7739b91; the inbox demand note pointed here)
  derived: KNOWN-MARKETPLACE-KIND (the fourth registration member; blockedBy
  VERIFIER-TYPED — its new `Registration` variant ripples to kind.rs+graph.rs,
  and VERIFIER-TYPED holds graph.rs open) + KNOWN-MARKETPLACE-EDGE
  (plugin→marketplace, blockedBy the kind); the KnownSurface segment (0039c)
  folded into COVERAGE-SEGMENT-PRESENCE 'beside the coverage fix'. 0039
  Consequences all routed (builtins.md=the spec change; kind/edge/segment=the
  three entries; demand-note=drained) → spec cursor to 7739b91. Lazy-grounds
  demand (driver withdrawn) → open fork (lazy-grounds). Both defects
  re-verified live on disk (discoverable_paths recomputes at import.rs:182/311;
  segment_coverage classifies KNOWN_SURFACES not present keys at
  coverage_note.rs:410). Line 1's falsified CLAUDE.md premise already
  discharged by eb58934 → routed the code half only.
- Queue: 9 entries — 3 pickable (open): DISCOVERY-WALK-SHARE, VERIFIER-TYPED,
  COVERAGE-SEGMENT-PRESENCE (pairwise disjoint: import.rs / the verifier set /
  coverage_note.rs). 4 blockedBy (KNOWN-MARKETPLACE-KIND→VERIFIER-TYPED,
  KNOWN-MARKETPLACE-EDGE→its kind, TELEMETRY-FIELD-STRAND→TAP-VERB [shipped
  152526c], TELEMETRY-HOOK-PROJECTION→VERIFIER-TYPED); 2 parked
  (IMPORT-HOP-CAP-CITE, PACKAGING).

Plan continues: yes — post-ship reconciliation of 7cf9ff0..HEAD owed (TAP-VERB
+ SETTINGS-LOCAL shipped at 152526c; the tap/settings-local builds + eb58934
need audit/sweep). Reconciliation must unblock TELEMETRY-FIELD-STRAND but
re-gate it blockedBy VERIFIER-TYPED if that is still open — they share
read.rs/main.rs/read_verbs.rs (disjoint regions, but never both open).
