# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: 137e1df — unchanged: `git log 137e1df..HEAD -- src/
  tests/ sdk/` is empty, no window to reconcile.
- Residue swept through: 137e1df — unchanged, same reason.
- Posture swept through: mid-rotation, at src/schema.rs (neighborhood:
  its imports — contract, extract, address — already covered, nothing
  folds in). Frontier: test_support.rs, toml_document.rs remain
  (tap.rs/telemetry.rs already folded into read.rs's neighborhood).
- This tick: POSTURE SWEEP src/schema.rs — filed
  SCHEMA-HEADER-RESTATES-INLINE-CHANNEL-DOCS (module header duplicates
  channel-split and field-address mechanics already carried adjacent to
  the code); everything else clean this neighborhood — DATUM in commit
  body.
- Queue: 3 pending — 1 open, 1 parked, 1 deferred. Open forks: 2,
  unchanged. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — SCHEMA-HEADER-RESTATES-INLINE-CHANNEL-DOCS
ships first; the posture sweep resumes (frontier: test_support.rs,
toml_document.rs) when the wave hands back.
