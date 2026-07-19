# Plan state

- Spec derived through: b645125 — unchanged, not this tick's job.
- Audited through: f130ebc — unchanged; not this tick's job, but no longer current: 3533e0b (build, past this cursor) touched sdk/src/{contract,declarations}.ts + sdk/test/{contract,emit}.test.ts — a reconciliation window is now open for next tick.
- Residue swept through: f130ebc — unchanged, same window as above; not this tick's job.
- Posture swept through: 77c0790 — unchanged, rotation was already closed; not this tick's job.
- This tick: INBOX. Routed the one live note — "round-trip gate covers one row, not the class" (field residual, observed at 9a7d7d1, filed to inbox at 98a4cd3) — into a new pending entry, BUILTIN-LOCK-ROUND-TRIP-ALL-KIND-ROWS (open, per engineering.md "A green verdict is proven non-vacuous"). Re-verified live at HEAD 9d0868b: `tests/builtin_lock_frozen.rs`'s `the_sdk_derived_installed_plugin_kind_round_trips_through_the_engine_reader` (127-160) still `find`s and decodes exactly one of the 14 memberless-lock kind rows through `CustomKind::from_kind_fact_row`; unchanged since 55b8539 and untouched by any commit in 9a7d7d1..HEAD. Drained the note from inbox.md.
- Queue: 3 pending, 1 open, 0 blockedBy, 2 parked. Refactor: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — post-ship reconciliation is live for next tick: 3533e0b shipped past the Audited-through/Residue-swept-through cursor (f130ebc) touching sdk/src and sdk/test. That window (f130ebc..HEAD) is the next tick's job once picked up.
