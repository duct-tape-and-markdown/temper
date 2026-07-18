# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: c1b0f51 — advanced from 4e46eac.
- Residue swept through: c1b0f51 — advanced from 4e46eac.
- Posture swept through: 285f57b — verbs ticked this tick and closes the
  rotation pass (foundation bulk-skipped, model quiet-on-clean, formats
  quiet-on-clean, pipeline 4 findings, judges bulk-skipped, provider 1
  finding, verbs this tick — 1 refactor capture). All seven subsystems
  from architecture.md's codemap now covered. A fresh cycle (foundation →
  model → formats → pipeline → judges → provider → verbs) opens next time
  the sweep re-arms.
- This tick: POSTURE SWEEP — verbs (`src/main.rs`, `src/install.rs`,
  `src/bundle.rs`, `src/lib.rs`, `src/test_support.rs`), per the
  `posture-sweep` rule. `git log 0b9d1f9..HEAD` over these five files
  names two touching commits, 516f8f6 (`definitions()` Result-wrapper
  collapse, install.rs+main.rs) and 0062150 (`read_file_unit` exhaustive
  match, main.rs) — both re-verified shipped clean: 516f8f6 unwraps every
  `builtin_kind::definitions()?` call site cleanly with no stale `?`/doc
  left behind; 0062150's wildcard replacement matches the cited sibling
  precedents exactly. lib.rs (module list only) and test_support.rs (two
  fixture builders, both grep-verified consumed from `frontmatter.rs`'s
  test module) are clean — no new finding. bundle.rs: every `pub` item
  (`BundleError`, `BundleReport`, `run`, `render`) has a real external
  caller (`tests/bundle.rs`, `main.rs`); its session-start duplication
  against install.rs is already tracked
  (BUNDLE-INSTALL-SESSION-START-SHAPE-CONSOLIDATE), re-verified still
  true. install.rs: the three remaining `_ =>` arms (530, 895, 1195)
  checked — 530 is the already-tracked INSTALL-PLACEMENT-KIND-ENUM
  wildcard; 895 is a two-armed `Option<&str>` match with a trim guard, not
  a shared-enum fallthrough; 1195 (`member_dir`) defaults over
  intentionally-open custom-kind names, not a closed enum — neither
  fileable. matches_projection/manifest_write_findings duplication
  (INSTALL-PROJECTION-MATCH-CONSOLIDATE) and GUARD_MANIFEST_MESSAGE's
  zero-consumer pub (INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE) re-verified
  still true; nothing new in install.rs. main.rs: one real finding, a
  cohesion violation, not mechanical. The module's own header (1-6)
  claims "a thin `clap` dispatch... all logic lives in the library so
  `tests/` can drive it," matching `rust.md`'s rule and
  architecture.md's Invariants ("`main` carries dispatch only"), but the
  file (2762 lines, 47 free functions beyond `main`) holds real corpus
  assembly and judgment instead — `gate` (847-1223), `explain`
  (513-641), clause/edge builders (`clause_from_row` 2485,
  `edges_from_declarations` 2502, `requirement_from_row` 2457,
  `mention_edges_from_declarations` 2542), and admissibility judges
  (`joined_kind_admissibility` 1738, `satisfies_label_admissibility`
  2034, `nested_member_admissibility` 2074, `local_locus_admissibility`
  2341, `governs_collision_diagnostics` 2394,
  `clause_collision_diagnostics` 2299, `kind_collision_diagnostic` 2263)
  among others — none reachable from `tests/*.rs` (a separate crate);
  `tests/cli.rs`'s own doc comment names subprocess testing's one
  legitimate reason (exit-code observability across the process
  boundary), which covers none of this. Deciding each function's right
  library home (compose.rs/graph.rs/engine.rs/a new entry point) is a
  design call, not a single mechanical move — sized like the standing
  READ-VERB-STRAND-COHESION precedent, not a hoist/prune entry. Filed as
  a refactor capture (`plan-main-thin-dispatch-cohesion.md`, observed
  285f57b) per the posture-sweep rule's routing ("needs a design decision
  → a `.flume/refactor/` capture"), not invented directly into pending.
- Queue: 37 pending — unchanged this tick (the one finding routed to a
  refactor capture, not pending). 1 pickable OPEN
  (PLACEMENT-MODULE-EXTRACTION), 31 chained blockedBy (all resolving to
  live tags), 5 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER, IMPORT-ROLLUP-WRITER-PLACEMENT,
  READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION). Open forks
  unchanged: (multi-harness-projection), (lazy-grounds), neither touched.
  Refactor captures: 1 live (filed this tick,
  plan-main-thin-dispatch-cohesion.md — awaits next tick's inbox job).
  Friction: 1 live (plan-architecture-debt-list-stale.md, filed last
  tick — awaits a human specs/ edit, untouched this tick). Inbox: 0
  notes.

Plan continues: yes — the refactor capture filed this tick makes job 1
(inbox/refactor drain) live for the next tick, ahead of the fresh posture
rotation the closed cycle otherwise hands to job 4.
