# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: b4e71723 (HEAD) — window 39429e47..HEAD reconciled;
  INSTALL-EMIT-COMPOSITION-REGRESSION-TEST shipped (0e92f868/b4e71723),
  audited on disk against its own acceptance bar (details in commit body).
- Residue swept through: b4e71723 (HEAD) — same window; no retirement or
  vocabulary residue.
- Posture swept through: cbdde828 mid-rotation — covered src/install.rs,
  src/frontmatter.rs, src/placement.rs this tick (one neighborhood:
  install.rs + its immediate imports); tests/install.rs remains in the
  frontier (imports install.rs, not the reverse — a separate neighborhood).
- This tick: POSTURE SWEEP, install.rs neighborhood — filed
  INSTALL-BANNER-GUARD-DUPLICATES-DELIMITER-DETECTION: project_banner
  hand-rolls a second opening-delimiter check 49c931ba should have routed
  through frontmatter::frontmatter_matter (details in commit body).
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 2,
  unchanged. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — the posture rotation is still open
(tests/install.rs left in the frontier) but the only pickable entry
(INSTALL-BANNER-GUARD-DUPLICATES-DELIMITER-DETECTION) ships first; the
sweep resumes once the wave hands back.
