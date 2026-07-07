# Plan state

- **Phase:** derived-lock chain draining. Inbox empty; spec delta empty (no
  `specs/` commits since 1a71b06).
- **Last shipped (3448ffa):** BUILTIN-KIND-FLATTEN (3712d24 — kind identity
  flattened onto the lock's bare row labels; the provider-qualified
  resolution machinery, `agents-md.memory`, and `AmbiguousKind`/`resolve_bare`
  gone). Verified on disk: `resolve_bare`/`AmbiguousKind`/`agents-md.memory`
  absent from `src/`; `check::Workspace` keys built-ins by bare name over one
  `memory` kind — collision-free. Closes `(builtin-workspace-qualified-key)`
  (deleted from open-questions).
- **This tick's correction — the projection had the wrong blocker.**
  BUILTIN-FLOOR-LOCK-PROJECTION was blockedBy FLATTEN (now shipped) and would
  have flipped to `open` — but it is **not implementable yet**: the lock's
  floor clause rows deliberately compile to a *reduced row* (identity+severity,
  `sdk/src/contract.ts:18`; `clauseRow()` :78-100 drops the node-scope
  predicate argument — max_len's max, allowed_chars's charset, deny's values,
  forbidden_keys's keys). `builtin.rs` cannot project a checkable floor Contract
  from argument-less rows. Filed the missing foundation link
  **LOCK-CLAUSE-PREDICATE-ARGS** (the LOCK-CLAUSE-CHANNELS analogue for args)
  and re-pointed the projection's `blockedBy` at it.
- **Queue — 5 entries, 1 open:** LOCK-CLAUSE-PREDICATE-ARGS (open — carry each
  floor clause's node-scope argument across the SDK→lock seam). Then serial:
  BUILTIN-FLOOR-LOCK-PROJECTION (blockedBy ARGS) → CHECK-LOCK-KIND-ROWS
  (blockedBy PROJECTION — cascade field report). COMMENT-STOCK-SWEEP deferred
  (whole-tree solo; promoted once the chain lands and the queue is otherwise
  empty). PACKAGING-CHANNELS parked (release creds + engine workflow + USPTO).
- **Residue reconciled:** curated-tree comment refs (builtin.rs `packages/`,
  builtin_kind.rs/bundle.rs/main.rs `KIND.md`) are claimed — builtin.rs's by
  the projection, the rest by the sweep; nothing orphaned. Refreshed the
  open-questions curated-trees asymmetry (FLATTEN landed; physical tree
  deletion now an actionable human chore).
- **What's next:** build picks LOCK-CLAUSE-PREDICATE-ARGS; the chain drains
  link by link. Queued **human** chore: the physical `packages/**` + `kinds/**`
  deletion (out of fence) once the projection + sweep clean the last refs.

Plan continues: no — queue reconciled (the projection's stale blocker
corrected, the missing foundation link filed as the one open entry, chain
serialized behind it), inbox empty, delta empty. Building is how the chain
drains.
