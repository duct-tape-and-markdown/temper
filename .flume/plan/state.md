# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 9329952 — window 638f051..9329952 touching src/tests/sdk was one
  commit (c732006, build: extract normalize_path to path.rs).
- Residue swept through: 9329952 — same window, same commit.
- Posture swept through: sdk/src/ tree fully covered; src/address.rs covered.
  src/admissibility.rs next in tree order — mid-rotation.
- This tick: POST-SHIP RECONCILIATION over 638f051..9329952. Audit: c732006 ships exactly
  what ADDRESS-NORMALIZE-PATH-COHESION-SPLIT scoped — normalize_path (with its doc comment
  and Component/Path/PathBuf imports) moved verbatim to new leaf module src/path.rs;
  address.rs's //! header now describes only field addressing; all call sites (drift.rs
  x3, compose.rs x2, graph.rs x3, import.rs x3+test, tests/directive_classing.rs)
  requalified to crate::path::normalize_path, no stale address::normalize_path reference
  anywhere (`rg` confirms); cargo build is clean; the ship commit (9329952) already
  removed the entry from pending.json — nothing left to drop. Stale-gate re-test: window
  touches no .github/ (PACKAGING-CHANNELS-REMAINDER's park unaffected, `git log
  ac40c72..HEAD -- .github/` empty) and no sdk/src/kind.ts or field-guidance surface
  (GUIDANCE-FIELD-DECLARATION-CHANNEL's defer unaffected) — both gates still hold as
  stated. Sweep: same window, no retirement named, no orphaned vocabulary, no second
  implementation — clean.
- Queue: 2 pending — 1 deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked
  (PACKAGING-CHANNELS-REMAINDER); 0 open. Open forks: 2, unchanged. Friction: 0.
  Amendments: 1, still awaiting ratification. Inbox: 0.

Plan continues: yes — posture rotation is mid-rotation with no pickable entry in queue
(both remaining entries are parked/deferred), so plan drives the sweep to
src/admissibility.rs next tick rather than waiting on build.
