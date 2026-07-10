# Plan state

- Spec derived through: a0fccaf
- Audited through: 8575dd6
- Residue swept through: 1817f39
- This tick: Ship audit — 3024687..HEAD, one code commit (9e48cd8,
  LOCK-READ-SWALLOW-LOUD) verified on disk: `mode_from_lock`
  (main.rs:508) and `with_locked_kinds` (coverage_note.rs:216) both
  propagate the `read_declarations` error; an out-of-vocabulary mode
  raises `LockRowError::Vocabulary`; rg confirms every remaining caller
  uses `?` (main.rs 288/414/509/579/771, coverage_note.rs:221); tests
  landed at tests/cli.rs:484 and tests/coverage_note.rs:188. The entry
  was already dropped from pending by the ship chore. PACKAGING-CHANNELS
  parked reason re-verified (still no release.yml; root package.json
  still the private flume manifest) and restamped at 8575dd6.
- Queue: PACKAGING-CHANNELS (parked) — none pickable.

Plan continues: yes — residue sweep (swept-through 1817f39 trails HEAD;
9e48cd8's hunks unswept).
