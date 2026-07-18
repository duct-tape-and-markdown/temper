# Plan state

- Spec derived through: 4adb1fb
- Audited through: f713a08
- Residue swept through: f713a08
- Posture swept through: 4e9d87a — foundation done this tick (found
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE, opened). Forward window
  `git log 4e9d87a..HEAD -- src/ sdk/src/ tests/` still carries
  src/kind.rs (model) and src/frontmatter.rs (formats) untouched by this
  tick's sweep — model, next in rotation, is live again next tick.
- This tick: POSTURE SWEEP, foundation subsystem (check.rs, tap.rs,
  hash.rs, address.rs, json_splice.rs, per architecture.md's codemap;
  extract.rs is foundation too but its upward-dependency debt is already
  queued via EXTRACT-FOUNDATION-BOUNDARY-RESTORE). Read every foundation
  file end to end. Found: check.rs's `ANNOUNCEMENT_HEADING` (168) is a
  `pub const` read only by `Announcement::render` (203, same file) and
  its own inline tests (258, 272) — grep-verified zero hits elsewhere in
  src/, tests/, sdk/ — the same shape as the shipped
  TAP-LOG-FILENAME/ADDRESS-FIELDPATH-SPELLING/DIAL-IS-EMPTY/GRAPH-WORLD
  zero-consumer prunes. Filed CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE
  (open — check.rs untouched by any other queued entry). Checked and
  found clean: tap.rs (TapError/log_path/record_from_payload/append/
  read_log/LogReadout each have a live external caller — LOG_FILENAME's
  own prior narrowing already settled), hash.rs (sha256_hex's sole
  consumer set unchanged; tests/emit.rs's independent Sha256 use is
  deliberate integration-test isolation, not residue, since tests/ can't
  reach `pub(crate) hash::sha256_hex`), address.rs (FieldPath's five pub
  methods — parse/is_bare_name/head_name/split_leaf/locate — each called
  externally; `spelling()`'s #[allow(dead_code)] is the already-settled
  terminal state of ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE, not
  new residue), json_splice.rs (every pub(crate) surface consumed by
  install.rs/json_manifest.rs). extract.rs's other pub items
  (host_address, MemberAddress, addressed_leaves, locate_presence,
  embedded_leaves, ValueType::from_name, FeatureValue::scalar) each have
  a live consumer — no finding beyond the already-queued entry. No
  non-exhaustive match over a shared enum found in any of the five files.
- Queue: 25 pending, 6 pickable OPEN (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH, DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE, DRIFT-LOCK-ROW-WALK-
  CONSOLIDATION — pairwise disjoint on files: kind.rs+builtin_kind.rs+
  bundle.rs+import.rs+install.rs+main.rs+tests/*, json_manifest.rs+
  toml_document.rs, engine.rs, dial.rs, check.rs, drift.rs), 15 chained
  blockedBy (unchanged links, all still resolve to live tags), 4 parked
  on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE, NORMALIZE-PATH-SUBSYSTEM-
  PLACEMENT — reasons unchanged, untouched this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — model, next in rotation, is live next tick (the
forward window past 4e9d87a still carries src/kind.rs, untouched by this
tick's foundation sweep).
