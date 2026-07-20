# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 92960fd — advanced from b829216.
- Residue swept through: 92960fd — advanced from b829216.
- Posture swept through: 6f74784, mid-rotation — unchanged, not this tick's job. Frontier still holds every module past src/address.rs and src/admissibility.rs; src/builtin.rs is the tree-order candidate next time posture sweep is live.
- This tick: POST-SHIP RECONCILIATION, window b829216..92960fd (8ec62d0, d108fbf). Audit: both RESOLVE-KIND-UNITS-DEAD-OVERLAY and OVERLAY-BUILTIN-KIND-SHARED-HOIST verified shipped on disk — resolve_kind_units's dead re-overlay replaced with `kind.clone()`, LockFamily now carries `overlaid_builtin_kinds` computed once in assemble_lock_family and threaded through gate.rs/admissibility.rs/read.rs/build_manifest_cache; only remaining bare `overlay_builtin_kind` call site is main.rs's `guarded_manifests` (the guard path, out of the entries' gate()/explain() scope — legitimate). `overlay_builtin_kind_count` pin (main.rs:771-798) asserts `0 < overlays <= builtin_count`, non-vacuous. cargo test, cargo clippy -D warnings both green; metrics.jsonl shows both ticks merged, no reverts. Sweep: the hoist's own call-site migration (declared_kinds → declared_kinds_with_overlaid) left the original `declared_kinds` (compose.rs:538-551) with zero callers anywhere in src/, tests/, sdk/ (`rg` confirmed) — filed DECLARED-KINDS-ZERO-CONSUMER per engineering.md "An export earns its consumer".
- Queue: 3 pending, 1 open (DECLARED-KINDS-ZERO-CONSUMER), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — DECLARED-KINDS-ZERO-CONSUMER is pickable now; the only remaining live input is the posture sweep's still-open rotation, which resumes once the wave hands back.
