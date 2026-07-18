# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: 870c52d — advanced from c1b0f51.
- Residue swept through: 870c52d — advanced from c1b0f51.
- Posture swept through: verbs next (mid-rotation) — pipeline read and
  swept this tick (touched: `drift.rs`, new `placement.rs` by 8704036
  since 285f57b) — quiet-on-clean, nothing new beyond the already-queued.
- This tick: POSTURE SWEEP — fresh cycle opened
  (`git log 285f57b..HEAD -- src/ sdk/src/ tests/`: one commit, 8704036,
  touching `drift.rs`, `install.rs`, `lib.rs`, new `placement.rs`).
  foundation/model/formats bulk-skipped (none of their modules in that
  commit's touch set). pipeline read and swept whole (`drift.rs`,
  `import.rs`, `read.rs`, `builtin_lock.rs`, `placement.rs` against every
  `engineering.md` lens plus cohesion/dead-plumbing) — quiet-on-clean:
  every duplicate/hoist pattern found (the `write_placement` inline copies
  in `emit_manifest`/`emit_one`, `walk_lock_rows` vs `read_lock_document`,
  `source_deps`/`read_declarations`'s independent lock reparses,
  `manifest_segment_reaps`'s independent manifest reread,
  `EmitOutcome::label`'s zero outside consumer, `declared_governed_paths`,
  the retired `READ_DIRS` counter, `DIRECTIVE_FIELD_LABEL`,
  the `by_kind` five-site scan) already resolves to a live queued tag
  (cross-checked against all 37 entries); `placement.rs` (new, first
  sweep) and `builtin_lock.rs` (never previously flagged) are both
  genuinely clean — every export has a real outside-module consumer, no
  leftover install.rs coupling, no dead paths. judges/provider
  bulk-skipped (untouched by 8704036). verbs (`install.rs`, `lib.rs`
  touched) is not clean-skippable — next tick reads it whole rather than
  skipping.
- Queue: 37 pending — unchanged; no entries added, no captures filed
  (pipeline swept quiet-on-clean, nothing new survived the cross-check).
  2 pickable OPEN (`EXTRACT-FOUNDATION-BOUNDARY-RESTORE`,
  `INSTALL-PROJECTION-MATCH-CONSOLIDATE`), 29 chained blockedBy (all
  resolving to live tags), 6 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER, IMPORT-ROLLUP-WRITER-PLACEMENT,
  READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION,
  MAIN-THIN-DISPATCH-COHESION). Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched. Refactor
  captures: 0 live. Friction: 1 live (plan-architecture-debt-list-stale.md,
  filed three ticks ago — awaits a human specs/ edit, untouched this
  tick). Inbox: 0 notes.

Plan continues: yes — the posture rotation is mid-cycle: pipeline swept
quiet this tick, judges/provider bulk-skipped behind it, and the next
tick reads verbs (`main.rs`, `install.rs`, `bundle.rs`, `lib.rs`,
`test_support.rs`) whole — 8704036 touched `install.rs`/`lib.rs`, so it
is not clean-skippable.
