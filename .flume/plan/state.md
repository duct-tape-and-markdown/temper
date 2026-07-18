# Plan state

- Spec derived through: 4adb1fb
- Audited through: 2f44341
- Residue swept through: 2f44341
- Posture swept through: formats done — pipeline next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  2f44341..HEAD -- src/ sdk/src/ tests/` empty apart from prior ticks' own
  build-adjacent commits (no fresh post-ship window). Job 4 was live:
  `Posture swept through: model done — formats next` named the next
  subsystem explicitly, so swept `formats` (architecture.md codemap:
  `frontmatter`, `document`, `json_manifest`, `toml_document`) — `git log
  20b7b21..HEAD -- src/frontmatter.rs src/document.rs src/json_manifest.rs
  src/toml_document.rs` (20b7b21 was this subsystem's last full sweep)
  showed three touches (6618b47 Document/DocumentError prune, 5ad2e61
  frontmatter.rs test-fixture swap, 62559ef discovery-result collapse), so
  this was a fresh read, not a skip-forward. Read all four files in full.
  `document.rs` and `toml_document.rs` clean on every engineering.md lens —
  every pub item and error variant grep-verified to a real caller or a
  reachable refusal path, both with solid inline/integration test coverage.
  Two fresh gaps surfaced in `frontmatter.rs`/`json_manifest.rs`/
  `toml_document.rs`, both verified on disk (symbol, line, grep for
  consumers) before filing:
  - `frontmatter.rs`'s `Member.companions` field and its sole producer
    `scan_companions` (pub(crate)) — zero consumer anywhere, not even a
    test beyond the file's own; the fn's own doc comment claims an
    `import.rs` caller that grep-verified does not exist (import.rs's only
    "companion" hits are unrelated prose) → filed
    `FRONTMATTER-COMPANIONS-ZERO-CONSUMER-PRUNE`, the export-earns-its-
    consumer bar the queue's existing visibility/prune entries already
    establish as precedent; consistent with builtins.md's own stance that
    non-markdown supporting files stay unmodeled.
  - `json_manifest.rs`'s `DocumentMember::parse` and `toml_document.rs`'s
    `parse` each hold a non-exhaustive match/let-else over the shared
    `UnitShape` enum (a `Some(_)` wildcard and an unnamed `else` arm,
    respectively), unlike `frontmatter.rs`'s own exhaustive match over the
    same four-variant enum in the same subsystem → filed
    `DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH`, the same
    exhaustive-match bar CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH already
    established for `Predicate`, applied here to `UnitShape`; behavior is
    unchanged, only future-variant safety.
  A third candidate — the `fs::read(source_file).map_err(..Io{..})`
  boilerplate repeated across all three read-face `Io` variants — was
  considered and rejected: each format's own doc comments explicitly
  frame their error types as deliberate per-format peers ("mirrors...",
  "the peer of..."), so this is the format-owns-its-vocabulary asymmetry
  (builtins.md: "Format implementations are engine code"), not residue.
- Queue: 25 pending — 9 pickable OPEN (READ-EXPLAIN-STRAND-VISIBILITY-
  NARROW, ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE,
  REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-
  PRUNE, TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE,
  ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE,
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE,
  FRONTMATTER-COMPANIONS-ZERO-CONSUMER-PRUNE (new),
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH (new),
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; all disjoint files, verified by
  cross-checking every open entry's files[] pairwise), 13 chained
  blockedBy (BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE →
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE; DRIFT-LOCK-ROW-WALK-
  CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-
  EXTRACTION → {EXTRACT-FOUNDATION-BOUNDARY-RESTORE →
  {KIND-ZERO-CONSUMER-EXPORTS-PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-
  MATCH → CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, DRIFT-SOURCE-DEP-PARSE-
  HOIST → GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE →
  GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST}, INSTALL-PROJECTION-MATCH-
  CONSOLIDATE → INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE →
  INSTALL-PLACEMENT-KIND-ENUM}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — posture sweep resumes at `pipeline`
(`drift`/`import`/`read`/`builtin_lock`/`placement`), the roster's
next subsystem, once nothing above it is live.
