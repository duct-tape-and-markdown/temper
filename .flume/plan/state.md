# Plan state

- Spec derived through: f87cc0c
- Audited through: dd38241
- Residue swept through: 16f2fb6
- This tick: Residue sweep (job 4). e120e66..HEAD touching src/tests/sdk:
  1b5dd23 (satisfies rationale from lock — its fallout already filed as
  MEMORY-ENTERS-REQUIREMENT-CORPUS + the gate/explain duplication it rides)
  and 664a522 (own-path surface retirement — its fallout already filed as
  RETIRE-DEAD-PUBLISHED-REQUIREMENTS-SURFACE). Verified the retirement itself
  left no residue: `rg` for surface_overlay/from_surface/to_document/
  from_member_document/REQUIREMENT_COLLISION_RULE/to_requirement/
  check::Workspace/union_published_requirements across src+tests+sdk returns
  nothing but one test-fn name. No TODO/FIXME/XXX/HACK markers anywhere in
  src/tests/sdk. Re-verified both accepted "kinds/+packages/ curated trees"
  debts unchanged at HEAD: session_start.rs's +++-fixtures test untouched by
  664a522's edits to the same file (only the two satisfies tests moved);
  builtins.ts:308,348,385 untouched since 706139a. Refreshed both stamps in
  open-questions.md to this tick's HEAD; retargeted their re-verify pointer
  from "next ship audit" to "next residue sweep" (this class is residue, not
  a pending-entry/parked-reason check). No refactor captures live (dir holds
  only README); friction dir has 3 live notes, out of plan's job per README.
- Queue: unchanged — TEMPLATES-RELOCATION-COLLISION-REGRESSION open (next);
  MEMORY-ENTERS-REQUIREMENT-CORPUS blockedBy it; RETIRE-DEAD-PUBLISHED-
  REQUIREMENTS-SURFACE open, disjoint; PACKAGING-CHANNELS parked.

Plan continues: yes — quiet closing pass next (all four inputs above are now
current; nothing moved this tick that reopens 1-3).
