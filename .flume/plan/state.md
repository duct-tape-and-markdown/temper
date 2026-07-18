# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: 785abd7 — unchanged. `git log 785abd7..HEAD -- src/
  tests/ sdk/` is empty (the three commits past it, 5d60a8a, 269a4a1,
  cd286da, are plan-only) — nothing to reconcile.
- Residue swept through: 785abd7 — unchanged, same empty window.
- Posture swept through: pipeline next (mid-rotation) — model swept
  quiet last tick, rotation advanced alone to `formats`, which the
  prior tick's note flagged NOT clean-skippable (d577bdf/d65af7d
  touched json_manifest.rs/toml_document.rs). Read whole this tick;
  see below.
- This tick: POSTURE SWEEP — job 4, formats subsystem (`frontmatter`/
  `document`/`json_manifest`/`toml_document`). Read all four files in
  full against every section of `specs/process/engineering.md` plus
  the sweep's cohesion/dead-plumbing lenses.
  d577bdf (JSON-MANIFEST-READ-DECODE-CONSOLIDATE) and d65af7d
  (TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE) read clean in place: no
  residue left at their sites. Cross-checked every `pub`/`pub(crate)`
  item's consumer count by grep across all four files for
  zero-consumer surface (`engineering.md`, "An export earns its
  consumer"): `fold_file_id`, `Member::field`/`has_field`,
  `item_to_json`/`value_to_json`, `Satisfies::new`, `write_document`,
  `CollectionSegment`, `DocumentMember::parse`/`read`,
  `Manifest::parse` all read live external callers (main.rs, bundle.rs,
  builtin_kind.rs, graph.rs, engine.rs, kind.rs, drift.rs, install.rs,
  tests) — no new gap. Exhaustive-match discipline ("A shared concept
  is one type") holds: every `UnitShape` match in all three loaders
  (frontmatter's `from_source_rooted`, json_manifest's
  `DocumentMember::parse`, toml_document's `parse`) names every variant,
  no `_` arm. All `FrontmatterError`/`JsonManifestError`/
  `TomlDocumentError` variants are live-constructed (no dead-plumbing).
  Architecture invariant check: `json_manifest.rs`'s `Manifest::read_kind`
  still imports `crate::import` (the already-queued, chain-blocked
  JSON-MANIFEST-DISCOVERY-BOUNDARY-RESTORE) — no new edge found.
  **One new finding**: `json_manifest.rs`'s `DocumentMember::parse`
  (158-174) and `Manifest::parse` (359-371) each independently hash the
  raw bytes, `serde_json::from_str` into a `JsonValue`, and require the
  top level be an `Object` else `Malformed` — byte-identical job, two
  authored copies differing only in a noun ("document" vs "manifest").
  Filed JSON-MANIFEST-TOP-LEVEL-OBJECT-PARSE-CONSOLIDATE, `per`
  engineering.md "One job, one home", `blockedBy`
  FORMAT-READ-UTF8-DECODE-CONSOLIDATE (the only open entry sharing
  json_manifest.rs — file-disjointness only, no functional dependency).
  Checked ahead for next tick: pipeline (`drift`/`import`/`read`/
  `builtin_lock`; `placement` not yet shipped) is clean-skippable —
  `git log 662cf07..HEAD -- src/drift.rs src/import.rs src/read.rs
  src/builtin_lock.rs` is empty. Next tick advances the rotation alone,
  no whole read.
- Queue: 33 pending — 4 pickable OPEN (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  FORMAT-READ-UTF8-DECODE-CONSOLIDATE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION — pairwise file-disjoint,
  re-verified this tick), 27 chained blockedBy (26 unchanged links plus
  the new entry's, all resolve to live tags), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — reasons
  unchanged, untouched this window). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. Refactor
  captures: 0 live. Friction: 1 live (build-worktree-commits-land-on-
  main-branch.md, unchanged). Inbox empty.
  Disjointness re-checked: no two OPEN entries share a file (the new
  entry is blockedBy, not open).

Plan continues: yes — posture sweep resumes at `pipeline`, clean-skippable
per the note above — the next tick advances the rotation alone.
