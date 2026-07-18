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
- Posture swept through: b710f2d — rotation closes. This tick swept
  `verbs` (main.rs, install.rs, bundle.rs, lib.rs, test_support.rs);
  foundation/model/formats/pipeline/judges/provider/verbs all ticked
  this cycle. Next rotation reopens at `foundation` once a forward
  window (`git log b710f2d..HEAD -- src/ sdk/src/ tests/`) touches it —
  this tick's window now does (the three build ships touch
  json_manifest.rs/toml_document.rs/dial.rs/compose.rs), so the next
  tick with no higher-priority job live opens rotation fresh at
  `foundation`.
- This tick: POST-SHIP RECONCILIATION (audit + residue sweep) over
  d40a9f8..785abd7 — see cursor lines above for detail. One gate
  unblocked (FORMAT-READ-UTF8-DECODE-CONSOLIDATE), one friction capture
  filed (build-phase git-worktree commit-routing bug), zero pending
  entries needed dropping beyond what the ship commit already removed.
- Queue: 32 pending (35 − 3 shipped). 4 pickable OPEN
  (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  FORMAT-READ-UTF8-DECODE-CONSOLIDATE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION), 26 chained blockedBy, 2 parked on
  human action. Open forks: (multi-harness-projection), (lazy-grounds)
  unchanged. Refactor captures: 0 live. Friction: 1 live (filed this
  tick). Inbox empty. Disjointness re-checked: no two OPEN entries share
  a file.

Plan continues: yes — the posture rotation's forward window is no longer
empty (this tick's audited window touched src/), so the next tick with no
inbox/spec-delta/reconciliation work live opens a fresh rotation at
`foundation`.
