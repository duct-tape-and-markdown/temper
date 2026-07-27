# Plan state

- Spec derived through: 53e6f615 — 0046 routed: two Consequences already
  shipped in-commit (verified on disk), the third filed as an entry.
- Audited through: 56eff8a4 — window 5b7158c4..56eff8a4 audited;
  TAP-WORKTREE-LAYOUT-FACT-UNCITED's work verified on disk
  (src/tap.rs::log_path's doc comment now cites git-scm.com/docs/git-worktree,
  retrieved 2026-07-26, for the gitdir/commondir relative-path fact), `cargo
  test --test tap` all 9 green, `cargo clippy --all-targets` clean, entry
  already absent from pending.json (dropped on ship). metrics.jsonl: clean
  ship, no revert.
- Residue swept through: 56eff8a4 — same window; a single doc-comment
  addition, no demolition or retirement named in either commit body; no
  residue class to check.
- Posture swept through: 173cdf54 — rotation closed: last frontier module
  src/read.rs (+ imports compose.rs/graph.rs/telemetry.rs/tap.rs) read,
  filed READ-NOTFOUND-SKILLS-RULES-LITERAL.
- This tick: INBOX — routed the 0037 requirement-grain note into REQUIREMENT-GRAIN-TELEMETRY-FIELD-STRAND (open); drained.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3. Friction:
  2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — spec delta unrouted past 53e6f615 (e81baa47,
bb531253); job 2 next tick.
