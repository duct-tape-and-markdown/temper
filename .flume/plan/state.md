# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: 785abd7 — unchanged. `git log 785abd7..HEAD -- src/
  tests/ sdk/` is empty (the two commits past it, 5d60a8a and 269a4a1,
  are plan-only) — nothing to reconcile.
- Residue swept through: 785abd7 — unchanged, same empty window.
- Posture swept through: model next (mid-rotation) — foundation swept
  quiet last tick (untouched since 3871eba), rotation advanced alone to
  `model`, which this tick's note flagged NOT clean-skippable
  (b4a2467 touched compose.rs). Read whole this tick; see below.
- This tick: POSTURE SWEEP — job 4, model subsystem (`kind`/`contract`/
  `compose`/`schema`/`roster`). Read all five files in full against
  every section of `specs/process/engineering.md` plus the sweep's
  cohesion/dead-plumbing lenses. b4a2467's own change
  (COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE) reads clean in place:
  `compose::severity_from_label` (compose.rs:265) carries exactly two
  callers (its own `clause_from_row` and `dial.rs`), doc comment
  accurate, no residue. Cross-checked every `pub`/`pub(crate)` item's
  consumer count by grep across all five files for zero-consumer
  surface (`engineering.md`, "An export earns its consumer"): no new
  gap beyond the three already queued in KIND-ZERO-CONSUMER-EXPORTS-PRUNE
  (`Commitment::label`, `Content::label`, `CustomKind::qualified_name`)
  and BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE (`KindError`) —
  `Charset::allows`, `Shape::admits`/`match_holds`/`demand`/`name`,
  `ExtentUnit::name`, `with_joined_clauses`, `spans_whole_manifest`,
  `key_field`, `identity_edge`, `glob_leaf`, `kind_narrowing_clause` all
  read live external callers (engine.rs, graph.rs, json_manifest.rs,
  coverage_note.rs, drift.rs, main.rs, roster.rs, tests). Exhaustive-match
  discipline ("A shared concept is one type") holds everywhere but the
  two already-queued gaps (kind.rs's `declared_fields`, contract.rs's
  `declared_keys`) — contract.rs's `target`/`documented_field`/
  `Predicate::key` and schema.rs's `emit` dispatch are all pipe-grouped
  exhaustive, no `_` arm. No cost-hoist or vacuity-pin gap: none of the
  five files performs I/O or ranges a judge loop of its own. No new
  pending entry — quiet-on-clean.
  Checked ahead for next tick: formats (`frontmatter`/`document`/
  `json_manifest`/`toml_document`) is NOT clean-skippable — `git log
  7a5f86c..HEAD -- src/frontmatter.rs src/document.rs
  src/json_manifest.rs src/toml_document.rs` shows d577bdf
  (JSON-MANIFEST-READ-DECODE-CONSOLIDATE) and d65af7d
  (TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE) in the window. Next tick
  reads formats whole rather than skipping.
- Queue: 32 pending, unchanged — 4 pickable OPEN
  (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  FORMAT-READ-UTF8-DECODE-CONSOLIDATE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION — pairwise file-disjoint,
  re-verified this tick), 26 chained blockedBy (unchanged links, all
  still resolve to live tags), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — reasons
  unchanged, untouched this window). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. Refactor
  captures: 0 live. Friction: 1 live (build-worktree-commits-land-on-
  main-branch.md, unchanged). Inbox empty.
  Disjointness re-checked: no two OPEN entries share a file.

Plan continues: yes — posture sweep resumes at `formats`
(`frontmatter`/`document`/`json_manifest`/`toml_document`), not
clean-skippable per the note above — the next tick reads it whole.
