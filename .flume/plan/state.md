# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8 — unchanged; `git log d40a9f8..HEAD -- src/
  tests/ sdk/` is empty (all eight commits since — 5af93d9, 3871eba,
  9e197d6, 7a5f86c, 69e7571, 662cf07, 5d7e712, b710f2d — are plan-only).
- Residue swept through: d40a9f8 — unchanged, same empty window.
- Posture swept through: b710f2d — rotation closes. This tick swept
  `verbs` (main.rs, install.rs, bundle.rs, lib.rs, test_support.rs);
  foundation/model/formats/pipeline/judges/provider/verbs all ticked
  this cycle. Next rotation reopens at `foundation` once a forward
  window (`git log b710f2d..HEAD -- src/ sdk/src/ tests/`) touches it.
- This tick: POSTURE SWEEP — verbs subsystem, all five files read
  whole/targeted against every section of `specs/process/
  engineering.md`, cross-checked against the queue to avoid
  re-flagging in-flight work. One new finding filed:
  BUNDLE-INSTALL-SESSION-START-SHAPE-CONSOLIDATE — bundle.rs carries a
  private `SESSION_START_COMMAND` (63) byte-identical to install.rs's
  `pub` one (88), and `hooks_manifest` (284-296) re-inlines the exact
  `SessionStart` hook-group JSON shape `install::session_start_group()`
  (1114-1116) already builds — verified on disk. One candidate
  discarded as already covered: `gate()`'s second
  `builtin_kind::definitions()` call at main.rs:1193 is already named
  a "secondary, same functions" fix inside the open
  GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST entry — not refiled. lib.rs and
  test_support.rs swept clean (module wiring and fixture constructors
  only; `test_support::tmpdir` vs `tests/common::tmpdir` is the
  rust.md-sanctioned cross-boundary duplication, not residue). No `_`
  wildcard arm over a shared enum, no hand-rolled crate mechanic, no
  zero-consumer pub export beyond the already-tagged
  INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE, no unconstructable enum
  variant found in any of the five files.
- Queue: 35 pending (34 + this tick's 1). 6 pickable OPEN, 27 chained
  blockedBy, 2 parked on human action. Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. Refactor
  captures: 0 live. Inbox empty.

Plan continues: no — the posture rotation just closed and its forward
window is empty (nothing between b710f2d and HEAD touches src/,
sdk/src/, or tests/); no other job (inbox, spec delta, reconciliation)
is live this tick.
