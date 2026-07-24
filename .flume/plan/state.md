# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 9c2db5f4 (HEAD) — window b4e71723..HEAD reconciled;
  INSTALL-BANNER-GUARD-DUPLICATES-DELIMITER-DETECTION shipped
  (bd122cf4/9c2db5f4), audited on disk: project_banner now delegates to
  frontmatter::frontmatter_matter, both regression tests
  (project_banner_declines_a_crlf_opened_frontmatter_source,
  project_banner_takes_an_unterminated_frontmatter_block) present and
  green (`cargo test --lib install::`, 16 passed).
- Residue swept through: 9c2db5f4 (HEAD) — same window; grepped for
  other hand-rolled `---`-delimiter detection (vs. frontmatter_matter)
  across src/ and tests/ — remaining hits are one-directional banner/
  modeline writers and test assertions, not duplicate detection logic;
  no residue.
- Posture swept through: cbdde828 mid-rotation — covered src/install.rs,
  src/frontmatter.rs, src/placement.rs this tick (one neighborhood:
  install.rs + its immediate imports); tests/install.rs remains in the
  frontier (imports install.rs, not the reverse — a separate neighborhood).
- This tick: POST-SHIP RECONCILIATION, window b4e71723..HEAD — audited
  INSTALL-BANNER-GUARD-DUPLICATES-DELIMITER-DETECTION on disk (clean,
  pending.json already retired it) and swept the same window for residue
  (clean, none found).
- Queue: 3 pending — 0 open, 1 parked, 2 deferred. Open forks: 2,
  unchanged. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — the posture rotation is still open (tests/install.rs
left in the frontier) and the queue has no pickable entries right now
(1 parked, 2 deferred), so plan drives the sweep itself next tick.
