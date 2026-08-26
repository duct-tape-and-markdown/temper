# Plan state

- Spec derived through: 13455d4a — unchanged, copied forward (not this tick's job).
- Audited through: 9b967326 — advanced from 2fa5efd7. Verified on disk: normalize_instructions_loaded_identity (src/telemetry.rs) now looks up the lock's path_to_id map (built in src/read.rs's explain_target via drift::emit_owned_targets) before falling back to the bare-stem heuristic; field/requirement_field/explain/requirement_detail thread the map through exactly as the entry's files[].description specified. Regression test added (tests/read_verbs.rs) passes (`cargo test --test read_verbs a_nested_instructions_loaded_record_joins_to_its_placement_folded_member_id`: ok); 29/30 other read_verbs tests pass, the one failure (sdk-round-trip test) is a pre-existing environment gap (sdk/dist unbuilt in this worktree), unrelated to this fix. Dropped TELEMETRY-INSTRUCTIONS-LOADED-NESTED-JOIN from the queue.
- Residue swept through: 9b967326 — advanced, same window (2fa5efd7..9b967326). No retirement/demolition named in either commit body; 3237033f only added a refactor-capture file (already drained last tick, not a src/tests/sdk touch) and carries no residue of its own.
- Posture swept through: 22f8064c — unchanged, still mid-rotation.
- This tick: post-ship reconciliation — audit + sweep on window 2fa5efd7..9b967326 (the only src/tests/sdk-touching commit since the last audit, 9b967326). Audit: TELEMETRY-INSTRUCTIONS-LOADED-NESTED-JOIN confirmed shipped on disk, dropped. Sweep: nothing to file.
- Queue: 4 pending — 1 open (COVERAGE-EMBEDDED-COUNT-MARKER), 1 parked, 2 deferred. Open forks: 3, unchanged. Friction: 0. Amendments: 0. Inbox: 0. Refactor: 0.

Plan continues: after-build — the only remaining live job is the posture sweep (forward window 22f8064c..HEAD touches src/, tests/, non-empty) and a pickable entry exists (COVERAGE-EMBEDDED-COUNT-MARKER, open); the sweep resumes once the wave hands back.
