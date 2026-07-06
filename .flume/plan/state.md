# Plan state

- **Phase:** reconciled the S3–S7 demolition chain against disk. S3
  (FIXTURES-OFF-IMPORT) shipped (build cb7ed1c / chore 8196180) — behavior-test
  fixtures now set up off golden-lock/emit, so the only live `import::run` callers
  left are tests/temper_toml.rs (96/1014) and main.rs's two scratch branches.
  Promoted S4 (SCRATCH-RETIRE) `blockedBy → open` and **rewrote it** off two disk
  findings.
- **Last shipped:** FIXTURES-OFF-IMPORT / S3 (build cb7ed1c / chore 8196180).
  Verified on disk: no test file carries a live `import::run` call except
  temper_toml.rs; the rest are comments naming the retired producer.
- **This tick — S4 rewrite (two real errors caught):** (1) the old entry ordered
  `write_rollup` deleted "its producer job ended in S3" — **false**: it is
  `drift::emit`'s live lock-writer (the sole producer, S1; drift.rs:31,439), so it
  KEEPS. (2) tests/temper_toml.rs has live `import::run` **and** `emit_manifest`
  calls, both deleted in S4, so it must retire HERE — moved its retire from S5
  (CODEC-RETIRE) into S4, and updated S5's acceptance/notes. Also enumerated the
  true transitive dead set (deleting `run_with_builtins` orphans
  `import_frontmatter_kind`/`collect_declarations`/`carrier_count` → clippy
  `-D warnings`), corrected drifted line numbers, and confirmed the
  `--harness`/session-start rewire is feasible (`resolve_kind_units` already
  discovers members off `harness_root` via `effective_governs`+`discover_kind_files`).
  Added an `inplace_members()` VERIFY caution to S5 (the surviving gate still reads
  it). Inbox empty — nothing to drain.
- **In flight:** one `open` head — SCRATCH-RETIRE (S4). CODEC-RETIRE (S5) →
  INSTALL-FRONT-DOOR (S6) → TEMPER-TOML-ZERO (S7) wait behind it, one tick at a
  time (all touch the shared main.rs/import.rs/compose.rs spine). PACKAGING-CHANNELS
  parked on human release setup.
- **What's next:** build drains S4; an S4 ship unblocks S5 on the following
  reconcile. Human owns PACKAGING-CHANNELS release setup, the USPTO name screen,
  and the genre-fence-format workshop (cascade pilot).

Plan continues: no — S3 shipped, S4 is promoted to `open` and rewritten truthfully
(write_rollup kept, temper_toml.rs retire moved in), the downstream chain is
serialized, and the inbox is drained. Building is how the queue drains from here.
