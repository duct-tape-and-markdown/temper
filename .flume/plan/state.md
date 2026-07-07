# Plan state

- **Phase:** derived-lock chain draining. Inbox empty; spec delta empty (no
  `specs/` commits since 1a71b06).
- **Last shipped (bedbfca):** LOCK-CLAUSE-PREDICATE-ARGS (a89279c) — the
  missing foundation link. ClauseRow (SDK + Rust) now carries the four
  node-scope argument columns (bound/charset/keys/values), round-tripped
  through to_table/from_table, and builtin_lock.toml was regenerated so every
  floor row carries its predicate's own argument. Verified on disk:
  `builtin_lock::declarations().clauses` is a lossless floor source
  (predicate·arg·field·severity·guidance·cite).
- **This tick's correction — the projection is unblocked and implementable.**
  BUILTIN-FLOOR-LOCK-PROJECTION was blockedBy LOCK-CLAUSE-PREDICATE-ARGS,
  which has shipped, and the reduced-row obstacle it named is gone (the lock
  now carries full args). Flipped to **open**. Added builtin_lock.rs to its
  scope: builtin.rs consuming `declarations()` makes that module's `//!`
  header ("nothing yet reads declarations" + the dangling
  BUILTIN-LOCK-ROW-DRIVEN tag) stale on contact (rust.md exit clause).
- **Queue — 4 entries, 1 open:** BUILTIN-FLOOR-LOCK-PROJECTION (open) →
  CHECK-LOCK-KIND-ROWS (blockedBy PROJECTION — cascade field report).
  COMMENT-STOCK-SWEEP deferred (whole-tree solo; promoted once the chain
  lands and the queue is otherwise empty). PACKAGING-CHANNELS parked (release
  creds + engine workflow + USPTO).
- **What's next:** build picks BUILTIN-FLOOR-LOCK-PROJECTION; the chain drains
  its second-to-last link. When it ships, plan re-checks whether the
  row-driven path already reads the committed lock's custom kinds (if so,
  CHECK-LOCK-KIND-ROWS collapses), then promotes COMMENT-STOCK-SWEEP.

Plan continues: no — queue reconciled (projection's cleared blocker flipped
to open, builtin_lock.rs header staleness folded into its scope), inbox empty,
delta empty. Building is how the chain drains.
</content>
