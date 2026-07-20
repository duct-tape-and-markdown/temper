# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: b829216 — unchanged, not this tick's job.
- Residue swept through: b829216 — unchanged, not this tick's job.
- Posture swept through: 6f74784, mid-rotation — unchanged, not this tick's job. Frontier still holds every module past src/address.rs and src/admissibility.rs; src/builtin.rs is the tree-order candidate next time posture sweep is live.
- This tick: INBOX. `.flume/refactor/plan-overlay-builtin-kind-dedup.md` (filed 17a1eb5, observed 29c5baf) was the sole live capture; re-verified at HEAD (`git log 29c5baf..HEAD -- src/ sdk/ tests/` empty, every cited line re-read and confirmed live). Drained into two entries per the capture's own suggested split: RESOLVE-KIND-UNITS-DEAD-OVERLAY (open — the capture's #4, resolve_kind_units's dead internal re-overlay on an already-overlaid kind, mechanical and low-risk) and OVERLAY-BUILTIN-KIND-SHARED-HOIST (blockedBy the first — the capture's #1/2/3/5/6, the wider once-per-run hoist through LockFamily/gate.rs/admissibility.rs/read.rs, scoped from the capture's own design sketch). Serialized rather than both `open`: both reshape the same resolve_kind_units/overlay_builtin_kind neighborhood in src/compose.rs (pending-entry rule). Capture file deleted.
- Queue: 4 pending, 1 open, 1 blockedBy, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — RESOLVE-KIND-UNITS-DEAD-OVERLAY is pickable now; the only remaining live input is the posture sweep's still-open rotation, which resumes once the wave hands back.
