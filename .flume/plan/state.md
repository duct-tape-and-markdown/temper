# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 4861fd3b — window bb27d6ab..4861fd3b (12ff1aaf, 6e94a6c9)
  audited; both entries' work verified on disk, matches acceptance.
- Residue swept through: 4861fd3b — same window; guard-driver residue
  class fully closed (only common::run_guard's definition and the
  unrelated `guard --help` case remain), no new residue found.
- Posture swept through: 39107255 — rotation closed: last frontier module
  sdk/src/builtins.ts (+ imports kind.ts/contract.ts/prose.ts) read, clean.
- This tick: INBOX — routed the relative-commondir defect report (observed
  b787061b) into TAP-WORKTREE-RELATIVE-COMMONDIR-JOIN (open); re-verified
  the claimed gap on disk and independently reproduced git's real relative
  commondir layout via a local `git worktree add` probe before scoping.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 4. Friction:
  2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: no — inbox drained, spec cursor current, reconciliation
window empty, posture rotation still closed; build picks up the one open
entry next.
