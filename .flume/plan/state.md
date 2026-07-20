# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 39026da — unchanged, no src/tests/sdk commits past it.
- Residue swept through: 39026da — unchanged, same reason.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs, src/compose.rs, src/contract.rs, src/coverage.rs covered, mid-rotation. src/coverage_note.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, src/coverage.rs — neighborhood is coverage.rs plus its internal imports (check::Diagnostic, compose::Requirement, extract::Features); sole consumer is gate.rs:414 (plus tests/layout_edge_slot.rs), so no zero-consumer or shared-enumeration issue. Found: the module header (11-24) restates, in near-duplicate prose, the same two-diagnostic behavior `check`'s own doc (65-80) and the two rule consts' docs (57-58/61-63) already state — three homes for two facts, against engineering.md's "Narration is the ladder's bottom rung." The header's "Kinship with the graph scope" section (31-49) verified accurate (src/graph.rs's `check`/`degree` still exist and match the cited mirroring) and is a legitimate inexpressible-elsewhere design rationale, left standing. Vacuity pin present (`no_requirements_is_silent_even_with_authored_satisfies`, 375-385). Filed COVERAGE-DOC-HEADER-RULE-NARRATION-DUP, open, comment-only.
- Queue: 3 pending, 1 open (COVERAGE-DOC-HEADER-RULE-NARRATION-DUP), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched by this tick, still holding). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — a pickable entry now exists (COVERAGE-DOC-HEADER-RULE-NARRATION-DUP) so build ships it before the posture-sweep rotation (mid-rotation, next candidate src/coverage_note.rs) resumes.
