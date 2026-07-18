# Plan state

- Spec derived through: 4adb1fb
- Audited through: 2f44341
- Residue swept through: 2f44341
- Posture swept through: pipeline done — judges next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  2f44341..HEAD -- src/ sdk/src/ tests/` empty apart from prior ticks' own
  build-adjacent commits (no fresh post-ship window). Job 4 was live:
  `Posture swept through: formats done — pipeline next` named the next
  subsystem explicitly, so swept `pipeline` (architecture.md codemap:
  `drift`, `import`, `read`, `builtin_lock`, `placement` — `placement`
  doesn't exist on disk yet, PLACEMENT-MODULE-EXTRACTION is still queued,
  so nothing to sweep there). `git log e2325df..HEAD -- src/drift.rs
  src/import.rs src/read.rs src/builtin_lock.rs` (e2325df was this
  subsystem's last full sweep, in the prior rotation cycle) showed one
  touch, 62559ef (discovery-result collapse, import.rs) — already inside
  the window job 3 fully reconciled through 2f44341, not fresh — so this
  was a genuine re-sweep, not a skip-forward. Read all four files in full
  (drift.rs and read.rs at the lighter pass the rule allows for
  heavily-already-queued files; import.rs and builtin_lock.rs — neither
  touched by any pending entry's internal logic — at full depth).
  `import.rs` clean on every lens: every `pub`/`pub(crate)` surface has a
  real cross-module caller, its cost-scale discipline (the `Discovery`
  per-flavor memoization, walk-count/read-dir-count pins) already matches
  the hoisted-and-pinned bar, no duplicate glob/path-normalize logic.
  `builtin_lock.rs` trivial and clean (one `LazyLock`, one real multi-
  consumer export). `read.rs`'s only zero-consumer surface is already
  READ-EXPLAIN-STRAND-VISIBILITY-NARROW; no unflagged wildcard-match or
  duplicate normalizer found. One fresh gap surfaced in `drift.rs`,
  verified on disk (symbol, line, grep for every consumer) before filing:
  - `EmitOutcome::label` (535-546, `pub fn`) has exactly one call site in
    the whole tree — `render_emit` (2166-2188, line 2180), same file;
    `tests/emit.rs` asserts `render_emit`'s rendered string, never calls
    `.label()` directly → filed `DRIFT-EMIT-OUTCOME-LABEL-ZERO-CONSUMER-
    PRUNE`, the same defect shape ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-
    PRUNE already pruned for `Selector::label` in engine.rs. Serialized
    (`blockedBy`) behind CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, the deepest
    existing chain entry touching drift.rs, since DRIFT-LOCK-ROW-WALK-
    CONSOLIDATION (open) already claims drift.rs and two `open` entries
    can't share a file (pending-entry.md) — no functional dependency.
- Queue: 26 pending — 10 pickable OPEN (READ-EXPLAIN-STRAND-VISIBILITY-
  NARROW, ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE,
  REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-
  PRUNE, TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE,
  ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE,
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE,
  FRONTMATTER-COMPANIONS-ZERO-CONSUMER-PRUNE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; all disjoint files, verified
  programmatically by cross-checking every open entry's files[]
  pairwise — zero path collisions), 14 chained blockedBy
  (BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE →
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE; DRIFT-LOCK-ROW-WALK-
  CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-
  EXTRACTION → {EXTRACT-FOUNDATION-BOUNDARY-RESTORE →
  {KIND-ZERO-CONSUMER-EXPORTS-PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-
  MATCH → CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP →
  DRIFT-EMIT-OUTCOME-LABEL-ZERO-CONSUMER-PRUNE (new), DRIFT-SOURCE-DEP-
  PARSE-HOIST → GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE →
  GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST}, INSTALL-PROJECTION-MATCH-
  CONSOLIDATE → INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE →
  INSTALL-PLACEMENT-KIND-ENUM}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — posture sweep resumes at `judges`
(`engine`/`graph`/`dial`/`coverage`/`coverage_note`/`display`/`reporter`),
the roster's next subsystem, once nothing above it is live.
