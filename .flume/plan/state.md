# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: 870c52d — advanced from c1b0f51.
- Residue swept through: 870c52d — advanced from c1b0f51.
- Posture swept through: 285f57b — verbs ticked, closing the rotation pass
  (foundation bulk-skipped, model quiet-on-clean, formats quiet-on-clean,
  pipeline 4 findings, judges bulk-skipped, provider 1 finding, verbs 1
  refactor capture). All seven subsystems from architecture.md's codemap
  covered once; a fresh cycle opens next time the sweep re-arms.
- This tick: POST-SHIP RECONCILIATION — window c1b0f51..870c52d
  (`git log c1b0f51..HEAD -- src/ tests/ sdk/`: one commit, 8704036 "build:
  extract placement module from install.rs", shipped by 870c52d). Audit:
  verified on disk — `src/placement.rs` now carries the three marker
  constants, `placement_lines`, `is_placement_comment`, and the moved
  round-trip test exactly as `PLACEMENT-MODULE-EXTRACTION` specified;
  `src/install.rs` imports the markers from `crate::placement` (no local
  redefinition); `src/drift.rs`'s `emit_one` calls
  `crate::placement::placement_lines` and carries no `use crate::install`
  — the drift→install edge is dissolved. The ship commit (870c52d) already
  removed the entry from `pending.json`. Re-tested the two entries still
  reading `blockedBy: PLACEMENT-MODULE-EXTRACTION`
  (`EXTRACT-FOUNDATION-BOUNDARY-RESTORE`, `INSTALL-PROJECTION-MATCH-CONSOLIDATE`)
  — both blocks were pure shared-file serialization, never a functional
  dependency (their own notes said so); the blocker shipped, so both flip
  to `open` this tick. Re-verified their file cites against HEAD before
  flipping: `EXTRACT-FOUNDATION-BOUNDARY-RESTORE`'s primary file,
  `src/extract.rs`, is untouched since its `f404e48` scoping (every cited
  line still exact); its five sibling-file cites (`drift.rs`,
  `json_manifest.rs`, `builtin_kind.rs`, `kind.rs`, `main.rs`) drifted a
  handful of lines from intervening ticks and are corrected in place.
  `INSTALL-PROJECTION-MATCH-CONSOLIDATE`'s `src/install.rs` cites shifted
  by the placement extraction itself (744→727, 783→766) and are corrected;
  its grep-verify picked up a new incidental hit at `drift.rs:691`
  (`normalize_lock_path`, single-path canonicalization, not a
  match-against-a-target-list job) — confirmed unrelated and noted. Sweep:
  no new residue in this one-commit window — the extraction is
  location-only as its own acceptance bar required, and left no stray
  symbol or duplicate surface behind. Both cursors advance to 870c52d
  (HEAD); the window is fully reconciled in one motion, no split needed.
- Queue: 37 pending — unchanged count; two entries' gates flipped
  `blockedBy PLACEMENT-MODULE-EXTRACTION` → `open`
  (`EXTRACT-FOUNDATION-BOUNDARY-RESTORE`, `INSTALL-PROJECTION-MATCH-CONSOLIDATE`,
  disjoint file sets — both pickable together). 2 pickable OPEN, 29 chained
  blockedBy (all resolving to live tags), 6 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  IMPORT-ROLLUP-WRITER-PLACEMENT, READ-CONTEXT-MEMBER-CITER-GRAIN,
  READ-VERB-STRAND-COHESION, MAIN-THIN-DISPATCH-COHESION). Open forks
  unchanged: (multi-harness-projection), (lazy-grounds), neither touched.
  Refactor captures: 0 live. Friction: 1 live
  (plan-architecture-debt-list-stale.md, filed three ticks ago — awaits a
  human specs/ edit, untouched this tick). Inbox: 0 notes.

Plan continues: yes — the posture rotation's forward window re-arms:
`Posture swept through: 285f57b` now names a commit behind HEAD whose
forward window (`git log 285f57b..HEAD -- src/`) touched a subsystem
(8704036's placement extraction), so the next tick opens a fresh
foundation→...→verbs cycle starting at foundation.
