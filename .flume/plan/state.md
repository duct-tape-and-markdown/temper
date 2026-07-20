# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 2a6e488 — window 385cf3e..2a6e488 reconciled.
- Residue swept through: 2a6e488 — same window.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs, src/compose.rs, src/contract.rs, src/coverage.rs, src/coverage_note.rs, src/dial.rs, src/display.rs, src/document.rs, src/drift.rs, src/engine.rs, src/extract.rs, src/frontmatter.rs, src/gate.rs covered, mid-rotation. src/glob.rs is the tree-order candidate next — unchanged, not this tick's job.
- This tick: POSTURE SWEEP, neighborhood src/gate.rs (immediate imports — admissibility/builtin_kind/check/compose/contract/coverage/dial/drift/engine/extract/graph/import/install/kind/roster — read for context, not re-swept). One finding: the built-in/custom/embedded kind-dispatch loops (117-153/188-211/265-279) each hand-copy the identical `with_joined_clauses`→dial→`engine::admissibility`→`engine::validate` "two-greens" sequence the code's own comments already name — filed GATE-TWO-GREENS-DISPATCH-DEDUP against engineering.md's "One job, one home". Rotation continues to src/glob.rs next tick.
- Queue: 3 pending, 1 open (GATE-TWO-GREENS-DISPATCH-DEDUP), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched this tick). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — GATE-TWO-GREENS-DISPATCH-DEDUP is pickable; the open posture rotation (next neighborhood src/glob.rs) resumes once the wave hands back.
