# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 3eb13268 — advanced; window 729b0cad..3eb13268 reconciled.
- Residue swept through: 3eb13268 — advanced; same window, clean.
- Posture swept through: 97d0241 — still mid-rotation, frontier unchanged
  (d2d89c94 touched src/read.rs, already inside it — no new module).
  Frontier remaining: src/read.rs, src/install.rs, sdk/src/builtins.ts.
- This tick: POST-SHIP RECONCILIATION — audited+swept 729b0cad..3eb13268.
  d2d89c94 (READ-EXPLAIN-DEFAULT-WORKSPACE-DERIVE build) matches:
  `explain_target` now derives its workspace via
  `format!("./{}", crate::WORKSPACE_DIR)`, matching main.rs's
  `DEFAULT_WORKSPACE` `LazyLock` pattern, instead of the hand-spelled
  `const DEFAULT_WORKSPACE: &str = "./.temper"`. Entry was already drained
  from pending.json by its own 3eb13268 ship commit — nothing left to
  drop. rg repo-wide for `"./.temper"`/`".temper"`: survivors are test-
  fixture path constructions and builtin_lock.toml's `governs_root` /
  dial.ts's locus `root` (both the kind's own declared data, not the
  fixed derived-state duplicate) — no residue. Verified live: fmt --check
  clean, clippy -D warnings clean, cargo test all green, pnpm sdk test
  141/141.
- Queue: 3 pending — 0 open, 1 parked, 2 deferred. Open forks: 2, unchanged.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — 0 open entries, so posture sweep has nothing pickable
to hand off to; plan takes the next tick to continue the rotation
(src/read.rs, src/install.rs, sdk/src/builtins.ts remain).
