# Plan state

- Spec derived through: 4adb1fb
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: pipeline done — judges next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  60faee0..HEAD -- src/ sdk/src/ tests/` empty (no post-ship window to
  audit or sweep). Job 4 was live — rotation mid-way (`formats done —
  pipeline next`) makes it live regardless of window content. Swept the
  `pipeline` subsystem (architecture.md codemap: `drift`, `import`,
  `read`, `builtin_lock` — `placement` not yet on disk, still a queued
  `new` file under PLACEMENT-MODULE-EXTRACTION), delegating the read to a
  background agent over engineering.md's postures, then independently
  verifying every candidate before filing:
  - `import.rs` and `builtin_lock.rs` are clean.
  - `drift.rs`'s already-known duplication (the top-level lock-row walk,
    emit()'s double-parse) is already queued
    (DRIFT-LOCK-ROW-WALK-CONSOLIDATION, DRIFT-EMIT-LOCK-PARSE-HOIST) — not
    re-filed. A distinct, verified finding: `source_deps` (2472) and its
    four callers (`layout_imports`, `includes`, `layout_import_stale`,
    `include_stale`) each independently open+parse `lock.toml`, on top of
    `read_declarations`'s own parse — confirmed on disk as 5 full parses
    of the same file inside one `gate()` call (main.rs:858, 891, 1208-9)
    and a matching pattern in `explain()` (main.rs:513,525,587). Filed
    DRIFT-SOURCE-DEP-PARSE-HOIST (`engineering.md`, "Cost scale is
    hoisted, and pinned by count") — blockedBy EXTRACT-FOUNDATION-
    BOUNDARY-RESTORE, the last existing-chain entry sharing drift.rs and
    main.rs.
  - `read.rs` yielded a new finding: `impact` (593), `context` (856),
    `requirements` (1254), `field` (1468) are `pub fn` with zero caller
    outside `explain` (218) in the same file — grep-verified zero hits
    for their qualified names across src/, tests/, sdk/, and
    `tests/read_verbs.rs` calls only `why`/`explain` directly. Filed
    READ-EXPLAIN-STRAND-VISIBILITY-NARROW (`engineering.md`, "An export
    earns its consumer") — gate open, disjoint (only src/read.rs).
  Rotation advances: `pipeline` done, `judges` next (`engine`, `graph`,
  `dial`, `coverage`, `coverage_note`, `display`, `reporter`).
- Queue: 15 pending — 5 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS, ROSTER-BUILTIN-KIND-
  NARROWING-RELOCATE, DOCUMENT-RETIRED-FENCE-SURFACE-PRUNE,
  READ-EXPLAIN-STRAND-VISIBILITY-NARROW; all disjoint files), 8 chained
  blockedBy (DRIFT-LOCK-ROW-WALK-CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-
  HOIST → PLACEMENT-MODULE-EXTRACTION → EXTRACT-FOUNDATION-BOUNDARY-
  RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-PRUNE → CONTRACT-DECLARED-KEYS-
  EXHAUSTIVE-MATCH → CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP,
  DRIFT-SOURCE-DEP-PARSE-HOIST}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — posture sweep resumes at `judges` (`engine`,
`graph`, `dial`, `coverage`, `coverage_note`, `display`, `reporter`), once
nothing above it is live.
