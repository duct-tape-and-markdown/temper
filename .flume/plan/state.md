# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: 785abd7 — advanced from d40a9f8. `git log
  d40a9f8..785abd7 -- src/ tests/ sdk/` carried four commits past the
  eight plan-only ones already covered: d577bdf, d65af7d, b4a2467 (the
  three build ticks) plus 785abd7 (their ship/removal commit). Verified
  on disk, each against its entry's acceptance: json_manifest.rs's
  `read_to_string` (309-318) now serves both `DocumentMember::read` and
  `Manifest::read` (JSON-MANIFEST-READ-DECODE-CONSOLIDATE); toml_document
  `parse` is `fn`, not `pub fn` (TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE);
  dial.rs's `severity_from_label` copy is gone, dial.rs calls
  `compose::severity_from_label` (now `pub(crate)`)
  (COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE). All three already dropped
  from pending.json by the ship commit. `cargo clippy --all-targets -D
  warnings` green at HEAD. Metrics glance surfaced two more
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE attempts in this window (both
  still open, unshipped): a 197-turn/795s bail
  (.flume/prior-attempts/builtin-kind-definition-result-collapse.json)
  whose own record names the cause a git-worktree commit-routing bug
  (three commit+reset cycles onto `main` instead of the worktree branch,
  reflog d9ebcbd/7f42ecd/33ee095), not a sizing problem — the
  already-split entry's own implementation verified clean
  (build/test/clippy/fmt) before the bail. Filed
  .flume/friction/build-worktree-commits-land-on-main-branch.md rather
  than re-splitting; the entry stays at its current scope. Stale-gate
  re-test: FORMAT-READ-UTF8-DECODE-CONSOLIDATE was `blockedBy
  JSON-MANIFEST-READ-DECODE-CONSOLIDATE`, which shipped in this window —
  unblocked to `open`, its json_manifest.rs file description reworded
  from conditional ("once X lands") to present fact, notes re-stamped at
  785abd7. No other pending entry referenced either of the other two
  shipped tags.
- Residue swept through: 785abd7 — same window. The three ships read
  clean against their own entries: no stale doc-comment, no leftover
  duplicate, no widened-visibility loose end (compose's
  `severity_from_label` narrowed to exactly the one new caller,
  dial.rs). No further residue found.
- Posture swept through: foundation next (mid-rotation) — fresh cycle
  reopened this tick per the prior note (foundation → model → formats →
  pipeline → judges → provider → verbs). `git log 3871eba..HEAD --
  src/check.rs src/extract.rs src/hash.rs src/address.rs src/tap.rs
  src/json_splice.rs` is empty — foundation's own last full sweep
  (3871eba, quiet) already covers this window, so per the posture-sweep
  rule ("on a subsystem untouched since its last sweep, skip forward")
  no re-read was needed. Quiet-on-clean, rotation advances alone to
  `model`.
- This tick: POSTURE SWEEP — job 4, foundation subsystem (rotation's
  first, reopened per last tick's note that the forward window was no
  longer empty). Verified the six foundation files untouched since
  their last full sweep (3871eba) rather than re-reading them; nothing
  in that window could have introduced a new finding. No new pending
  entry.
  Checked ahead for next tick: model (`kind`/`contract`/`compose`/
  `schema`/`roster`) is NOT clean-skippable — `git log 9e197d6..HEAD --
  src/kind.rs src/contract.rs src/compose.rs src/schema.rs
  src/roster.rs` shows b4a2467 (COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE,
  narrowed `compose::severity_from_label` to `pub(crate)` and gave it a
  second caller). Next tick reads model whole rather than skipping.
- Queue: 32 pending, unchanged — 4 pickable OPEN
  (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  FORMAT-READ-UTF8-DECODE-CONSOLIDATE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION — pairwise file-disjoint,
  re-verified this tick), 26 chained blockedBy (unchanged links, all
  still resolve to live tags), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — reasons
  unchanged, untouched this window). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. Refactor
  captures: 0 live. Friction: 1 live (build-worktree-commits-land-on-
  main-branch.md, filed last tick, unchanged). Inbox empty.
  Disjointness re-checked: no two OPEN entries share a file.

Plan continues: yes — posture sweep resumes at `model`
(`kind`/`contract`/`compose`/`schema`/`roster`), the roster's next
subsystem, not clean-skippable per the note above — the next tick
reads it whole.
