# Plan state

- **Phase:** derived-lock chain draining. Inbox empty; spec delta empty (no
  `specs/` commits since 1a71b06).
- **Last shipped (6f09395):** LOCK-CLAUSE-CHANNELS (eca35e6 — clause guidance+cite
  now ride the SDK→lock seam and the derived `builtin_lock.toml`) and
  BUILTIN-LOCK-FROZEN-LANE (7decfd6 — a CI test re-derives the built-in lock from
  the SDK module and byte-compares it against the embed). The seam foundation the
  floor projection needs is in place.
- **Queue — 5 entries, 1 open:** BUILTIN-KIND-FLATTEN (open — flatten kind
  identity onto the lock's bare row labels, drop agents-md.memory + the
  provider-qualified/bare-name machinery; closes (builtin-workspace-qualified-key)).
  Then serial: BUILTIN-FLOOR-LOCK-PROJECTION (blockedBy FLATTEN — floors project
  the lock's clause rows) → CHECK-LOCK-KIND-ROWS (blockedBy PROJECTION — check
  reads the committed lock's custom kind rows; cascade field report).
  COMMENT-STOCK-SWEEP deferred (whole-tree solo, promoted once the chain lands and
  the queue is otherwise empty). PACKAGING-CHANNELS parked (release creds + engine
  workflow + USPTO screen).
- **Reconcile this tick:** the two chain-foundation entries shipped and dropped
  from the queue (verified on disk: `builtin_lock.toml` clause rows carry
  guidance+cite, `builtin_lock_frozen.rs` exists). Verified BUILTIN-KIND-FLATTEN
  still truthful (BUILTIN_KINDS roster, agents_md_memory, resolve_bare,
  AmbiguousKind, `claude-code.*` qualified labels all live in `src/`). Residue
  sweep: the package-noun/provider residue in install.rs/main.rs falls inside the
  chain's file lists; comment-only staleness in untouched files (coverage_note.rs
  "qualified identity") rides COMMENT-STOCK-SWEEP — nothing orphaned. Freshened
  FLOOR-PROJECTION's note (LOCK-CLAUSE-CHANNELS shipped) and the open-questions
  curated-trees line (CURATED-TREES-RETIRE shipped).
- **What's next:** build picks BUILTIN-KIND-FLATTEN; the chain drains link by
  link. Queued **human** chore: the physical `packages/**` + `kinds/**` deletion
  (out of fence) once FLATTEN lands.

Plan continues: no — queue reconciled (shipped foundation dropped, one open entry
truthful and pickable, chain serialized behind it), inbox empty, delta empty.
Building is how the chain drains.
