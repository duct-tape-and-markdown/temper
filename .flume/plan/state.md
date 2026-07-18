# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: c1b0f51 — advanced from 4e46eac.
- Residue swept through: c1b0f51 — advanced from 4e46eac.
- Posture swept through: formats next (mid-rotation) — a fresh cycle
  opened this tick. foundation bulk-skipped (quiet: `git log
  fe3ff3f..c1b0f51 -- src/check.rs src/extract.rs src/hash.rs
  src/address.rs src/tap.rs src/json_splice.rs` empty, git-log test only,
  no file read); model read and swept (touched: kind.rs by dcdfbff's
  declared_fields exhaustive-match ship) — quiet-on-clean.
- This tick: POSTURE SWEEP — model subsystem (`src/kind.rs`,
  `src/contract.rs`, `src/compose.rs`, `src/schema.rs`, `src/roster.rs`),
  touched by dcdfbff (declared_fields's exhaustive-match rewrite,
  ship-audited last tick). All five files read whole against every
  `specs/process/engineering.md` section plus the sweep's cohesion/dead-
  plumbing lenses. compose.rs/schema.rs/roster.rs clean — no exhaustive-
  match fallthrough over a Rust enum (schema.rs's own `Predicate` match at
  85-165 already names every variant), no zero-consumer export (`rg`-
  confirmed a consumer outside its own module for every checked `pub` item:
  `glob_leaf`, `Charset::allows`, `Shape::demand`, `ranges_over_selection`,
  `edge_field_slots`, `surface_subdir`, `local_locus_fault`,
  `overlay_content`, `nested_file`, `spans_whole_manifest`, `key_field`,
  `collection_key`, `glob_compile_count`, `with_joined_clauses`), no
  duplicate matcher/normalizer. kind.rs/contract.rs: every already-queued
  finding re-verified still true (KIND-ZERO-CONSUMER-EXPORTS-PRUNE's
  `Commitment::label`/`Content::label`/`CustomKind::qualified_name` still
  grep-confirmed zero-consumer; CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH's
  `declared_keys` `_ => None` at line 741 still stands beside `target`/
  `documented_field`'s exhaustive style; CONTRACT-REQUIRE-SECTIONS-
  ROUNDTRIP's `requireSections()` still takes no argument in
  sdk/src/contract.ts) — each entry's cited line range is within ±1 of
  the current file (kind.rs's post-945 insertions from dcdfbff land after
  every cited site here, so nothing shifted materially; the ±1 is the
  same rounding already present across this corpus's citations, not real
  drift, so no correction filed). The `_ => None`/`_ => {}` arms
  elsewhere in kind.rs/contract.rs/compose.rs (format_from_label,
  unit_shape_from_label, commitment_from_label,
  collection_key_path_from_label, registration_from_label,
  severity_from_label, `Shape`/`ExtentUnit::from_name`,
  `predicate_from_row`, `charset_from_row`) are all closed-vocabulary
  *string label* parsers, not consumers of a shared Rust enum — outside
  this section's exhaustive-match bar by the section's own scoping.
  Verdict: quiet-on-clean. Rotation advances to formats — also touched in
  the same window (8f96918's json_manifest.rs parse consolidation) — so
  next tick sweeps it (one touched subsystem read per tick; formats was
  not read this tick).
- Queue: 28 pending — unchanged (no entry filed, dropped, or edited this
  tick; posture sweep found nothing new). 1 pickable OPEN
  (PLACEMENT-MODULE-EXTRACTION), 25 chained blockedBy (all resolving to
  live tags), 2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched. Refactor
  captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep is still mid-rotation (formats
next, already known touched by 8f96918) and no commit past c1b0f51 has
landed to re-trigger reconciliation, so the sweep is the next live input.
