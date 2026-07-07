# Plan state

- Spec derived through: f4189c3
- Audited through: 44b4f27
- Residue swept through: ce6c183
- This tick: Residue sweep caed5cf..HEAD (build commits f8e039b/4049691/6fd37f5
  + chore 44b4f27). One class found and confirmed on disk: `src/drift.rs:1405`
  `declarations_from_doc` doc comment reads 'the four declaration families'
  while the code extracts six (kinds, clauses, requirements, assembly,
  satisfies, mentions) — it lagged the mentions family EMIT-REAP-ORPHANS added;
  peers :289/:296/:1057 already read 'six'. Comment-only staleness → the job-4
  exception (never a standalone entry): routed as a RIDER on EMIT-LF-NORMALIZE
  (next entry opening drift.rs), in its drift.rs edit description. No retired
  vocabulary in the build additions (coverage_note/drift/install/main scanned).
  Cursor → ce6c183. Spec f4189c3 + Audit 44b4f27 copied forward verbatim (inbox
  empty, no spec delta past f4189c3, no unaudited src past 44b4f27).
- Queue: 4 — two disjoint open fronts ready for build: EMIT-LF-NORMALIZE
  (drift.rs+tests/emit.rs, carries the comment rider), SETTINGS-FORMAT-PRESERVING
  (install.rs+tests/install.rs). MODULE-RELATIVE-PATHS blockedBy SETTINGS
  (install.rs chain tail; +SDK), PACKAGING-CHANNELS parked.

Plan continues: no — every input current (inbox empty, spec cursor f4189c3 with
no delta, audit 44b4f27 with no unaudited src, residue at HEAD). Queue holds two
pickable `open` entries over disjoint files — build takes over.
