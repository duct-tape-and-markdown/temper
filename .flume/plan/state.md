# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 067d6c7 — advanced; window c8b2c8a..067d6c7 reconciled.
- Residue swept through: 067d6c7 — advanced; same window, clean.
- Posture swept through: 97d0241 — armed for next tick: this window's
  ddd755a6 (src/) and b78d7895 (sdk/src/) touch modules, a code delta.
- This tick: POST-SHIP RECONCILIATION — audited+swept c8b2c8a..067d6c7.
  ddd755a6 (EMBEDDED-KIND-GUIDANCE-DELIVERY build) matches its entry's
  files[] exactly (declarations.ts, compose.rs, main.rs, read.rs,
  emit.test.ts, read_verbs.rs); its two departures (kindFactKindsInPlay's
  guidance/cite-gated admission, resolve's fallback-only kind lookup) are
  commit-body-justified deviations from plan shape, not scope creep; entry
  already drained by its own 162e1cce ship commit. b78d7895 is a docs-only
  kind-count correction. Verified live: cargo test (22/22 read_verbs),
  pnpm --dir sdk test (141/141), clippy -D warnings clean, fmt clean, no
  `unitKindsInPlay` residue (rg, zero hits). No findings.
- Queue: 5 pending — 2 open, 1 parked, 2 deferred. Open forks: 2, unchanged.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — reconciliation closed; the only remaining
live plan job is the posture sweep (armed above), but 2 pickable open
entries (SCHEMA-KIND-DOMAIN-WIDEN, INSTALL-SCAFFOLD-MEMBER-DIR-WIDEN) sit
in the queue, so build takes the wave first; the sweep resumes once it
hands back.
