# Plan state

- Spec derived through: 9626a1a6 — copied forward; the delta past it is empty.
- Audited through: dc145f4e — the fold verified on disk, both refusal cases green.
- Residue swept through: dc145f4e — same window, swept with the audit: clean.
- Posture swept through: mid-rotation — frontier armed at ee7de9f9 (the
  221-commit window off 7d695577; 862f0b60's phrase delta arms the whole
  domain). Covered: `src/drift.rs`. `src/install.rs` next.
- This tick: drained the drift.rs cohesion capture into parked
  LOCK-DECLARATION-CODEC-GETS-ITS-OWN-MODULE and reconciled the open entry's
  ripple (`src/install.rs` widened in).
- Queue: 8 pending — 1 open, 4 parked, 3 deferred. Pickable: 1. Open forks: 1.
  Friction: 2. Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the posture rotation resumes on `src/install.rs`
once the wave hands back; nothing else is queue-shaping.
