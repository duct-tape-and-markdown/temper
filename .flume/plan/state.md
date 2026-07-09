# Plan state

- Spec derived through: f87cc0c
- Audited through: 2c853af
- Residue swept through: 5d995a3
- This tick: Ship audit (job 3), commits 5d995a3..2c853af touched src/tests.
  `3e125da` and `19a4a5c` shipped TEMPLATES-RELOCATION-COLLISION-REGRESSION
  (main.rs's `row_relocates_builtin` templates equality drop, confirmed —
  diff matches the entry's description) and RETIRE-DEAD-PUBLISHED-
  REQUIREMENTS-SURFACE (Features.published_requirements and impact's
  dangling-demand narration gone, confirmed — zero `published_requirements`
  hits left in src/); `2c853af` (human `chore(flume)`) already drained both
  from pending.json. Re-tested the stale `blockedBy` gate on
  MEMORY-ENTERS-REQUIREMENT-CORPUS per job 3's instruction: its blocker
  shipped, so verified the entry's premise live on disk —
  main.rs:582-601 (gate's skill/rule-only dispatch), :893-904
  (assemble_by_kind's two-param signature), :400-438 (explain's independent
  re-derivation) all confirmed unchanged by the two builds. Unblocked:
  gate flips blockedBy -> open, line refs and notes refreshed to HEAD.
  PACKAGING-CHANNELS parked reason re-checked, unchanged (no
  `.github/workflows/release.yml`, root package.json still the private
  flume manifest).
- Queue: MEMORY-ENTERS-REQUIREMENT-CORPUS open (next, disjoint —
  touches only src/main.rs); PACKAGING-CHANNELS parked, disjoint (touches
  package.json + a new release.yml).

Plan continues: yes — residue sweep cursor (5d995a3) trails HEAD (2c853af);
job 4 is next.
