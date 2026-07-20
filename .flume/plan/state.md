# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 1b0ea01 — advanced from f17678f.
- Residue swept through: 1b0ea01 — advanced from f17678f.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs covered, mid-rotation — advanced from src/builtin_kind.rs. src/bundle.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, src/builtin_lock.rs neighborhood (frontier module + its immediate imports: drift::{Declarations, parse_declarations}). Quiet-on-clean: sole real consumer is crate::builtin (compose::default_contract_from_rows over declarations().clauses) plus several tests, so the export earns its consumer; the LazyLock parse-once shape satisfies cost-scale hoisting; the module doc's "derived, never transcribed" claim is mechanically enforced by tests/builtin_lock_frozen.rs's writer-vs-reader byte-equality gate against the SDK's own memberless emit, not a bare comment; the hand-written kind-name/locus literals in this file's own test module assert already-generated, gate-verified data (protected against silent drift by that same frozen test) rather than inventing provider knowledge outside the provider face. No cohesion, shared-enumeration, derived-state, or vacuity violation found.
- Queue: 2 pending, 0 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep is mid-rotation with its frontier open past src/builtin_lock.rs, no pickable entry exists to hand off to build, so plan resumes it next tick at src/bundle.rs.
