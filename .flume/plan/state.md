# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 729b0cad — unchanged, no src/tests/sdk commits past it.
- Residue swept through: 729b0cad — unchanged, same.
- Posture swept through: 97d0241 — mid-rotation: src/main.rs covered
  this tick. Frontier remaining: src/read.rs, src/install.rs,
  sdk/src/builtins.ts.
- This tick: POSTURE SWEEP — neighborhood src/main.rs (+ immediate imports
  builtin_kind, bundle, check, compose, drift, gate, install, kind, read,
  reporter, schema, tap) read against engineering.md + architecture.md.
  Finding: `read.rs::explain_target`'s local `const DEFAULT_WORKSPACE: &str
  = "./.temper"` hand-respells `WORKSPACE_DIR` instead of deriving it —
  the exact shape main.rs's own `DEFAULT_WORKSPACE` `LazyLock` (and its
  doc comment) was built to avoid. Filed
  READ-EXPLAIN-DEFAULT-WORKSPACE-DERIVE (per: engineering.md, "Derived
  state is computed, never stored beside its source"). No embedded-
  provider-knowledge literal in main.rs itself (grep clean for hook/kind/
  path literals). `HarnessPath` (3 variants) and `Reporter` (4 variants)
  are both exhaustively matched. `join_locus`'s two call sites (main.rs's
  `manifest_path`, drift.rs's `manifest_target_path`) adapt two distinct
  row shapes (`CustomKind` vs `KindFactRow`) onto the one shared
  primitive — legitimate glue, not duplicate logic.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 2, unchanged.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — READ-EXPLAIN-DEFAULT-WORKSPACE-DERIVE is
pickable; the posture rotation stays open (src/read.rs, src/install.rs,
sdk/src/builtins.ts remain) and resumes once the wave hands back.
