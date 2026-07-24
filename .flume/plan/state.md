# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: 5a80006 — unchanged this tick.
- Residue swept through: 5a80006 — unchanged this tick.
- Posture swept through: mid-rotation, at src/test_support.rs (neighborhood:
  its imports — kind.rs, frontmatter::Member — already covered by prior
  rotations, nothing folds in). Frontier: toml_document.rs remains
  (test_support.rs now covered).
- This tick: POSTURE SWEEP — read src/test_support.rs (86 lines) in full;
  its imports (kind.rs, frontmatter::Member) are already-covered ground,
  nothing new folds in. Found three stale `#[allow(dead_code)]`
  annotations (skill_kind:22, rule_kind:51, surface_unit:72): re-ran
  `cargo test --no-run` and `cargo clippy --all-targets` with all three
  removed, both stayed warning-free — each already has a real consumer
  (frontmatter.rs, builtin_kind.rs). Filed TEST-SUPPORT-ALLOW-DEAD-CODE-STALE.
- Queue: 3 pending — 1 open, 1 parked, 1 deferred. Open forks: 3.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — the only remaining live job is the posture
sweep (frontier: toml_document.rs) and a pickable open entry now exists
(TEST-SUPPORT-ALLOW-DEAD-CODE-STALE); build ships it, sweep resumes after.
