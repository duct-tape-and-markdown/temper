# Plan state

- Spec derived through: 4adb1fb
- Audited through: f713a08
- Residue swept through: f713a08
- Posture swept through: 4e9d87a — unadvanced this tick (job 3 ran, not
  job 4). Forward window `git log 4e9d87a..HEAD -- src/ sdk/src/ tests/`
  is now non-empty: the build ship touched src/address.rs, src/tap.rs
  (foundation), src/kind.rs (model), src/frontmatter.rs (formats) — so
  foundation, next in rotation, is live again next tick.
- This tick: POST-SHIP RECONCILIATION, window 1f6afe5..f713a08. Audit:
  verified on disk (not just the log) that all four shipped prunes
  landed as described — tap.rs's LOG_FILENAME private (34f8ab1),
  address.rs's FieldPath::spelling private (78048d7), kind.rs's
  member_document removed (8c1d03d), frontmatter.rs's companions field
  + scan_companions removed (c1341fc); `cargo test` and `cargo clippy
  --all-targets -- -D warnings` both green; metrics.jsonl glanced, the
  four build ticks sit in the normal smart-zone (14-23 turns, 2-9k
  output tokens). Sweep: no retirement named in this window beyond the
  four prunes themselves; no new residue found. Stale-gate re-test:
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE's blockedBy named
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE, which shipped this window —
  re-verified every cited file on disk (kind.rs's KindError shifted
  1480→1470, matching the ship's -10 net line change; builtin_kind.rs
  508/519 and bundle.rs/import.rs/install.rs/main.rs and every cited
  test file unmoved), confirmed no currently-open entry shares a file
  with it, and opened it.
- Queue: 24 pending, 5 pickable OPEN (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH, DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION — pairwise disjoint on files: kind.rs+
  builtin_kind.rs+bundle.rs+import.rs+install.rs+main.rs+tests/*,
  json_manifest.rs+toml_document.rs, engine.rs, dial.rs, drift.rs), 15
  chained blockedBy (unchanged links, all still resolve to live tags), 4
  parked on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE, NORMALIZE-PATH-SUBSYSTEM-
  PLACEMENT — reasons unchanged, untouched this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — posture-sweep re-armed: the ship touching
address.rs/tap.rs (foundation), kind.rs (model), frontmatter.rs
(formats) since the last sweep at 4e9d87a makes foundation, next in
rotation, live for the following tick.
