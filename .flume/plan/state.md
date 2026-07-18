# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: 870c52d — advanced from c1b0f51. No src/sdk/tests
  commits land past it (`git log 870c52d..HEAD -- src/ sdk/src/ tests/`
  empty — only plan commits since).
- Residue swept through: 870c52d — advanced from c1b0f51 (same empty
  window).
- Posture swept through: 1973522 — verbs ticked this tick and closes the
  rotation pass (foundation/model/formats bulk-skipped, pipeline 0 new
  findings, judges/provider bulk-skipped, verbs this tick — 0 new pending
  entries, 1 orphan routed to open-questions.md). All seven subsystems
  from architecture.md's codemap now covered for this cycle. A fresh
  cycle (foundation → model → formats → pipeline → judges → provider →
  verbs) opens next time the sweep re-arms.
- This tick: POSTURE SWEEP — verbs (`src/main.rs`, `src/install.rs`,
  `src/bundle.rs`, `src/lib.rs`, `src/test_support.rs`, plus new
  `src/placement.rs` read as part of the subsystem), not clean-skippable
  (8704036 touched `install.rs`/`lib.rs`, added `placement.rs`). Read
  every file whole against every `engineering.md` lens plus
  cohesion/dead-plumbing, cross-checked against all 37 queued entries.
  `main.rs` (2762 lines, `gate()`/`explain()`/~45 helpers): clean beyond
  what's already queued — every match exhaustive, no new duplicate
  cost-hoist pattern, `MAIN-THIN-DISPATCH-COHESION`'s cohesion finding
  already parked. `install.rs`/`bundle.rs`: every prior finding
  (`INSTALL-PROJECTION-MATCH-CONSOLIDATE`,
  `INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE`, `INSTALL-PLACEMENT-KIND-ENUM`,
  `BUNDLE-INSTALL-SESSION-START-SHAPE-CONSOLIDATE`) re-verified still
  true and unmoved beyond line drift. `lib.rs`/`test_support.rs`: clean,
  no findings. One new defect: 8704036's extraction moved
  `placement_lines`/`is_placement_comment` to `placement.rs` but left
  their doc comment orphaned in `install.rs` (1640-1646), now glued —
  no blank line — onto `render`'s own doc comment (1647), reading as
  render's opening paragraph though it describes a deleted function.
  Comment-only, no gate impact — per the standing ride-only rule
  (open-questions.md, "One stale cite, ride-only, never an entry") this
  is routed as a fourth live orphan there, not a standalone pending
  entry: none of install.rs's four chained entries touches that line
  range or names the docblock today, so it rides whichever one first
  does. Also corrected that section's stale claim that
  `PLACEMENT-MODULE-EXTRACTION` still chains onto `drift.rs`'s
  `RawLockRow` orphan — it shipped (870c52d) without touching that
  docblock, so the remaining chain is `EXTRACT-FOUNDATION-BOUNDARY-
  RESTORE`/`DRIFT-SOURCE-DEP-PARSE-HOIST` only. `placement.rs` itself
  (new module, first sweep): clean — both markers and the recognizer
  have real cross-module consumers (`install.rs`, `drift.rs`), the
  moved test travelled intact.
- Queue: 37 pending — unchanged; no entries added, none rewritten (the
  tick's one finding is comment-only and routes to open-questions.md's
  ride-only orphan list, never a pending entry, per that section's own
  rule). 2 pickable OPEN (`EXTRACT-FOUNDATION-BOUNDARY-RESTORE`,
  `INSTALL-PROJECTION-MATCH-CONSOLIDATE`), 29 chained blockedBy (all
  resolving to live tags), 6 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER, IMPORT-ROLLUP-WRITER-PLACEMENT,
  READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION,
  MAIN-THIN-DISPATCH-COHESION). Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched. Refactor
  captures: 0 live. Friction: 1 live (plan-architecture-debt-list-stale.md,
  filed four ticks ago — awaits a human specs/ edit, untouched this
  tick). Inbox: 0 notes.

Plan continues: no — the posture rotation just closed its cycle with a
comment-only finding (no pending entry, no refactor capture), the spec
delta is fully routed, and post-ship reconciliation's window is empty.
Nothing below this tick's job is live; the loop hibernates until a
future commit touches src/, sdk/src/, or tests/ (re-arming the posture
sweep) or the inbox/spec-delta/reconciliation cursors have new ground to
cover. 2 pickable OPEN entries remain for build to take up meanwhile.
