# Plan state

- Spec derived through: 7739b91
- Audited through: 68fae5c
- Residue swept through: 68fae5c
- This tick: RECONCILED 7cf9ff0..HEAD (post-ship). Window's only src/tests/sdk
  touches: 4119bed (tap) + 96f6a10 (settings-local) — TAP-VERB-EVENT-RECORD +
  SETTINGS-LOCAL-KIND, both already dropped from pending at 68fae5c. eb58934 is
  pure harness (.temper/, CLAUDE.md) — outside audit scope. AUDIT: verified live
  on disk — src/tap.rs ships `read_log` returning `LogReadout{records,
  older_version}` (the version-tolerant reader, 4119bed), `append`,
  `pub(crate) log_path`, `TAP_RECORD_VERSION`; tests/settings_local_kind.rs
  ships. Stale-gate re-test: TELEMETRY-FIELD-STRAND was blockedBy
  TAP-VERB-EVENT-RECORD (now shipped) — re-gated blockedBy VERIFIER-TYPED: the
  strand shares read.rs (field strand vs. ~1489 fixture), main.rs (explain
  assembly vs. requirement_from_row), read_verbs.rs (explain helper vs. ~54
  fixture) with VERIFIER-TYPED — disjoint regions but file-shared, never both
  open, so serialized behind it. SWEEP: no residue — the bespoke tap JSONL log
  is corpus-sanctioned (pipeline.md 'Telemetry': machine-written,
  bespoke-parsed, internal, versioned in lockstep), settings-local follows the
  shipped-kind pattern; no retirement named, no second implementation of one
  job, cargo check green. Both cursors -> HEAD.
- Queue: 9 entries — 3 pickable (open): DISCOVERY-WALK-SHARE (import.rs),
  VERIFIER-TYPED (the verifier set), COVERAGE-SEGMENT-PRESENCE (coverage_note.rs)
  — pairwise file-disjoint. 4 blockedBy (KNOWN-MARKETPLACE-KIND→VERIFIER-TYPED,
  KNOWN-MARKETPLACE-EDGE→its kind, TELEMETRY-FIELD-STRAND→VERIFIER-TYPED [was
  TAP-VERB, shipped 4119bed], TELEMETRY-HOOK-PROJECTION→VERIFIER-TYPED); 2
  parked (IMPORT-HOP-CAP-CITE, PACKAGING).

Plan continues: no — window reconciled through HEAD, inbox + spec-delta empty;
3 pickable open entries (DISCOVERY-WALK-SHARE, VERIFIER-TYPED,
COVERAGE-SEGMENT-PRESENCE), build takes over.
