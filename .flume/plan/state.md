# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: c1b0f51 — advanced from 4e46eac.
- Residue swept through: c1b0f51 — advanced from 4e46eac.
- Posture swept through: provider next (mid-rotation) — pipeline read and
  swept this tick (touched: drift.rs by 0282dc7/72daab3/112b188 since its
  own last full sweep) — 4 new findings.
- This tick: POSTURE SWEEP — pipeline subsystem (`src/drift.rs`,
  `src/import.rs`, `src/read.rs`, `src/builtin_lock.rs`; `placement` not
  yet shipped), the rotation's second full pass over this subsystem. All
  four files read whole against every `specs/process/engineering.md`
  section plus the sweep's cohesion/dead-plumbing lenses, cross-checked
  against the 10 pending entries already chained onto these files from the
  rotation's first pass to avoid re-filing. Zero-consumer and
  exhaustive-match checks came back clean beyond what those existing
  entries already cover (`builtin_lock.rs` has no queued entry and is
  itself clean — single job, no dead paths). Filed 4 new pending entries,
  all cost-scale-hoist shaped (`engineering.md`, "Cost scale is hoisted,
  and pinned by count"), each serialized behind the nearest existing
  entry sharing its file: DRIFT-CONFIG-STALE-LOCK-PARSE-HOIST
  (`config_stale`'s own `walk_lock_rows` re-parse of `lock.toml` inside
  `gate()`, uncovered by DRIFT-SOURCE-DEP-PARSE-HOIST's narrower
  `source_deps` scope; folds in `walk_lock_rows` duplicating
  `read_lock_document`'s read+parse), DRIFT-MANIFEST-SEGMENT-REAP-READ-HOIST
  (`manifest_segment_reaps`/`emit_manifest` each re-read the same
  represented-manifest file within one `emit()` pass),
  IMPORT-DECLARED-GOVERNED-PATHS-HOIST (`declared_governed_paths`
  recomputed identically per nested-file kind instead of once per run),
  READ-BY-KIND-INDEX-HOIST (`explain()`'s member path scans the whole
  `by_kind` corpus independently at 5+ sites — `resolve`/`why`/
  `count_satisfiers`/`narrate_satisfied`/`field` — instead of one shared
  index). Filed 4 refactor captures for design-decision-shaped findings
  the sweep cannot resolve unilaterally: `plan-import-read-dirs-vacuous-pin`
  (`READ_DIRS` count-pin never incremented anywhere — the
  `check_cost.rs` assertion it backs is structurally vacuous,
  `engineering.md` "A green verdict is proven non-vacuous"),
  `plan-import-write-rollup-placement` (the lock roll-up writer
  — `RollupEntry`/`write_rollup`/`rollup_tables` — lives in `import.rs`
  though architecture.md's codemap and drift.rs's sole-caller status both
  point at `drift.rs`; needs a page amendment or a ruling that import's
  header-stated second job stands), `plan-read-context-member-citer-grain`
  (`context_member_one` re-implements `narrate_citers`' filter-and-narrate
  shape at a different grain — same job or genuinely distinct is a design
  call), `plan-read-verb-strand-cohesion` (read.rs's five dispatch strands,
  particularly the telemetry-only `field` strand, against architecture.md's
  own flat-split growth rule vs. the module's stated "one CLI verb"
  framing). Checked ahead: judges is clean-skippable (`git log
  fe3ff3f..HEAD -- src/engine.rs src/graph.rs src/dial.rs src/coverage.rs
  src/coverage_note.rs src/display.rs src/reporter.rs` empty) — skipped
  forward in bulk this tick per the posture-sweep rule. provider is not
  clean-skippable (`git log fe3ff3f..HEAD -- src/builtin.rs
  src/builtin_kind.rs` shows 516f8f6 touched builtin_kind.rs since its own
  last full sweep) — next tick reads it whole rather than skipping.
- Queue: 32 pending — 4 added this tick (all `blockedBy`, none open).
  1 pickable OPEN (PLACEMENT-MODULE-EXTRACTION), 29 chained blockedBy (all
  resolving to live tags), 2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched. Refactor
  captures: 4 live (filed this tick, listed above). Friction: 0 live.
  Inbox: 0 notes.

Plan continues: yes — 4 live refactor captures now sit in `.flume/refactor/`
for the next tick's inbox job to verify and drain into pending entries (or
retire if HEAD has moved past the claim), taking priority over the posture
sweep's own continuation to provider.
