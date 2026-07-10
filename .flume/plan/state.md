# Plan state

- Spec derived through: a0fccaf
- Audited through: 8575dd6
- Residue swept through: df667e4
- This tick: Residue sweep — 1817f39..HEAD, one code commit (9e48cd8)
  swept clean: no retired vocabulary in its hunks; the mode-string parse
  has one home (main.rs:518-520 — install.rs maps enum→enum, sdk
  assembly.ts is the authoring-side type, not a second parser); the
  out-of-vocabulary rejection reuses `drift::LockRowError::Vocabulary`,
  no second error shape; new tests use `common::tmpdir`, and their
  hand-written malformed lock strings are correct spelling
  (`common::write_lock` serializes a typed value that cannot express
  corrupt bytes). Nothing fileable. All eight riding debts re-verified
  on disk at df667e4 and restamped — 9e48cd8 touched none of their
  files.
- Queue: PACKAGING-CHANNELS (parked) — none pickable.

Plan continues: yes — quiet pass (closing verification, then hand off).
