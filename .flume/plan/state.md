# Plan state

- **Phase:** derived-lock chain — foundation shipped, consumer link re-sliced.
  Inbox empty; spec delta empty (no `specs/` commits since 1a71b06).
- **Last shipped:** BUILTIN-LOCK-DERIVED + CURATED-TREES-RETIRE (cf0cdac) —
  `src/builtin_lock.toml` + `builtin_lock.rs` embed and parse the lock derived
  from the memberless module emit; bundle.rs's `packages/` embeds dropped.
- **Reconcile this tick — BUILTIN-LOCK-ROW-DRIVEN was stale *and* oversized,
  so it split:** (1) its test premise (two `memory` providers colliding) is
  unwritable — the derived lock carries ONE `memory` kind, no provider column,
  no `agents-md.memory`; (2) a fresh find on disk: the SDK→lock seam **drops
  clause guidance+cite** (`sdk/src/declarations.ts` `clauseRow` and
  `drift::ClauseRow` carry neither — drift.rs:990 says so), so a floor
  projection would lose the teaching channel the offline gate delivers
  (10-contracts, "The clause"); (3) blast radius (~13 src + ~10 test files) is
  multi-commit. Re-sliced into LOCK-CLAUSE-CHANNELS (seam+lock carry
  guidance+cite), BUILTIN-KIND-FLATTEN (flatten kind identity, drop agents-md +
  the qualified machinery), BUILTIN-FLOOR-LOCK-PROJECTION (floors project the
  lock). New fork `(agents-md-builtin-kind)` surfaced (dropping engine AGENTS.md
  coverage to match the module — feature-add question, not a blocker).
- **Queue — 7 entries, 3 open (disjoint file sets):** LOCK-CLAUSE-CHANNELS
  (seam+lock: declarations.ts/drift.rs/builtin_lock.toml), BUILTIN-KIND-FLATTEN
  (builtin_kind/kind/builtin/main/read/compose/install/check + memory tests),
  BUILTIN-LOCK-FROZEN-LANE (new test file). Then serial:
  BUILTIN-FLOOR-LOCK-PROJECTION (blockedBy FLATTEN) → CHECK-LOCK-KIND-ROWS
  (blockedBy PROJECTION; cascade custom-kind-rows fix). COMMENT-STOCK-SWEEP
  deferred (whole-tree solo). PACKAGING-CHANNELS parked (release creds + engine
  workflow + USPTO screen).
- **What's next:** build picks the 3 open disjoint entries in parallel; the
  chain drains link by link. Queued **human** chore: the physical `packages/**`
  + `kinds/**` deletion (out of fence) once the chain + CURATED land.

Plan continues: no — queue reconciled (ROW-DRIVEN re-sliced into a tractable
chain, guidance/cite gap filed as its foundation), inbox empty, three disjoint
open entries pickable. Building is how the chain drains.
