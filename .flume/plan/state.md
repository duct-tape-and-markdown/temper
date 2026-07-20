# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: f17678f — advanced from cc28af8.
- Residue swept through: f17678f — advanced from cc28af8.
- Posture swept through: src/builtin.rs covered, mid-rotation — unchanged, not this tick's job. Frontier still holds every module past src/builtin.rs; src/builtin_kind.rs is the tree-order candidate next time posture sweep is live.
- This tick: POST-SHIP RECONCILIATION, window cc28af8..f17678f (a27f815, f17678f). Audit: BUILTIN-CONTRACT-DOC-KIND-LIST-STALE verified shipped on disk — builtin.rs's module header (a27f815) now reads "Every embedded kind's floor Contract is a lossless projection...", the fixed 5-kind parenthetical is gone, matching the finding exactly. f17678f is the ship marker, touching only pending.json (entry removal, 26 lines). cargo test and cargo clippy --all-targets -D warnings both green; metrics.jsonl shows the build tick (28 turns, small — matches a one-paragraph reword) merged with no reverts. Sweep: the window is the reword itself — no other file references the retired 5-kind list; no fresh residue.
- Queue: 2 pending, 0 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job; re-verified still holding, neither's cited files (src/graph.rs, tests/graph.rs, .github/workflows/release.yml) touched in this window). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep's rotation is still open (mid-rotation at src/builtin.rs) and no pickable entries exist to ship first, so it is next tick's job: the src/builtin_kind.rs neighborhood, tree order.
