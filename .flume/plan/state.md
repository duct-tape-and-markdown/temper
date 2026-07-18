# Plan state

- Spec derived through: 64828d9 — unchanged, not this tick's job.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: judges next (mid-rotation) — pipeline read and
  swept this tick.
- This tick: POSTURE SWEEP. Inbox and spec delta both drained (empty,
  re-verified this tick: `git log 64828d9..HEAD -- specs/` and the
  inbox file both empty); audit/residue cursors current (`git log
  64828d9..HEAD -- src/ sdk/src/ tests/` empty — every commit since is
  a `plan:` commit, none touching code). Rotation continues the cycle
  4d1c261 opened (roster from architecture.md's codemap: foundation,
  model, formats, pipeline, judges, provider, verbs; foundation,
  model, and formats swept the prior three ticks). `pipeline` was
  already known-touched by 404b73a (drift.rs gained
  `nested_members_from_rows`/`embedded_member_from_row`, moved in from
  extract.rs, plus two call sites requalified from `crate::extract::`
  to `crate::json_manifest::`), so it is this tick's one
  read-and-swept subsystem: all five modules (drift, import, read,
  builtin_lock, placement — 7791 lines total) read in full against
  every engineering.md section and architecture.md's Invariants
  section. Quiet on clean: every candidate traced back either matched
  an already-queued tag (the `by_kind`-rescan shape in read.rs under
  READ-BY-KIND-INDEX-HOIST; the lock re-read/re-parse shapes in
  drift.rs under the four DRIFT-*-PARSE-HOIST/DRIFT-MANIFEST-SEGMENT-
  REAP-READ-HOIST entries; the `write_placement` duplication under
  DRIFT-WRITE-PLACEMENT-CONSOLIDATE), or resolved to a real consumer
  on grep-verification (`import::NestedFileUnit`,
  `discover_nested_file`/`discover_kind_files`/`Discovery`/
  `LocalOverride`, `drift::read_layout_document`/`LayoutDocumentRows`/
  `layout_edge_fields`/`project_bytes`/`EmitOwnedEntry`/
  `emit_owned_targets`/`lock_read_count`/`lock_parse_count`,
  `placement::NOTE_MARKER`/`BANNER_MARKER`/`MODELINE_MARKER` — all
  live outside their own module), or read as a correct closed-
  vocabulary decode rather than a wildcard over a growing Rust enum
  (drift.rs's `verifier_from_table` `_ => Err(...)` decodes a string
  column, the same "unknown value is an error" shape the row family
  uses throughout — not a shared-concept seam). One candidate
  considered and explicitly left alone: `read::Citation`'s always-
  empty construction at main.rs:604 is a stated, doc-commented forward
  scaffold ("the floor carries no producer yet ... the mechanism is
  proven in unit tests here", read.rs:100-103) — an acknowledged gap,
  not silent residue. No cohesion split, no dead plumbing beyond that
  acknowledged scaffold, no new stale cite in these five files (the
  six already-tracked stale cites all live elsewhere — json_splice.rs,
  document.rs, install.rs, extract.rs, json_manifest.rs). `judges`
  (engine.rs, graph.rs, dial.rs, coverage.rs, coverage_note.rs,
  display.rs, reporter.rs) is next in roster order and was NOT touched
  by 404b73a; its forward window since its own last full sweep
  (fe3ff3f) is also empty (`git log fe3ff3f..HEAD -- src/engine.rs
  src/graph.rs src/dial.rs src/coverage.rs src/coverage_note.rs
  src/display.rs src/reporter.rs` empty) — untouched, so next tick
  skips it forward in bulk per the posture-sweep rule and lands on
  `provider` (builtin_kind.rs), already known-touched by 404b73a, as
  the tick's actual read-and-swept subsystem.
- Queue: 38 pending (unchanged this tick — quiet). 7 pickable OPEN
  (DRIFT-SOURCE-DEP-PARSE-HOIST, INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE,
  KIND-ZERO-CONSUMER-EXPORTS-PRUNE, IMPORT-ROLLUP-WRITER-PLACEMENT,
  READ-CONTEXT-MEMBER-CITER-GRAIN, READ-VERB-STRAND-COHESION,
  MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT), 28 chained blockedBy, 3 parked
  on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  MAIN-JUDGE-VERB-HOME-RULING). Open forks unchanged: (multi-harness-
  projection), (lazy-grounds), neither touched. Refactor captures: 0
  live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture-sweep rotation is mid-cycle
(`judges` next, itself untouched since fe3ff3f and skip-forwarded in
bulk, landing on `provider` as next tick's actual read-and-swept
subsystem, already known-touched by 404b73a) so next tick's job is
live without a fresh forward-window check beyond what this tick
already ran.
