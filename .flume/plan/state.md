# Plan state

- Spec derived through: 684dfec
- Audited through: 0349821
- Residue swept through: 0349821
- This tick: INBOX (refactor drain). Drained the one live refactor capture,
  `.flume/refactor/build-extent-embedded-span.md` (observed fe8790e,
  re-verified on disk at HEAD): EXTENT-PREDICATE (4bdb796) shipped the
  file-member half of the extent predicate but not the embedded case 0035
  exists for — `src/main.rs:2060` hardcodes an embedded member's
  `rendered_lines`/`rendered_chars` to 0, `drift::NestedMemberRow`
  (`src/drift.rs:3019`) carries no captured span, and `src/engine.rs:256`
  fences `Predicate::Extent` bodyless for embedded kinds. Filed
  EMBEDDED-EXTENT-CAPTURE (per engineering.md 'A shared concept is one
  type', gate blockedBy EMBEDDED-EDGE-SOURCE-SEAM — both edit the same
  `embedded_member_features` function): capture the rendered span at emit
  onto NestedMemberRow (the `placed_edges` optional-column precedent), lift
  it into the embedded member's Features, drop Extent from the bodyless
  fence. Widens the shared NestedMemberRow row — consumers named in files[].
  Capture DELETED (git rm); claim held at HEAD.
- Queue: 13 entries — 5 pickable (gate:open): MANIFEST-SEGMENT-REAP,
  EMBEDDED-EDGE-SOURCE-SEAM, TEMPLATE-DISCOVERY-CARVE, STAR-SEGMENT-LOCUS,
  GAUNTLET-CORPUS. Then EMBEDDED-EXTENT-CAPTURE (blockedBy EMBEDDED-EDGE),
  TAP-VERB-EVENT-RECORD (blockedBy EMBEDDED-EDGE), TELEMETRY-FIELD-STRAND
  (blockedBy TAP-VERB); SETTINGS-LOCAL-KIND + VERIFIER-TYPED (blockedBy
  EXTENT — shipped 4bdb796/b576844, unblock at next audit);
  TELEMETRY-HOOK-PROJECTION (blockedBy VERIFIER-TYPED); + 2 parked
  (IMPORT-HOP-CAP-CITE, PACKAGING).

Plan continues: yes — inbox drained this tick; next live input is post-ship
reconciliation of b85df4a..HEAD (EXTENT-PREDICATE shipped 4bdb796/b576844 —
audit unblocks the two blockedBy-EXTENT entries SETTINGS-LOCAL-KIND +
VERIFIER-TYPED, then sweep the same window code against corpus).
