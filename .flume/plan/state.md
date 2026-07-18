# Plan state

- Spec derived through: 4adb1fb
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: ad66e46 — rotation closed (foundation, model,
  formats, pipeline, judges, provider, verbs all swept this cycle);
  foundation next cycle
- This tick: POSTURE SWEEP — `verbs` subsystem (`src/main.rs`,
  `src/install.rs`, `src/bundle.rs`, `src/lib.rs`, `src/test_support.rs`,
  architecture.md's codemap). Inbox and refactor captures were empty; spec
  delta was empty (`git log 4adb1fb..HEAD -- specs/` empty); the post-ship
  reconciliation window was empty too (`git log 60faee0..HEAD -- src/
  sdk/src/ tests/` empty — every commit since 60faee0 has been a `plan:`
  tick touching only `.flume/**`). `verbs` was next in rotation (`provider`
  swept last tick) and closes the first full cycle through
  architecture.md's seven-subsystem codemap.
  Delegated the read to a subagent, then verified every finding on disk
  before filing. `src/lib.rs` (58 lines) and `src/test_support.rs` (13
  lines): minimal, every const/fn has a real external consumer, clean.
  `src/bundle.rs` (434 lines): read fully, defers to `drift`/`json_manifest`
  for its write faces as documented, no duplicate encoders, clean.
  `src/main.rs` (2759 lines): `gate()` (847) and `explain()` (513) each
  resolve every kind's members via `resolve_kind_units` TWICE per
  invocation — once through the two-greens dispatch (`kind_features`/
  `builtin_features_by_kind`), again through `collect_directive_members`
  (2166-2197) independently re-deriving `builtin_kind::definitions()`,
  `partition_kind_rows`, and `resolve_kind_units` per kind a second time.
  Every member document in the corpus is read and parsed twice per run —
  filed GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST (engineering.md, "Cost scale
  is hoisted, and pinned by count").
  `src/install.rs` (1898 lines): three findings. (1) `matches_projection`
  (744-750) and `manifest_write_findings` (783-827) each independently
  normalize backslashes and suffix-compare `file_path` against a target
  list — the identical matcher written twice — filed
  INSTALL-PROJECTION-MATCH-CONSOLIDATE ("One job, one home"). (2)
  `GUARD_MANIFEST_MESSAGE` (131) is `pub` with zero consumer outside its
  own module (`render_manifest_findings`, same file) — unlike its sibling
  `GUARD_MESSAGE`, which main.rs reads directly and whose doc comment
  states why it's public — filed INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE ("An
  export earns its consumer"). (3) `InstallEntry.placement` is four bare
  `&'static str` consts matched with a wildcard fallback in
  `gate_installed` (522-533) — `MODELINE` is never named, reached only via
  `_ => modelines += 1`, which would silently absorb a future fifth
  placement too — filed INSTALL-PLACEMENT-KIND-ENUM ("A shared concept is
  one type"). All four new entries touch main.rs or install.rs, both
  already touched by open/chained entries (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE open on both files; PLACEMENT-MODULE-EXTRACTION the last
  chained entry touching install.rs; GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE
  the last chained entry touching main.rs) — each serialized `blockedBy`
  accordingly (pending-entry.md, "Disjoint, or serialized"); the three
  install.rs findings additionally chain onto each other for the same
  reason. No functional dependency in any of the four cases.
- Queue: 23 pending — 8 pickable OPEN (unchanged this tick: DISCOVERY-
  INFALLIBLE-RESULT-COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS,
  ROSTER-BUILTIN-KIND-NARROWING-RELOCATE, DOCUMENT-RETIRED-FENCE-SURFACE-
  PRUNE, READ-EXPLAIN-STRAND-VISIBILITY-NARROW, ENGINE-SELECTOR-LABEL-
  ZERO-CONSUMER-PRUNE, REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-
  QUALIFIED-ZERO-CONSUMER-PRUNE — all disjoint files), 13 chained blockedBy
  (the existing DISCOVERY → DRIFT-LOCK-ROW-WALK-CONSOLIDATION →
  DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-EXTRACTION →
  {EXTRACT-FOUNDATION-BOUNDARY-RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-
  PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH →
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, DRIFT-SOURCE-DEP-PARSE-HOIST →
  GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE → GATE-KIND-UNITS-DOUBLE-RESOLVE-
  HOIST}, INSTALL-PROJECTION-MATCH-CONSOLIDATE → INSTALL-GUARD-MANIFEST-
  MESSAGE-PRUNE → INSTALL-PLACEMENT-KIND-ENUM}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: no — the posture-sweep rotation closed with an empty
forward window on every other job (inbox, spec delta, and reconciliation
all quiet); no live input remains this tick.
