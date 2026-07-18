# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: c1b0f51 — advanced from 4e46eac.
- Residue swept through: c1b0f51 — advanced from 4e46eac.
- Posture swept through: verbs next (mid-rotation) — provider read and
  swept this tick (touched: builtin_kind.rs by 516f8f6 since its own last
  full sweep at fe3ff3f) — 1 new finding.
- This tick: POSTURE SWEEP — provider (`src/builtin.rs`,
  `src/builtin_kind.rs`), per the `posture-sweep` rule. `git log
  fe3ff3f..HEAD -- src/builtin.rs src/builtin_kind.rs` names one touching
  commit, 516f8f6 (`definitions()` Result-wrapper collapse) — already
  shipped clean, no residue: `KindError` deleted, all call sites unwrapped,
  no stale `# Errors` doc left behind. Full read of both files against
  every `engineering.md` section plus the sweep's cohesion/dead-plumbing
  lenses: no `_ =>` wildcards over `Predicate`/`ClauseRow`/any shared enum,
  no zero-consumer `pub` (`contract`/`contracts`/`definition`/
  `definitions`/`skill_features`/`rule_features`/`features` all have real
  external callers, grep-verified), no cohesion split wanted. One real
  finding: `builtin.rs`'s `contract_for_kind` (36-48) hand-rolls the exact
  filter-by-kind/build-`Contract` shape `compose::default_contract_from_rows`
  (141-155) already implements over the committed lock — compose.rs's own
  doc comment (132-133) names them "the same lift," but the code duplicates
  rather than shares it. Filed BUILTIN-CONTRACT-FOR-KIND-CONSOLIDATE,
  serialized behind CONTRACT-WHEN-GUARD-CLAUSE-FORM (the sole queue entry
  already touching `builtin.rs`, shared-file safety) — that entry's own
  `files[]` already flagged this exact duplication as a to-verify item, so
  this narrows its scope. Also swept: `all_kinds()`/`builtin_kind::definitions()`
  is rebuilt from scratch at 6+ call sites in `main.rs`'s `gate`/`explain`
  paths — considered against "Cost scale is hoisted," but the roster is a
  fixed 14-entry Rust literal, not work that scales with the consumer's
  harness (the section's own framing: tree walks, file reads, glob
  compilations), and no field-measured cost drives it — correctly not
  filed per the section's measure-first bar. Separately, cross-checking
  `builtin_kind.rs` against `architecture.md`'s Invariants section (naming
  it one endpoint of the now-dissolved `frontmatter → builtin_kind`
  debt edge) surfaced that the section's prose still lists that edge as
  live/unshipped when `test_support.rs`/`frontmatter.rs` on disk (5ad2e61)
  and the already-drained `FRONTMATTER-TEST-SYNTHETIC-KINDS` entry both
  confirm it shipped — spec prose plan cannot edit, so filed as friction
  (`plan-architecture-debt-list-stale.md`) rather than invented into
  pending or open-questions.
- Queue: 37 pending — 1 added this tick (blockedBy). 1 pickable OPEN
  (PLACEMENT-MODULE-EXTRACTION), 31 chained blockedBy (all resolving to
  live tags), 5 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER, IMPORT-ROLLUP-WRITER-PLACEMENT,
  READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION). Open forks
  unchanged: (multi-harness-projection), (lazy-grounds), neither touched.
  Refactor captures: 0 live. Friction: 1 live (filed this tick,
  plan-architecture-debt-list-stale.md — awaits a human specs/ edit).
  Inbox: 0 notes.

Plan continues: yes — no live inbox/spec-delta/reconciliation input
remains, so the next tick resumes the posture sweep's own rotation at
verbs (`src/main.rs`, `src/install.rs`, `src/bundle.rs`, `src/lib.rs`,
`src/test_support.rs`), already known not-clean-skippable from this
tick's forward check (516f8f6/0062150 both touch verbs-subsystem files
since its own last full sweep at 0b9d1f9).
