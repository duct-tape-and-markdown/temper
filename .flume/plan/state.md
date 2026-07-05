# Plan state

- **Phase:** demolition-wave draining, one link left. HEAD 8de7a0d (+ this plan
  commit). Inbox empty; no corpus change since last reconcile.
- **Last shipped:** KIND-PACKAGE-PARSE-RETIRE (build ce00d60 / chore 8de7a0d).
  Verified on disk: `src/kind.rs:49` + `src/builtin_kind.rs:6` state there is no
  `KIND.md`/`PACKAGE.md` file format to parse — kinds and floors are compiled Rust
  data (`builtin_kind`, `builtin`); remaining `KIND.md`/`PACKAGE.md` strings are
  curated-source comments and `include_str!` embeds, not a live parse.
- **This tick:** unblocked **EXPLAIN-UNIFY** — its predecessor
  KIND-PACKAGE-PARSE-RETIRE shipped and its fork `(explain-target-disambiguation)`
  is RESOLVED, so it flips `blockedBy` → `open`. Reconciled its blast radius: the
  read-verb CLI split (`why`/`impact`/`context`/`requirements`) is already gone from
  main.rs's `Command` enum (CLI-COLLAPSE); the four traversals survive as the
  library engine in `src/read.rs`, and `tests/read_verbs.rs` already drives it
  directly awaiting the `explain` reframe — the entry's files (main.rs/read.rs/
  tests) are truthful.
- **In flight:** one pickable `open` head — EXPLAIN-UNIFY (terminal leaf, blocks
  nothing). PACKAGING-CHANNELS parked on human release creds.
- **What's next:** build drains EXPLAIN-UNIFY; the demolition wave is then fully
  landed and the pending queue is PACKAGING-CHANNELS (parked) only. Session half
  still open (human-hand, not plan's): retire the transitional temper.toml→lock
  producer in favor of the SDK `harness.ts` producing the dogfood lock.

Plan continues: no — queue reconciled to the ship, one pickable `open` head exists
(EXPLAIN-UNIFY), inbox empty. Building drains it now.
