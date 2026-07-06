# Plan state

- **Phase:** derived-lock chain draining. This tick: REQUIREMENT-CLAUSES-ALGEBRA
  and CONTRACT-CORPUS-RETIRE shipped (build 3d61141/fb1edaf, chore dc1bfbf), so
  the chain's head unblocks. Empty spec delta, empty inbox — a reconcile, not a
  new-intent tick.
- **Last shipped:** REQUIREMENT-CLAUSES-ALGEBRA (3d61141) — Count/Unique/
  Membership/Degree admitted to the closed clause `Predicate` enum, parsed and
  round-tripped through the lock's ClauseRow, but Indeterminate in conformance
  until the recut wires them. CONTRACT-CORPUS-RETIRE (fb1edaf) — `contract/`
  deleted (verified gone on disk).
- **In flight — 7 entries, 1 open:**
  (1) REQUIREMENT-CLAUSES-RECUT — **open, next**. Moves the requirement's
  count/unique/membership/degree off its facet fields onto `clauses`; retires the
  facets across compose/drift/roster/graph + SDK + seam; bumps SEAM_VERSION 1→2.
  Atomic (one seam bump, both spellings can't coexist). Priority: the new
  predicates are live-but-Indeterminate today, a standing law-1 no-op the recut
  closes.
  Then the serialized derived-lock chain, each blockedBy the prior:
  FIRST-PARTY-MODULE-COMPLETE → BUILTIN-LOCK-DERIVED →
  {BUILTIN-LOCK-FROZEN-LANE, CURATED-TREES-RETIRE} (parallel-safe leaves behind
  D3). COMMENT-STOCK-SWEEP — deferred (whole-tree solo sweep, rides after the
  chain lands). PACKAGING-CHANNELS — parked (human release creds + engine-binary
  workflow + USPTO screen).
- **What's next:** build picks RECUT; the chain unblocks link by link. Queued
  **human** chores, not build work: the physical `packages/**` + `kinds/**` tree
  deletion (out of build's fence) once BUILTIN-LOCK-DERIVED + CURATED-TREES-RETIRE
  remove the code refs; the PACKAGING-CHANNELS release setup. OPEN forks in
  open-questions.md await John or the SDK-primary foundation.

Plan continues: no — queue reconciled, inbox drained, one open entry ready.
Building is how the chain drains; re-planning the same queue would be spin.
