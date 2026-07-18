# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: 4e46eac — advanced from 73c76ca.
- Residue swept through: 4e46eac — advanced from 73c76ca.
- Posture swept through: fe3ff3f — verbs ticked last cycle and closed the
  rotation pass (foundation, model, formats, pipeline, judges, provider,
  verbs all covered). A fresh cycle opens next time the sweep re-arms.
- This tick: POST-SHIP RECONCILIATION — window 73c76ca..4e46eac (three
  build commits: 72daab3, 516f8f6, 8f96918), shipped as 4e46eac
  (DRIFT-INCLUDE-SOURCE-PATH-CWD-LEAK, BUILTIN-KIND-DEFINITIONS-RESULT-
  COLLAPSE, JSON-MANIFEST-TOP-LEVEL-OBJECT-PARSE-CONSOLIDATE). Audit: all
  three ship commits read on disk and verified to match their entries'
  acceptance (harness_relative canonicalizes both sides before
  strip_prefix; definitions() returns bare BTreeMap and KindError is
  deleted, zero remaining references grep-confirmed; DocumentMember::parse/
  Manifest::parse both route through the new parse_top_level_object). All
  three already dropped from pending.json by the ship commit. Metrics
  glanced (.flume/metrics.jsonl) — no bail/revert markers in the window.
  Three chained entries were blockedBy one of the three shipped tags —
  unblocked to `open`: KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH and
  MAIN-READ-FILE-UNIT-FORMAT-EXHAUSTIVE-MATCH (both blockedBy
  BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE), DRIFT-EMIT-LOCK-PARSE-HOIST
  (blockedBy DRIFT-INCLUDE-SOURCE-PATH-CWD-LEAK). All three re-verified
  file-disjoint (kind.rs, main.rs, drift.rs) so all three go live
  together. Their citations were re-verified against the shipped diffs
  and corrected where a shipped commit's line-count growth shifted them:
  DRIFT-EMIT-LOCK-PARSE-HOIST's five drift.rs fn cites shifted +13 (72daab3's
  harness_relative growth precedes them) — read_prior_provenance 1973→1986,
  walk_lock_rows 1916→1929, read_declarations 3407→3420, parse_declarations
  3430→3443, declarations_from_doc 3448→3461 — and MAIN-READ-FILE-UNIT-
  FORMAT-EXHAUSTIVE-MATCH's project_bytes cite shifted the same +13
  (2156-2170→2169-2183); KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH's kind.rs
  cites (945-1427) sit before 516f8f6's deletion at 1462, unaffected.
  Sweep: diffed the full window (`git diff --stat 73c76ca..HEAD`) against
  the three commits already read for the audit — no residue: KindError has
  zero remaining references tree-wide (rg-confirmed), the JSON top-level
  parse consolidation left no second implementation, harness_relative's
  fix is self-contained. Nothing new filed.
- Queue: 31 pending (-3 shipped, +0 filed) — 3 pickable OPEN
  (KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH, MAIN-READ-FILE-UNIT-FORMAT-
  EXHAUSTIVE-MATCH, DRIFT-EMIT-LOCK-PARSE-HOIST — pairwise file-disjoint,
  newly unblocked this tick), 26 chained blockedBy (all resolving to live
  tags), 2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched. Refactor
  captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: no — the post-ship reconciliation window is caught up
(Audited/Residue swept through 4e46eac, the current HEAD), no spec delta
past 2d66fc9, and the posture rotation stays closed until a forward window
touches a subsystem. Next wake re-arms on a spec delta, a new post-ship
window, or the posture rotation re-arming.
