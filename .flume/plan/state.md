# Plan state

- Spec derived through: f87cc0c
- Audited through: d6c37a3
- Residue swept through: ef72da3
- This tick: Residue sweep (job 4), range da9245c..ef72da3. A same-tick
  prior attempt (ff355da) was reverted by the pending.json-parses gate: two
  split-entry tags used a hyphenated parenthesized slice
  (`(RECORD-WRITERS)`, `(LOCK-UNIT-BUILDERS)`), and flume's `TAG_PATTERN`
  (`/home/john-work/repos/flume/src/PendingSchema.ts:71`) only allows
  `\([a-z0-9]+\)` in the suffix — lowercase alnum, no separators. HEAD was
  unchanged since that attempt (still ef72da3), so every cited symbol and
  line number was re-verified on disk this tick (`grep -n` each of the ~35
  cited fns/lines across src/ and tests/ — all accurate) and the same
  substance re-filed with corrected tags:
  `TEST-HELPER-DUPES-CONSOLIDATE(recordwriters)` and
  `TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders)`. Also filed
  CLAUSE-ROW-LIFT-CONSOLIDATE (open, disjoint files) for a triple
  near-duplicate "lift a ClauseRow into a Clause"
  (src/builtin.rs:24 panicking, src/compose.rs:143 tolerant inline closure
  whose own doc comment already admits the duplication, src/main.rs:1082
  tolerant, drops guidance/source) — named in prose, never consolidated in
  code. Filed the two-entry chain
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) then (lockunitbuilders),
  blockedBy TEST-FIXTURE-HELPERS-CONSOLIDATE, for 16 more copy-pasted/
  near-duplicate test builders the four existing test-scaffolding entries
  don't cover (write_requirements/write_rule/write_retired_manifest/
  write_lock/retired_manifest_name/clean_skill, then surface_unit/
  skill_member/rule_member/rule_kind_facts/skill_kind_facts/
  skill_surface_unit/findings_for/author_rule_satisfies/requirement, plus
  folding check_harness into check_in). Validated all 8 pending tags
  against TAG_PATTERN and re-confirmed every `per`/`files` path resolves on
  disk before writing. Cursor advances to HEAD (ef72da3).
- Queue: PATH-NORMALIZER-CONSOLIDATE (open) — CLAUSE-ROW-LIFT-CONSOLIDATE
  (open, independent) — PLURAL-HELPER-CONSOLIDATE (blockedBy
  path-normalizer) — TEST-SCAFFOLDING-CONSOLIDATE (blockedBy
  plural-helper) — TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy
  test-scaffolding) — TEST-HELPER-DUPES-CONSOLIDATE(recordwriters)
  (blockedBy test-fixture-helpers) — TEST-HELPER-DUPES-CONSOLIDATE
  (lockunitbuilders) (blockedBy recordwriters) — PACKAGING-CHANNELS
  (parked, condition unchanged).

Plan continues: yes — quiet closing pass (job 5) is next: inbox, spec
delta, ship audit, and residue sweep are all current as of this tick;
job 5 re-derives state.md and confirms the queue is disjoint before
handing off to build.
