# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 385cf3e — window c9d8b24..385cf3e reconciled.
- Residue swept through: 385cf3e — same window.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs, src/compose.rs, src/contract.rs, src/coverage.rs, src/coverage_note.rs, src/dial.rs, src/display.rs, src/document.rs, src/drift.rs, src/engine.rs covered, mid-rotation. src/extract.rs is the tree-order candidate next — unchanged, not this tick's job.
- This tick: POST-SHIP RECONCILIATION, window c9d8b24..385cf3e (bdfc5d6 the only src/tests/sdk-touching commit). Audit: ENGINE-DECIDE-TYPE-ENUM-ARMS-GUARD-HELPER-REUSE verified shipped on disk — `Predicate::Type`'s decide arm (935) and `Predicate::Enum`'s (974) both bind `pred @ Predicate::{Type,Enum}` and call `guard_membership_fails(pred, value)` for the boolean, diagnostic formatting untouched; the two pinned tests (`type_fires_on_a_kind_mismatch_and_is_silent_on_match_and_absence` 1458, `enum_fires_off_the_permitted_set` 2267) pass (`cargo test --lib`); entry correctly dropped from pending.json by the ship commit, and this time the ship message's claim matches disk. Sweep: all three `guard_membership_fails` call sites (936, 974, 1237) are the one home now — no third hand-write survives; no fresh residue.
- Queue: 2 pending, 0 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched this tick). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — no pickable entry remains (both pending entries are human-parked), so the open posture rotation (next neighborhood src/extract.rs) is plan's own to drive next tick.
