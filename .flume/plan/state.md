# Plan state

- Spec derived through: f87cc0c
- Audited through: 6c89f92
- Residue swept through: e120e66
- This tick: Inbox (job 1). Routed the field report's two findings, both
  re-verified live at HEAD 967b0e6: (a) `row_relocates_builtin`'s templates
  equality check collapses to "declared templates must be empty" for every
  built-in (its own `templates` is always `Vec::new()`), hard-failing a
  legitimate templates-extension of a built-in host — filed
  TEMPLATES-RELOCATION-COLLISION-REGRESSION (open); (b) `assemble_by_kind`
  hardcodes skill/rule params and `gate()`/`explain()` independently
  re-derive the same corpus shape, so a memory member's `satisfies` never
  reaches roster/graph/coverage — filed MEMORY-ENTERS-REQUIREMENT-CORPUS,
  blockedBy the first (both edit src/main.rs, so serialized). The
  not-filed T14-refuted note needed no routing (already resolved,
  informational only). Also drained the one live `.flume/refactor/`
  capture (published_requirements permanently dead post-RETIRE-DEAD-OWN-
  PATH-SURFACE-OVERLAY, re-verified: extract.rs:328/kind.rs:652 confirmed,
  main.rs has zero references) into RETIRE-DEAD-PUBLISHED-REQUIREMENTS-
  SURFACE (open, disjoint files) and deleted the capture file. Inbox
  drained to the empty template.
- Queue: TEMPLATES-RELOCATION-COLLISION-REGRESSION open (next);
  MEMORY-ENTERS-REQUIREMENT-CORPUS blockedBy it (shared file, serialized);
  RETIRE-DEAD-PUBLISHED-REQUIREMENTS-SURFACE open (disjoint files, can run
  alongside); PACKAGING-CHANNELS parked, unchanged (not re-audited this
  tick — ship audit is job 3, not serviced).

Plan continues: yes — ship audit is live (664a522 touched src/ past
`Audited through: 6c89f92`).
