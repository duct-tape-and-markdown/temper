# Plan state

- Spec derived through: f87cc0c
- Audited through: 425968d
- Residue swept through: ef72da3
- This tick: Ship audit d6c37a3..425968d (job 3; jobs 1-2 checked quiet first
  — inbox/refactor-captures empty, spec-delta empty). Verified on disk:
  24f9bbc consolidated import.rs's `normalize` into graph.rs's
  `normalize_path` (import.rs's three callers can't hit the `..` case, no
  behavior change); ad9f4af extracted `compose::clause_from_row`, wrapped by
  builtin.rs (kept its `.expect()` panics) and main.rs (kept its
  guidance/source drop). Both shipped entries (PATH-NORMALIZER-CONSOLIDATE,
  CLAUSE-ROW-LIFT-CONSOLIDATE) already removed from pending.json by the
  shipping commit (425968d) — nothing left to drop. Re-tested the one stale
  gate this unblocks: PLURAL-HELPER-CONSOLIDATE was `blockedBy`
  PATH-NORMALIZER-CONSOLIDATE (now shipped) — moved to `open`; its cited
  files (display.rs, bundle.rs:324, coverage_note.rs:221,
  install.rs:494/498) are untouched by either shipped commit, line numbers
  still accurate. Re-checked PACKAGING-CHANNELS's parked reason on disk:
  still no `.github/workflows/release.yml`, root package.json still the
  private flume manifest, sdk/package.json still 0.0.5 — reason holds
  unchanged. `cargo check` clean, working tree otherwise clean.
- Queue: PLURAL-HELPER-CONSOLIDATE (open, pickable) —
  TEST-SCAFFOLDING-CONSOLIDATE (blockedBy plural-helper) —
  TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy test-scaffolding) —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers) — TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders)
  (blockedBy recordwriters) — PACKAGING-CHANNELS (parked, condition
  unchanged).

Plan continues: yes — residue sweep (job 4) is now live: `Residue swept
through` (ef72da3) trails the newly advanced audit HEAD (425968d), and
24f9bbc/ad9f4af touched src/. Next tick sweeps that range.
