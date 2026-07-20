# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: b9233dd — window f88e96d..b9233dd audited: the one src-touching commit in it, e2c4d1a (build: delete entry_shape_to_label), verified on disk against KIND-ENTRY-SHAPE-LABEL-PRUNE's own acceptance — `grep -rn entry_shape_to_label src/ tests/ sdk/ specs/` returns empty (deleted clean, no dangling `#[allow(dead_code)]`), `entry_shape_from_label` (908-946) and its live consumer `collection_address_from_row` (878-882) untouched, `cargo build` and `cargo clippy --all-targets -- -D warnings` both green. pending.json unchanged (2 entries, both parked; KIND-ENTRY-SHAPE-LABEL-PRUNE already drained by the b9233dd ship commit).
- Residue swept through: b9233dd — same window. No retirement or demolition named beyond the entry shipped; no new zero-consumer export introduced. Both parked entries re-tested: IMPORT-HOP-CAP-CITE (`git diff f88e96d..HEAD -- src/graph.rs tests/graph.rs` empty, park holds) and PACKAGING-CHANNELS-REMAINDER (`git diff f88e96d..HEAD -- .github/` empty, park holds). open-questions.md's only kind.rs mention (the fixture-string exclusion list, line 214) is unrelated to the deleted function — no orphan cite shifted by this window.
- Posture swept through: src/builtin.rs..src/json_splice.rs (prior rotation), plus src/kind.rs covered — mid-rotation, unchanged this tick. src/layout.rs is the tree-order candidate next.
- This tick: POST-SHIP RECONCILIATION, window f88e96d..b9233dd — audit + sweep both closed clean, cursors advanced to b9233dd, no new entries filed.
- Queue: 2 pending, 0 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched this tick, both re-tested and holding). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep rotation is still open (next candidate src/layout.rs) with no pickable entries in queue; it resumes next tick.
