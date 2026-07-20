# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 83a3cd9 — unchanged, not this tick's job (61fb5b9 is a harness-only chore commit, touched no src/tests/sdk).
- Residue swept through: 83a3cd9 — unchanged, not this tick's job.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs, src/compose.rs covered, mid-rotation — advanced from src/check.rs. src/contract.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, src/compose.rs (+ its immediate imports read for context). Two findings filed: COMPOSE-DOC-HEADER-ERA-NARRATION-CUT (module header narrates TEMPER-TOML-ZERO's retirement, same class just fixed in check.rs) and COMPOSE-ZERO-CONSUMER-VISIBILITY-NARROW (`declared_kinds_with_overlaid`/`manifest_units` are `pub` with zero external callers, rg-verified crate-wide — the same lens that already retired `compose::declared_kinds`, 83d16ed). Considered and cleared: the module's combined domain-types/kind-resolution/lock-assembly scope is not a cohesion violation (its own header states the combination deliberately); the two invocation-count thread-locals have live test consumers in main.rs; no embedded-provider-literal or dead-plumbing violation found.
- Queue: 4 pending, 2 open (the two new entries), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — the posture sweep is the only remaining live job and pickable entries now exist, so ready work ships first; the sweep resumes at src/contract.rs when the wave hands back.
