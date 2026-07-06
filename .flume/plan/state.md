# Plan state

- **Phase:** reconciled the S5–S7 demolition chain against disk after S4 shipped.
  Promoted **CODEC-RETIRE (S5)** `blockedBy → open` and **rewrote it** off five
  disk findings — the old entry was stale in three ways that would have broken
  the tick.
- **Last shipped:** SCRATCH-RETIRE / S4 (build 00d6025 / chore 82c494e). Verified
  on disk: `import::run`/`run_with_builtins`/`emit_manifest` are gone,
  tests/temper_toml.rs retired, `--harness`/session-start gate the harness root
  directly (no scratch copy tree). The surviving `[[member]]` codec callers are
  now only `init`/`lift`.
- **This tick — S5 rewrite (three real errors caught):** (1) **drifted line
  numbers** — `init`/`lift` are import.rs:156/239 (entry said 254/337),
  `Command::Init` is main.rs:255 (said 256). (2) **`member_corpus()` is already
  dead** — zero callers, S2 moved the gate onto the lock; it retires as dead code,
  not a live-corpus reader. (3) **missing blast radius** — `members()` is still
  read at main.rs:899 (the `declared` check's members arm) and `import::init` is
  driven by a drift.rs unit test (drift.rs:1447); both break when the symbols die,
  so main.rs:899 and drift.rs are now in `files`, plus the transitive dead set in
  import.rs (`scan_inplace_members`/`rel_slash`/`load_manifest`). Confirmed the
  in-place READ path (`parse_inplace_member`/`inplace_members()`) must **stay** —
  the gate still reads it (main.rs 470/900/943/1029); S7 retires it with
  `AuthorLayer::load`. Inbox empty — nothing to drain; no fork moved.
- **In flight:** one `open` head — CODEC-RETIRE (S5). INSTALL-FRONT-DOOR (S6) →
  TEMPER-TOML-ZERO (S7) wait behind it, one tick at a time (all touch the shared
  compose.rs/main.rs spine). PACKAGING-CHANNELS parked on human release setup.
- **What's next:** build drains S5; an S5 ship unblocks S6 on the following
  reconcile (its line refs will need a refresh then). Human owns
  PACKAGING-CHANNELS release setup, the USPTO name screen, and the
  genre-fence-format workshop (cascade pilot).

Plan continues: no — S4 shipped, S5 is promoted to `open` and rewritten truthfully
(line numbers fixed, member_corpus named dead, main.rs:899 + drift.rs blast radius
added), the downstream chain is serialized, and the inbox is drained. Building is
how the queue drains from here.
