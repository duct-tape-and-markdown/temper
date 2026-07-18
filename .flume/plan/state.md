# Plan state

- Spec derived through: 4adb1fb
- Audited through: 2f44341
- Residue swept through: 2f44341
- Posture swept through: ad66e46 — rotation closed (foundation, model,
  formats, pipeline, judges, provider, verbs all swept this cycle);
  foundation next cycle
- This tick: POST-SHIP RECONCILIATION over `60faee0..2f44341` (`git log
  60faee0..HEAD -- src/ sdk/src/ tests/`: 4 build commits, all shipped by
  `2f44341`). Audit: read each shipped commit's diff and the resulting
  files on disk. `62559ef` (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE) collapses
  six `discover_*` signatures to bare collections, retires
  `ImportError::ReadDir` and `JsonManifestError::Discovery` — grep-verified
  zero remaining references to either. `5ad2e61`
  (FRONTMATTER-TEST-SYNTHETIC-KINDS) swaps frontmatter.rs's test fixtures
  to synthetic kinds built in test_support.rs, dissolving the
  frontmatter→builtin_kind test-only edge architecture.md names as debt.
  `394b03f` (ROSTER-BUILTIN-KIND-NARROWING-RELOCATE) moves
  `kind_narrowing_clause` from builtin.rs (provider) to compose.rs (model),
  dropping roster.rs's upward `builtin` import. `6618b47`
  (DOCUMENT-RETIRED-FENCE-SURFACE-PRUNE) prunes the zero-consumer `+++`
  Document/DocumentError machinery from document.rs — grep-verified no
  surviving reference; `TomlDocumentError` (an unrelated live type) is the
  only near-name-match. `cargo test` (all crates) and `cargo clippy
  --all-targets -- -D warnings` both green at HEAD. No residue found; all
  four pending entries were already removed from pending.json by the ship
  commit itself. Re-tested every stale gate per the audit motion:
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION's blocker, DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, shipped — its citations (src/drift.rs, tests/install.rs,
  tests/emit.rs) are unmoved since f404e48 (`git diff f404e48..2f44341`
  empty on all three), so it reopens unchanged. No other blockedBy/parked
  gate's named condition cleared (every other blocker tag is still
  unshipped; the two parked entries' human-action conditions still hold).
  Glanced `.flume/metrics.jsonl`: DISCOVERY-INFALLIBLE-RESULT-COLLAPSE cost
  9 plan-phase rows plus 1 build-phase row this window — the largest entry
  in the window, consistent with its 4-file+tests blast radius; nothing
  anomalous beyond what the entry's own scope predicted, so no new sizing
  signal to act on. Aside, not actionable this tick: architecture.md's
  "declared debt" paragraph names three tension edges; the
  frontmatter→builtin_kind one is now resolved in code (5ad2e61) while the
  other two (drift→install, extract's upward imports) are not — the
  paragraph is one-third stale prose a future `specs:` session should trim,
  never a plan-tick edit (spec-system.md, "Change ceremony": specs commits
  are the session's, not an autonomous phase's).
- Queue: 19 pending — 5 pickable OPEN (READ-EXPLAIN-STRAND-VISIBILITY-
  NARROW, ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE,
  REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-
  PRUNE, DRIFT-LOCK-ROW-WALK-CONSOLIDATION — newly reopened this tick, all
  disjoint files), 12 chained blockedBy (DRIFT-LOCK-ROW-WALK-CONSOLIDATION
  → DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-EXTRACTION →
  {EXTRACT-FOUNDATION-BOUNDARY-RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-
  PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH →
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, DRIFT-SOURCE-DEP-PARSE-HOIST →
  GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE → GATE-KIND-UNITS-DOUBLE-RESOLVE-
  HOIST}, INSTALL-PROJECTION-MATCH-CONSOLIDATE → INSTALL-GUARD-MANIFEST-
  MESSAGE-PRUNE → INSTALL-PLACEMENT-KIND-ENUM}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — the posture-sweep rotation's cursor (`ad66e46`) now
sits behind a forward window (`ad66e46..HEAD -- src/ sdk/src/ tests/`) that
touched multiple subsystems (this tick's four ship commits), so the next
tick opens the new cycle at `foundation`, the first subsystem in rotation
order.
