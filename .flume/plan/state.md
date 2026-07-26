# Plan state

- Spec derived through: 53e6f615 — 0046 routed: two Consequences already
  shipped in-commit (verified on disk), the third filed as an entry.
- Audited through: f25ce1ad — window 4861fd3b..f25ce1ad (b9ea1a2f) audited;
  TAP-WORKTREE-RELATIVE-COMMONDIR-JOIN's work verified on disk (src/tap.rs's
  gitdir/commondir joins go through the shared `crate::path::normalize_path`,
  no hand-rolled duplicate), test re-run green
  (log_path_resolves_linked_worktree_to_primary_checkout, now driven by a
  real `git worktree add` fixture), entry already absent from pending.json
  (dropped on ship). metrics.jsonl: clean ship, no revert.
- Residue swept through: f25ce1ad — same window; grepped `gitdir|commondir`
  across src/ and tests/ outside tap.rs/tests/tap.rs — no other site carries
  the old absolute-path assumption, no residue found.
- Posture swept through: 39107255 — rotation closed: last frontier module
  sdk/src/builtins.ts (+ imports kind.ts/contract.ts/prose.ts) read, clean.
- This tick: SPEC DELTA — derived 0046 into TELEMETRY-FIELD-STATES-
  DECLARED-ABSENCE (open).
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3
  (`(advisory-report-channel)` resolved, dropped by 0046). Friction: 2,
  unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — spec cursor now current and reconciliation
window empty, but 39107255..HEAD (b9ea1a2f) touched src/, reopening the
posture rotation; a pickable open entry exists, so build takes the wave
first and the sweep resumes when it hands back.
