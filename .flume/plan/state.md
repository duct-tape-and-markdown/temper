# Plan state

- Spec derived through: f87cc0c
- Audited through: 85fdffd
- Residue swept through: 6d00c14
- This tick: Ship audit 7904498..85fdffd (job 3). Verified on disk (diff
  read, not log alone): 726769d shipped KIND-NAME-COLLISION-ADMISSIBILITY
  cleanly — `partition_kind_rows`/`row_relocates_builtin`/
  `kind_collision_diagnostic` land in src/main.rs, both new tests
  (`a_kind_name_colliding_with_a_built_in_fires_an_admissibility_diagnostic`,
  `a_kind_row_relocating_a_built_ins_governs_fires_no_collision_diagnostic`)
  pass, cargo check green; 85fdffd removed the entry from pending.json.
  Re-tested the stale `blockedBy` on SATISFIES-CLAUSES-RATIONALE-FROM-LOCK
  per the "if the blocker shipped" rule: its blocker is gone and no longer
  contends for explain's loop (now routed through `partition_kind_rows`,
  no remaining overlap), so flipped its gate to `open` and refreshed its
  main.rs line citations (loop shifted; resolve_kind_units now ~L849-891,
  surface_overlay ~L820, CustomMember.satisfies build ~L439).
  RETIRE-DEAD-OWN-PATH-SURFACE-OVERLAY's blockedBy chain is unaffected.
  Re-verified PACKAGING-CHANNELS's parked reason still holds.
- Queue: SATISFIES-CLAUSES-RATIONALE-FROM-LOCK open (sole pickable entry);
  RETIRE-DEAD-OWN-PATH-SURFACE-OVERLAY blockedBy it; PACKAGING-CHANNELS
  parked, unchanged. Queue is disjoint (only one open entry).

Plan continues: yes — residue sweep is next (swept-through 6d00c14 trails
HEAD 85fdffd by 726769d, a src/tests-touching commit not yet swept).
