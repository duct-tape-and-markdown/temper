# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: 73c76ca — unchanged; no commit past it has landed.
- Residue swept through: 73c76ca — unchanged; no commit past it has landed.
- Posture swept through: fe3ff3f — verbs ticked this tick and closes the
  rotation pass (foundation done, model done, formats done, pipeline
  quiet, judges bulk-skipped, provider quiet-on-clean, verbs this tick —
  1 new finding). All seven subsystems from architecture.md's codemap now
  covered. A fresh cycle (foundation → model → formats → pipeline →
  judges → provider → verbs) opens next time the sweep re-arms.
- This tick: POSTURE SWEEP — verbs subsystem (`src/main.rs`,
  `src/install.rs`, `src/bundle.rs`, `src/lib.rs`, `src/test_support.rs`),
  triggered by 0282dc7 touching main.rs/bundle.rs. All five files read
  whole against every `specs/process/engineering.md` section plus the
  sweep's cohesion/dead-plumbing lenses. lib.rs/test_support.rs clean.
  bundle.rs clean (its `SESSION_START_COMMAND` duplicate against
  install.rs's own is already tracked, BUNDLE-INSTALL-SESSION-START-SHAPE-
  CONSOLIDATE, re-verified still true). install.rs: every already-queued
  finding re-verified still true at its cited lines (matches_projection/
  manifest_write_findings' duplicate normalizer — INSTALL-PROJECTION-
  MATCH-CONSOLIDATE; GUARD_MANIFEST_MESSAGE zero-consumer — INSTALL-GUARD-
  MANIFEST-MESSAGE-PRUNE; InstallEntry.placement's wildcard fallback —
  INSTALL-PLACEMENT-KIND-ENUM); nothing new. main.rs: one new finding —
  MAIN-READ-FILE-UNIT-FORMAT-EXHAUSTIVE-MATCH. `read_file_unit`'s
  `Content::File | Content::Fields` arms name `Format::JsonDocument` and
  `Format::TomlDocument` explicitly then fall through a `_` wildcard
  (1439) covering `YamlFrontmatter`/`None` — non-exhaustive over the
  `Format` enum, unlike this same file's `local_document_rows` four
  functions below (1776-1787, already exhaustive over the identical
  enum) and drift.rs's `project_bytes` (2156-2170), engineering.md's own
  cited precedent for this enum. Filed `blockedBy`
  BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE, the queue-front open entry
  already sharing main.rs, for disjointness.
- Queue: 34 pending (+1) — MAIN-READ-FILE-UNIT-FORMAT-EXHAUSTIVE-MATCH
  filed. 3 pickable OPEN (DRIFT-INCLUDE-SOURCE-PATH-CWD-LEAK,
  BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE,
  JSON-MANIFEST-TOP-LEVEL-OBJECT-PARSE-CONSOLIDATE — pairwise
  file-disjoint), 29 chained blockedBy (all resolving to live tags), 2
  parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched. Refactor
  captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: no — the posture rotation closes this tick (all seven
subsystems covered this cycle), no spec delta past 2d66fc9, and no
post-ship reconciliation window past 73c76ca. Next wake re-arms the
rotation once a forward window touches a subsystem, or a spec/post-ship
input goes live.
