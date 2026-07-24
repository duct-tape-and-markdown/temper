# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: e5e23e78 — release commits 7e83f117/43743407 land past
  it (sdk/package.json, sdk/package-lock.json, Cargo.toml/lock,
  CHANGELOG.md — version/lockfile sync, no src/tests/sdk logic); not yet
  reconciled — next tick's job.
- Residue swept through: e5e23e78 — same window, same two commits, not yet
  swept.
- Posture swept through: cbdde828 — forward window (cbdde828..HEAD over
  src/, sdk/src/, tests/, engineering.md, architecture.md) empty; rotation
  stays closed.
- This tick: INBOX — drained #16/#16a. Both were already routed directly
  (interactive commit 2535b6cf, not a plan tick) into
  FRONTMATTER-CRLF-OPEN-DELIMITER (gate open); re-verified against HEAD
  (git log 43743407..HEAD touching src/frontmatter.rs, src/install.rs,
  src/placement.rs, tests/install.rs — empty) that nothing shipped since
  scoping to invalidate it. No pending.json rewrite needed; inbox emptied.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 2,
  unchanged. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation window e5e23e78..HEAD
(the two release commits above) is unreconciled and outranks the posture
sweep; audit it next tick even though it's a version/lockfile-only diff.
