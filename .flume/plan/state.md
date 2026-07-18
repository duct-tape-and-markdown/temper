# Plan state

- Spec derived through: 4adb1fb
- Audited through: 2f44341
- Residue swept through: 2f44341
- Posture swept through: provider done — verbs next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  2f44341..HEAD -- src/ sdk/src/ tests/` empty (every commit landing in
  the window since 2f44341 is a `plan:` tick, none touching those trees).
  Job 4 was live: `Posture swept through: judges done — provider next`
  named the next subsystem explicitly. Checked `provider`'s own last-
  full-sweep window (ad66e46, the prior `posture-sweep provider` tick,
  which filed BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE) against HEAD:
  `git log ad66e46..HEAD -- src/builtin.rs src/builtin_kind.rs` came back
  non-empty — 394b03f (`build: relocate kind_narrowing_clause to
  compose.rs`) touches src/builtin.rs. Per the posture-sweep rule, a
  touched subsystem cannot skip forward; read both provider files in
  full this tick rather than trusting the rotation cursor alone.
  `git show 394b03f -- src/builtin.rs` confirms a clean extraction: the
  function's whole body left with it, the import line narrowed from
  `contract::{self, Clause, Contract, Predicate, Severity}` to
  `contract::{Clause, Contract}` in the same commit — no orphaned
  import, no dead re-export, no residue. Read src/builtin.rs (71 lines)
  and src/builtin_kind.rs (1130 lines) whole. builtin.rs: `contract`/
  `contracts` both grep-verified live outside the module (main.rs:62,
  eight tests/*.rs files) — no new zero-consumer surface. builtin_kind.rs:
  `qualified` (508-522) and the `Result`-wrapped `definition`/
  `definitions` are the two already-queued defects
  (BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE, blocking
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE) — re-verified still present,
  not re-filed. `skill_features`/`rule_features`/`features` all
  grep-verified live outside the module (tests/extract_equivalence.rs,
  tests/acceptance.rs, src/install.rs, src/main.rs, src/json_manifest.rs,
  src/kind.rs, src/drift.rs, six more tests/*.rs files). No wildcard
  match arms, no TODO/FIXME, no non-test unwrap/expect/panic (every hit
  sits inside `#[cfg(test)] mod tests`, 591-1130). No cohesion split or
  duplicate-matcher residue found — the file is a flat kind std-lib, one
  job. Verdict: quiet-on-clean, verified rather than skipped — rotation
  cursor advances alone.
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

Plan continues: yes — posture sweep resumes at `verbs`
(`main`/`install`/`bundle`/`lib`/`test_support`), the roster's next
subsystem, once nothing above it is live.
