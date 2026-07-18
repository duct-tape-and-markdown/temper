# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: c1b0f51 — advanced from 4e46eac.
- Residue swept through: c1b0f51 — advanced from 4e46eac.
- Posture swept through: provider next (mid-rotation) — pipeline read and
  swept last tick (touched: drift.rs by 0282dc7/72daab3/112b188 since its
  own last full sweep) — 4 new findings, unchanged this tick.
- This tick: INBOX — drained the 4 refactor captures pipeline's posture
  sweep filed last tick. Each claim re-verified at HEAD (`git log
  7ac498a..HEAD -- src/import.rs src/drift.rs src/read.rs
  tests/check_cost.rs` empty — nothing shipped since filing, no claim
  narrowed or moved) before scoping. Two resolved decidably without
  inventing intent, filed `open`/`blockedBy`: IMPORT-READ-DIRS-VACUOUS-PIN-RETIRE
  (`rg 'fs::read_dir\b' src/` confirms zero call sites anywhere in the
  crate — the guarded site the counter was built for was cut at 0bc0ee9
  and never replaced, so `READ_DIRS`/`read_dir_count`/the `check_cost.rs`
  `read_dirs == 0` assertion are structurally unconstructable to nonzero;
  `engineering.md` "A green verdict is proven non-vacuous" makes retiring
  the dead pin decidable, not a design call — serialized behind
  IMPORT-DECLARED-GOVERNED-PATHS-HOIST, the chain's last import.rs
  toucher). Two are genuinely unruled and filed `parked` rather than
  invented: IMPORT-ROLLUP-WRITER-PLACEMENT (import.rs's roll-up writer
  contradicts the codemap's stated import="discovery"/drift="the lock"
  split the same way the four edges architecture.md's Invariants section
  already enumerates and rules do — but unlike those four, no human edit
  has named this fifth instance yet, and Growth rules makes that
  ratification human-only, not plan's to presume) and
  READ-CONTEXT-MEMBER-CITER-GRAIN / READ-VERB-STRAND-COHESION (grain and
  cohesion judgment calls a re-read of `read.rs` confirmed still turn on
  taste — leaf-vs-member address narration and the module's stated "one
  CLI verb" framing vs. Growth rules' flat-split default — neither
  decidable from the code alone). All 4 capture files deleted.
- Queue: 36 pending — 4 added this tick (1 blockedBy, 3 parked, none
  open). 1 pickable OPEN (PLACEMENT-MODULE-EXTRACTION), 30 chained
  blockedBy (all resolving to live tags), 5 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  IMPORT-ROLLUP-WRITER-PLACEMENT, READ-CONTEXT-MEMBER-CITER-GRAIN,
  READ-VERB-STRAND-COHESION). Open forks unchanged: (multi-harness-projection),
  (lazy-grounds), neither touched. Refactor captures: 0 live (drained this
  tick). Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — no live inbox/spec-delta/reconciliation input
remains, so the next tick resumes the posture sweep's own rotation at
provider (`src/builtin.rs`, `src/builtin_kind.rs`), already known
not-clean-skippable from last tick's forward check.
