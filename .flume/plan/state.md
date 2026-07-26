# Plan state

- Spec derived through: 53e6f615 — 0046 routed: two Consequences already
  shipped in-commit (verified on disk), the third filed as an entry.
- Audited through: 5b7158c4 — window 8d15b725..5b7158c4 audited;
  TELEMETRY-FIELD-REDERIVES-MEMBER-INDEX's work verified on disk
  (src/telemetry.rs::field takes `member_index` from its sole caller
  instead of rebuilding a `declared: HashSet` from `by_kind`), read_verbs.rs
  all 23 green, entry already absent from pending.json (dropped on ship).
  metrics.jsonl: clean ship, no revert.
- Residue swept through: 5b7158c4 — same window; grepped for the retired
  `declared: HashSet`/`declared.contains` pattern — the only other
  `declared` sites (admissibility.rs, engine.rs) are pre-existing, distinct
  jobs, not a copy of this one. clippy clean.
- Posture swept through: 39107255 — mid-rotation, window 39107255..HEAD.
  Frontier: src/telemetry.rs closed (reconciled prior tick); src/tap.rs
  closed this tick; src/read.rs still open, unread in full.
- This tick: POSTURE SWEEP — src/tap.rs neighborhood, filed TAP-WORKTREE-LAYOUT-FACT-UNCITED (see commit body for the finding).
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3. Friction:
  2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — TAP-WORKTREE-LAYOUT-FACT-UNCITED is pickable
and ships first; the posture rotation (src/read.rs still in the frontier)
resumes once the wave hands back.
