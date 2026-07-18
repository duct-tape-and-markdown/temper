# Plan state

- Spec derived through: 4adb1fb
- Audited through: 2f44341
- Residue swept through: 2f44341
- Posture swept through: model done — formats next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  2f44341..HEAD -- src/ sdk/src/ tests/` empty apart from the prior tick's
  own build-adjacent commits (no fresh post-ship window). Job 4 was live:
  `Posture swept through: foundation done — model next` named the next
  subsystem explicitly, so swept `model` (architecture.md codemap: `kind`,
  `contract`, `compose`, `schema`, `roster`) — `git log a83fe23..HEAD --
  src/kind.rs src/contract.rs src/compose.rs src/schema.rs src/roster.rs`
  (a83fe23 was this subsystem's last full sweep) showed one touch,
  394b03f's `kind_narrowing_clause` relocation into compose.rs, so this
  was a fresh read, not a skip-forward. Read all five files in full.
  `contract.rs`/`compose.rs`/`schema.rs`/`roster.rs` clean on every
  engineering.md lens — every `pub`/`pub(crate)` item grep-verified to a
  real external caller (`schema::emit` from main.rs; `roster::selections`/
  `admissibility` from graph.rs/main.rs; `compose::kind_narrowing_clause`
  from roster.rs plus tests/contract_template.rs). `contract.rs`'s
  `Predicate::RequireSections` unconstructable shape and three of
  `kind.rs`'s zero-consumer `label`/`qualified_name` methods are already
  queued (CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, KIND-ZERO-CONSUMER-EXPORTS-
  PRUNE) — not re-filed. Two fresh gaps surfaced in `kind.rs`, both
  verified on disk (symbol, line, grep for consumers) before filing:
  - `CustomKind::member_document` (957-964) — zero consumer anywhere,
    not even a test; its own doc comment's claimed callers
    (frontmatter.rs, import.rs) actually dispatch through the kind's
    declared `governs.glob` instead → filed
    `KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE`, the export-earns-its-
    consumer bar the queue's four existing visibility/prune entries
    already establish as precedent.
  - `builtin_kind::definition`/`definitions` (508-536) wrap an infallible
    plain-data lookup in `Result<_, KindError>` — `KindError` (kind.rs
    1479-1480) is a doc-conceded empty enum, statically unreachable
    `Err`, the exact residue class DISCOVERY-INFALLIBLE-RESULT-COLLAPSE
    (2f44341) fixed for import.rs's `discover_*` family → filed
    `BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE`, serialized behind
    BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE (shared builtin_kind.rs,
    and that entry's removal of `qualified` — the third `KindError`-
    returning fn — simplifies this one's diff).
  `owns_source` (kind.rs 985-997) was considered and rejected: its one
  caller is `tests/nested_member.rs:448`, an external test, which counts
  as an earned consumer per pending-entry.md ("a test counts") even
  though nothing in production calls it.
- Queue: 23 pending — 8 pickable OPEN (READ-EXPLAIN-STRAND-VISIBILITY-
  NARROW, ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE,
  REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-
  PRUNE, TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE,
  ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE,
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE (new),
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; all disjoint files, verified by
  cross-checking every open entry's files[] pairwise), 13 chained
  blockedBy (BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE →
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE (new); DRIFT-LOCK-ROW-WALK-
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

Plan continues: yes — posture sweep resumes at `formats`
(`frontmatter`/`document`/`json_manifest`/`toml_document`), the roster's
next subsystem, once nothing above it is live.
