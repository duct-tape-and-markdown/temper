# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 7b8927e — unchanged, not this tick's job.
- Residue swept through: 7b8927e — unchanged, not this tick's job.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs covered, mid-rotation — advanced from src/bundle.rs. src/compose.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, src/check.rs — neighborhood is check.rs alone (its own imports are std::fmt and miette, nothing internal). Found the module header (1-10) narrating a retired heuristic rule registry's demolition in a parenthetical alongside the still-live per-rule-code invariant — era narration, none of engineering.md's three licensed narration grades, and rust.md's own named cut category. Filed CHECK-DOC-HEADER-ERA-NARRATION-CUT (open, comment-only). No other violation verified on disk: Diagnostic/Severity/Announcement/render/any_error each have live consumers (gate.rs, main.rs, reporter.rs, engine.rs, admissibility.rs, drift.rs, install.rs, coverage.rs, coverage_note.rs, roster.rs, dial.rs, graph.rs, tests/*), no parallel diagnostic-rendering surface found elsewhere, no dead-plumbing or embedded-provider-literal found.
- Queue: 3 pending, 1 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — build ships CHECK-DOC-HEADER-ERA-NARRATION-CUT, then the posture sweep resumes at src/compose.rs.
