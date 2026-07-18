# Plan state

- Spec derived through: 4adb1fb
- Audited through: f713a08
- Residue swept through: f713a08
- Posture swept through: 4e9d87a — model done this tick (found
  KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH and
  COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE, both opened as blockedBy
  chains behind the one currently-open entry each shares a file with).
  Forward window `git log 4e9d87a..HEAD -- src/ sdk/src/ tests/` still
  carries src/frontmatter.rs (formats) untouched by this tick's sweep —
  formats, next in rotation, is live again next tick.
- This tick: POSTURE SWEEP, model subsystem (kind.rs, contract.rs,
  compose.rs, schema.rs, roster.rs, per architecture.md's codemap: "what
  a harness IS"). Read every model file end to end (5257 lines total).
  Found: (1) `CustomKind::declared_fields` (kind.rs 945-954) matches
  `Primitive` with `Field { key } => Some(...)` then `_ => None` for the
  remaining six variants, unlike two sibling matches over the same enum
  in this file — `Primitive::key` (1392-1402) and `Primitive::apply`
  (1407-1427) — both exhaustive by name. Filed
  KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH, blockedBy
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE (the one currently-open entry
  sharing kind.rs; KIND-ZERO-CONSUMER-EXPORTS-PRUNE also touches kind.rs
  but is itself blockedBy, not open, no present conflict). (2) dial.rs's
  private `severity_from_label` (144-150) is byte-for-byte identical to
  compose.rs's `pub fn severity_from_label` (266-272) — grep-verified
  the only two definitions in the tree. compose.rs's own doc comment
  (261-265) claims its `pub` visibility is earned by a `main`-binary
  consumer that, verified on disk, does not exist (main.rs calls
  `compose::clause_from_row` only, never `severity_from_label` directly)
  — the doc's predicted "never a second copy" failure landed anyway, in
  dial.rs, under cover of the wrong fn's `pub`. Filed
  COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE (dial.rs calls
  compose::severity_from_label; compose's copy narrows pub→pub(crate),
  doc corrected; clause_from_row's cross-referencing doc comment
  corrected too), blockedBy DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE, the one
  currently-open entry sharing dial.rs. Checked and found clean beyond
  the four already-queued model-file entries (BUILTIN-KIND-DEFINITION-
  RESULT-COLLAPSE's KindError; KIND-ZERO-CONSUMER-EXPORTS-PRUNE's
  Commitment::label/Content::label/CustomKind::qualified_name;
  CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH's declared_keys;
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP's RequireSections gap): schema.rs
  (its own match over Predicate in `emit` is fully exhaustive), roster.rs
  (its match over Verifier is fully exhaustive, no other non-exhaustive
  match over a shared enum), compose.rs and contract.rs otherwise clean
  (no other zero-consumer export, no other duplicate job).
- Queue: 27 pending, 6 pickable OPEN (unchanged set:
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH, DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE, DRIFT-LOCK-ROW-WALK-
  CONSOLIDATION — pairwise disjoint on files, unchanged), 17 chained
  blockedBy (15 prior + the 2 filed this tick, both verified disjoint
  from every other currently-open entry's files), 4 parked on human
  action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE, NORMALIZE-PATH-SUBSYSTEM-
  PLACEMENT — reasons unchanged, untouched this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — formats, next in rotation, is live next tick (the
forward window past 4e9d87a still carries src/frontmatter.rs, untouched
by this tick's model sweep).
