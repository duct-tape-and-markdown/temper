# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 39429e47 (HEAD) — window d37b7f39..HEAD reconciled;
  FRONTMATTER-CRLF-OPEN-DELIMITER shipped (49c931ba/39429e47), audited on
  disk, filed a residual gap (details in commit body).
- Residue swept through: 39429e47 (HEAD) — same window; no retirement or
  vocabulary residue beyond the filed gap.
- Posture swept through: cbdde828 — unchanged; the shipped window's code
  delta (src/frontmatter.rs, src/install.rs, src/placement.rs,
  tests/install.rs) now re-arms the frontier for next tick.
- This tick: POST-SHIP RECONCILIATION d37b7f39..HEAD — audited
  FRONTMATTER-CRLF-OPEN-DELIMITER's shipped commit: the fix lands and its
  own two CRLF tests pass, but its acceptance named a third test (the
  verb-composition case) that never shipped; filed
  INSTALL-EMIT-COMPOSITION-REGRESSION-TEST (details in commit body).
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 2,
  unchanged. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — the only remaining live job is the posture
sweep, now re-armed over src/frontmatter.rs, src/install.rs,
src/placement.rs, tests/install.rs; INSTALL-EMIT-COMPOSITION-REGRESSION-TEST
is pickable, so build takes it first and the sweep resumes once the wave
hands back.
