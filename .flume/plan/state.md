# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 385cf3e — window c9d8b24..385cf3e reconciled, unchanged.
- Residue swept through: 385cf3e — same window, unchanged.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs, src/compose.rs, src/contract.rs, src/coverage.rs, src/coverage_note.rs, src/dial.rs, src/display.rs, src/document.rs, src/drift.rs, src/engine.rs, src/extract.rs covered, mid-rotation. src/frontmatter.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, neighborhood src/extract.rs (a leaf module — zero crate-internal imports, so the neighborhood is the module alone). One finding filed: EXTRACT-BODY-HEADINGS-COLLECT-HEADS-DEDUP (`body_headings` reimplements `collect_heads`'s fence/ATX scan instead of delegating to it, per "One job, one home"); its scope also names and will discharge the tracked ride-only `manifest_members`/`number_kind` doc-comment orphan. Checked and cleared: the `code.claude.com` cites in `body_at_imports`'s doc comments are the same cite-at-point-of-use pattern used across graph.rs/kind.rs/json_manifest.rs/bundle.rs/builtin_kind.rs, not provider knowledge leaking outside the provider face — not filed.
- Queue: 3 pending, 1 open (EXTRACT-BODY-HEADINGS-COLLECT-HEADS-DEDUP), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched this tick). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — the only remaining live input is the open posture rotation (next neighborhood src/frontmatter.rs), but a pickable entry now exists, so ready work ships first and the sweep resumes when the wave hands back.
