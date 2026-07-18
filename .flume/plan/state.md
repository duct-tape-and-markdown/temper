# Plan state

- Spec derived through: 4adb1fb
- Audited through: 2f44341
- Residue swept through: 2f44341
- Posture swept through: 04cbd6d — rotation closed (foundation, model,
  formats, pipeline, judges, provider, verbs all swept this cycle);
  foundation next cycle
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  2f44341..HEAD -- src/ sdk/src/ tests/` empty (every commit landing in
  the window since 2f44341 is a `plan:` tick, none touching those trees).
  Job 4 was live: `Posture swept through: provider done — verbs next`
  named the next subsystem explicitly. Checked `verbs`'s own last-full-
  sweep window (33d00be, the prior `posture-sweep verbs` tick) against
  HEAD: `git log 33d00be..HEAD -- src/main.rs src/install.rs src/bundle.rs
  src/lib.rs src/test_support.rs` came back non-empty — 5ad2e61 (`build:
  frontmatter.rs test fixtures swap to synthetic kinds via test_support`)
  touches src/test_support.rs, and 62559ef (`build: collapse infallible
  discovery result signatures to bare collections`) touches src/install.rs
  and src/main.rs. Per the posture-sweep rule, a touched subsystem cannot
  skip forward; read all five verbs files in full this tick rather than
  trusting the rotation cursor alone.
  Read src/lib.rs (59 lines — clean module list), src/bundle.rs (434 lines
  whole — clean; its `builtin_kind::definition(kind)` `Ok(Some(..))`
  pattern at 317/402 is already BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE's
  own queued target, re-verified present, not re-filed), src/test_support.rs
  (67 lines — `skill_kind`/`rule_kind` grep-verified consumed by
  src/frontmatter.rs's own inline tests, matching 5ad2e61's commit message;
  not zero-consumer), src/main.rs (2759 lines whole), and src/install.rs
  (1898 lines whole). `git show 62559ef -- src/main.rs src/install.rs`
  confirms a clean mechanical extraction: three `?` operators dropped after
  `discover_builtin`/`discover_nested_file`/`discover_kind_files` collapsed
  to bare collections, no orphaned import, no residue.
  Grepped both large files for wildcard match arms, TODO/FIXME, and traced
  every `pub` item to its consumer. `src/install.rs:531`'s `_ => modelines
  += 1` is INSTALL-PLACEMENT-KIND-ENUM's own queued target, re-verified
  present; `:896`'s `_ => fresh_settings(..)` and `:1196`'s `_ =>
  "members"` are ordinary Option/string-keyed fallbacks over kind names
  (data, not a closed enum — representation.md, "kind"), not the
  shared-concept defect shape. Every `pub` item in install.rs traces to a
  real outside consumer (`npm_program` → tests/common/mod.rs;
  `InstallEntry`/`DiscoveryReport` → tests/install.rs field access;
  `GuardedManifest`/`GuardVerdict`/`Represent`/`InstallOutcome` → main.rs
  and tests) except `GUARD_MANIFEST_MESSAGE`, already queued
  (INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE). The `matches_projection`/
  `manifest_write_findings` duplication (INSTALL-PROJECTION-MATCH-
  CONSOLIDATE) and the `placement_lines`/`is_placement_comment`/marker-const
  relocation (PLACEMENT-MODULE-EXTRACTION) both re-verified present at
  their queued line ranges, not re-filed. main.rs's double `resolve_kind_
  units` shape across `explain`/`gate`/`collect_directive_members` and the
  secondary `builtin_kind::definitions()` re-call at the coverage-note call
  site are GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST's own queued targets,
  re-verified present. No new cohesion split, no new dead plumbing, no new
  zero-consumer export found. Verdict: quiet-on-clean, verified rather than
  skipped. `verbs` is the roster's last subsystem (architecture.md codemap:
  foundation, model, formats, pipeline, judges, provider, verbs) — this
  closes the full rotation cycle; next cycle resumes at foundation.
- Queue: 26 pending — 10 pickable OPEN (READ-EXPLAIN-STRAND-VISIBILITY-
  NARROW, ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE,
  REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-
  PRUNE, TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE,
  ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE,
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE,
  FRONTMATTER-COMPANIONS-ZERO-CONSUMER-PRUNE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; all disjoint files, unchanged from
  last tick's pairwise cross-check — zero path collisions), 14 chained
  blockedBy (BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE →
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE; DRIFT-LOCK-ROW-WALK-
  CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-
  EXTRACTION → {EXTRACT-FOUNDATION-BOUNDARY-RESTORE →
  {KIND-ZERO-CONSUMER-EXPORTS-PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-
  MATCH → CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP →
  DRIFT-EMIT-OUTCOME-LABEL-ZERO-CONSUMER-PRUNE, DRIFT-SOURCE-DEP-
  PARSE-HOIST → GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE →
  GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST}, INSTALL-PROJECTION-MATCH-
  CONSOLIDATE → INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE →
  INSTALL-PLACEMENT-KIND-ENUM}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: no — the posture-sweep rotation closed with an empty
forward window on every other job (inbox, spec delta, and reconciliation
all quiet); no live input remains this tick.
