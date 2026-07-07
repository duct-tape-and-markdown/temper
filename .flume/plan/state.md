# Plan state

- **Phase:** derived-lock chain head unblocks + cascade field reports routed.
  REQUIREMENT-CLAUSES-RECUT shipped (c17332a/3e12840): Requirement carries a
  `clauses` vec, SEAM_VERSION 1→2, so FIRST-PARTY-MODULE-COMPLETE's blockedBy
  clears → open. Empty spec delta; inbox drained (3 cascade lines routed).
- **Last shipped:** REQUIREMENT-CLAUSES-RECUT (c17332a) — count/unique/
  membership/degree recut off requirement facets onto ordinary nested clause
  rows across compose/drift/roster/graph + SDK; end-to-end emit test added.
- **In flight — 8 entries, 2 open (parallel-safe, disjoint):**
  (1) EMIT-INTO-PATH-DOUBLING — **open**. `run_sdk_program` sets node cwd to
  the entry's parent yet passes the relative entry path → `<into>/<into>/
  harness.ts`, MODULE_NOT_FOUND on the `./.temper` default; fix + fail-loud
  test. drift.rs + tests/emit.rs.
  (2) FIRST-PARTY-MODULE-COMPLETE — **open**. Export the four built-in floors
  (skill/rule/memoryAnthropic/memoryAgentsMd) from @dtmd/temper/claude-code,
  each clause cited; sdk/** only.
  Then the serialized derived-lock chain: BUILTIN-LOCK-DERIVED (D3) →
  {CHECK-LOCK-KIND-ROWS, BUILTIN-LOCK-FROZEN-LANE, CURATED-TREES-RETIRE}
  (three parallel-safe leaves behind D3; CHECK-LOCK-KIND-ROWS carries the
  cascade custom-kind-rows fix, serialized because it edits the same
  check/main/engine kind-resolution D3 makes row-driven). COMMENT-STOCK-SWEEP
  — deferred (whole-tree solo). PACKAGING-CHANNELS — parked (release creds +
  engine-binary workflow + USPTO screen).
- **What's next:** build picks EMIT-INTO-PATH-DOUBLING + FIRST-PARTY-MODULE-
  COMPLETE in parallel; the chain unblocks link by link. Cascade DATUM (drained,
  no entry): the 0.0.3 seam-flow writer handoff is byte-perfect (`0 emitted, 95
  unchanged`); TEMPER-TOML-ZERO confirmed externally (root temper.toml deleted,
  membership reads off the lock). Queued **human** chores: the physical
  packages/** + kinds/** deletion (out of fence) once D3 + CURATED-TREES land;
  PACKAGING-CHANNELS release setup. Open forks in open-questions.md await John
  or the SDK-primary foundation.

Plan continues: no — queue reconciled, inbox drained, two disjoint open entries
ready. Building is how the chain drains; re-planning the same queue would spin.
