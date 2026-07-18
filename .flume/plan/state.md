# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8 — unchanged; `git log d40a9f8..HEAD -- src/
  tests/ sdk/` is empty (both commits since, 5af93d9 and 3871eba, are
  plan-only).
- Residue swept through: d40a9f8 — unchanged, same empty window.
- Posture swept through: 5af93d9 — foundation ticked last tick
  (3871eba, quiet). Rotation: foundation → model → formats → pipeline
  → judges → provider → verbs.
- This tick: POSTURE SWEEP — job 4, model subsystem (kind/contract/
  compose/schema/roster), the rotation's next per last tick's note.
  `git log 1c5b0a9..HEAD -- src/kind.rs src/contract.rs src/compose.rs
  src/schema.rs src/roster.rs` is empty — untouched since model's own
  last full sweep (1c5b0a9, which filed
  KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH and
  COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE). Per the posture-sweep rule
  ("on a subsystem untouched since its last sweep, skip forward"), no
  re-read: quiet-on-clean, rotation advances alone. Checked ahead for
  next tick's benefit: formats (frontmatter/document/json_manifest/
  toml_document) is NOT clean-skippable — `git log 07a9c04..HEAD --
  src/frontmatter.rs src/document.rs src/json_manifest.rs
  src/toml_document.rs` shows 87221b2 (the
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH ship, widening
  json_manifest.rs's and toml_document.rs's UnitShape matches) —
  formats needs a fresh whole-file read against engineering.md's
  lenses next tick, not a skip.
- Queue: 27 pending, unchanged — 6 pickable OPEN
  (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  JSON-MANIFEST-READ-DECODE-CONSOLIDATE,
  TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE,
  COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION — pairwise file-disjoint, count
  re-verified this tick), 19 chained blockedBy (unchanged links, all
  still resolve to live tags), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — reasons
  unchanged, untouched this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — posture sweep resumes at `formats`
(`frontmatter`/`document`/`json_manifest`/`toml_document`), which
87221b2 touched since its own last sweep (07a9c04) — next tick reads
it whole against engineering.md's lenses rather than skipping.
