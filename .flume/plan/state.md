# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 82d415c — unchanged, no commits past it touched src/, tests/, or sdk/.
- Residue swept through: 82d415c — unchanged, same reason.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs, src/check.rs, src/compose.rs, src/contract.rs covered, mid-rotation. src/coverage.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, src/contract.rs (+ imports drift.rs/extract.rs/address.rs/compose.rs read for context) — `Shape::LeadingDotSlash`, live on the marketplace kind's `plugins[*].source` (sdk/src/builtins.ts:1113), carries none of the behavioral test coverage its two sibling shapes get in tests/shape_predicate.rs; filed SHAPE-LEADING-DOT-SLASH-UNTESTED (open). Also surfaced a new ride-only stale cite (contract.rs's `SHAPE_PATTERNS` doc naming a deleted `crate::contract::tests` module) — recorded in open-questions.md's existing ride-only record, not filed as an entry. `Predicate::Kind`/`DependencyExists`'s absence from `predicate_from_row`'s match checked and confirmed deliberate (both documented, both synthesized/fenced elsewhere) — not residue.
- Queue: 3 pending, 1 open (SHAPE-LEADING-DOT-SLASH-UNTESTED), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched by this tick, still holding). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — the only remaining live job is the posture sweep (mid-rotation, next candidate src/coverage.rs), but a pickable entry now exists (SHAPE-LEADING-DOT-SLASH-UNTESTED), so the wave ships it first and the sweep resumes after.
