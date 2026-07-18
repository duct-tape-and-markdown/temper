# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: 73c76ca — unchanged; no commit past it has landed.
- Residue swept through: 73c76ca — unchanged; no commit past it has landed.
- Posture swept through: verbs next (mid-rotation) — judges bulk-skipped
  this tick (quiet: `git log cc1a40b..HEAD -- src/engine.rs src/graph.rs
  src/dial.rs src/coverage.rs src/coverage_note.rs src/display.rs
  src/reporter.rs` empty, git-log test only, no file read); provider read
  and swept (touched: 0282dc7 edited src/builtin_kind.rs) — quiet-on-clean.
- This tick: POSTURE SWEEP — provider (`src/builtin.rs`,
  `src/builtin_kind.rs`). Judges skipped forward in bulk per the rule
  (untouched since cc1a40b). Provider was touched (0282dc7's
  `builtin_kind::definition` API-shape edit), so it does not bulk-skip:
  read both files in full against every `specs/process/engineering.md`
  section plus the sweep's own cohesion/dead-plumbing lenses. No `_ =>`
  fallthrough in either file; every pub export
  (`contract`/`contracts`/`definition`/`definitions`/`skill_features`/
  `rule_features`) grep-confirmed a consumer outside its own module; no
  duplicate matcher/normalizer against another module; `all_kinds()`'s
  per-call reconstruction is fixed-size Rust-literal data, not tree-scaled
  work, so the cost-hoist section doesn't reach it. The one live shape —
  `definitions()` wraps an infallible lookup in `Result<_, KindError>`,
  confirmed verbatim at lines 501-519 (the doc comment says so itself: "the
  `Result` is kept for API stability") — already matches
  BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE; no entry edit needed. Verdict:
  quiet-on-clean. Rotation advances to verbs — also touched by 0282dc7
  (main.rs/bundle.rs), so next tick sweeps it (one touched subsystem read
  per tick; verbs was not read this tick).
- Queue: 33 pending — unchanged (no entry filed, dropped, or edited this
  tick; posture sweep found nothing new). 3 pickable OPEN
  (DRIFT-INCLUDE-SOURCE-PATH-CWD-LEAK, BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE,
  JSON-MANIFEST-TOP-LEVEL-OBJECT-PARSE-CONSOLIDATE — pairwise file-disjoint),
  28 chained blockedBy (all resolving to live tags), 2 parked on human
  action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds), neither touched. Refactor
  captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep is still mid-rotation (verbs
next, already known touched by 0282dc7) and no commit past 73c76ca has
landed to re-trigger reconciliation, so the sweep is the next live input.
