# Plan state

- Spec derived through: f87cc0c
- Audited through: dd38241
- Residue swept through: e120e66
- This tick: Ship audit (job 3). Commits past the prior cursor (6c89f92)
  touching src/tests/sdk: 664a522 (build: retire the pre-0016 own-path
  surface-document mechanism, 22 files) and 967b0e6 (chore(flume) ship
  commit, pending.json-only). Re-read src/main.rs on disk and confirmed
  every citation in the three entries the prior inbox tick scoped against
  967b0e6 still resolves exactly at HEAD: row_relocates_builtin/
  KIND_COLLISION_RULE/partition_kind_rows at main.rs:969-1010
  (TEMPLATES-RELOCATION-COLLISION-REGRESSION); explain/gate/
  assemble_by_kind at main.rs:400,549,896 with the skill/rule-only
  dispatch and independent re-derivation both confirmed
  (MEMORY-ENTERS-REQUIREMENT-CORPUS, still correctly blockedBy the first —
  shared file); extract.rs:328, kind.rs:652, read.rs's sole_publisher/
  narration block all confirmed dead-but-present
  (RETIRE-DEAD-PUBLISHED-REQUIREMENTS-SURFACE). No entry's work shipped
  early, none needed a rewrite. Re-tested PACKAGING-CHANNELS' parked
  reason: still no .github/workflows/release.yml (only temper.yml, a
  check job) and root package.json is still the private flume manifest —
  refreshed the reason's re-verification stamp to this tick's sha, gate
  unchanged. No live refactor captures or friction to route (job 1 stays
  quiet; friction drains out of band per README).
- Queue: TEMPLATES-RELOCATION-COLLISION-REGRESSION open (next);
  MEMORY-ENTERS-REQUIREMENT-CORPUS blockedBy it (shared file, serialized);
  RETIRE-DEAD-PUBLISHED-REQUIREMENTS-SURFACE open (disjoint files, can run
  alongside); PACKAGING-CHANNELS parked, re-confirmed unchanged.

Plan continues: yes — residue sweep is live (Residue swept through: e120e66
trails HEAD; no spec delta, no inbox content, ship audit now current).
