# Plan state

- **Phase:** reconciled after GATE-CONTRACT-FROM-LOCK (S7a) shipped. Verified on
  disk: `compose::effective(&declarations.clauses, …)` (main.rs:697, compose.rs:984)
  now composes each kind's contract from the lock's `ClauseRow`, and the gate's own
  comment marks the `temper.toml` `[kind.*]` layer `load_layer` still parses as
  vestigial-for-contracts. So the terminal **TEMPER-TOML-ZERO** premise holds — the
  reader feeds no contract; it is safe to pull.
- **Last shipped:** GATE-CONTRACT-FROM-LOCK / S7a (build dd087a5 / chore 08173bd).
- **This tick:** promoted **TEMPER-TOML-ZERO** to `open`; refreshed its
  compose.rs/main.rs line refs (the contract-from-lock commit shifted both);
  **dropped `src/schema.rs`** from its fence (zero temper.toml refs on disk now).
  20 files (9 src + 11 test). Inbox empty. No fork moved.
- **In flight:** one `open` entry — TEMPER-TOML-ZERO (sole pickable, no shared-file
  parallelism concern). PACKAGING-CHANNELS parked on human release setup.
- **What's next:** build drains TEMPER-TOML-ZERO — retire the reader + in-place
  `[[member]]` path, sweep every doc/message/test ref to `rg=0`. That closes the
  `(inplace-lock-producer)` demolition chain (S1–S7). Human owns PACKAGING-CHANNELS
  release setup, the USPTO name screen, and the genre-fence-format workshop.

Plan continues: no — S7a shipped, the terminal is re-scoped truthful and `open`,
the inbox is drained, and no queue change beyond the promotion is warranted.
Building is how the queue drains from here.
