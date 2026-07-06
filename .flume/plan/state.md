# Plan state

- **Phase:** reconciled the S1–S7 chain against disk. S2 (CHECK-READS-LOCK-GOVERNS)
  and UNSHIP-PRESCRIBED-GENRES both shipped — promoted S3 to `open`, made S4's
  main.rs description truthful about S2's residual, drained the inbox.
- **Last shipped:** CHECK-READS-LOCK-GOVERNS / S2 (build e5db3e9 / chore 526a9ce)
  + UNSHIP-PRESCRIBED-GENRES (build 7ab58e7 / chore 526a9ce). Verified on disk:
  `gate` reads `drift::read_declarations` + walks each kind's `governs` locus
  (main.rs:442/615, `effective_governs`, `resolve_kind_units`); `check::surface_units`,
  `skill_rule_corpus`, `live_extract_inplace` are gone; no-lock falls back to each
  kind's embedded `governs` (the built-in lock). SDK's `decision`/`law`/`bound`/
  `Alternative` are gone; generic `genre()`/`GenreValue`/`genreValue` survive.
- **This tick:** S2's `blockedBy` is discharged, so FIXTURES-OFF-IMPORT (S3) is
  promoted `blockedBy → open` — re-verified its targets intact (test files still
  set up via `import::run` at coverage.rs:76/graph.rs:96/gate_fail_loud.rs:133/
  lock_declaration_rows.rs:93; schema.rs:47 still `pub mod interchange;`; the four
  retire files exist; contract_fixtures.rs is interchange's sole caller). Found S2
  shipped its lock-read core but left the `Check --harness` (main.rs:286) and
  session-start (main.rs:583) branches on the transitional `scratch_surface` +
  `import::run` producer — that residual is owned by S4 (SCRATCH-RETIRE), whose
  main.rs description was rewritten to rewire those two branches to gate the
  harness directly (built-in lock) before deleting `scratch_surface`. Inbox drained:
  PACKAGING-CHANNELS' parked reason re-worded off the stale creds framing (no
  marketplace/signing creds exist; only notarization + USPTO remain, decide-at-release).
- **In flight:** one `open` head — FIXTURES-OFF-IMPORT (S3, tests/ + src/schema.rs,
  disjoint from all other entries). S4→S7 wait behind it, one tick at a time;
  PACKAGING-CHANNELS parked on human release setup.
- **What's next:** build drains S3; an S3 ship unblocks S4 (SCRATCH-RETIRE) on the
  following reconcile. Human owns PACKAGING-CHANNELS release setup, the USPTO name
  screen, and the genre-fence-format workshop (cascade pilot).

Plan continues: no — S2/genre-unship shipped, S3 is unblocked to `open`, S4's
residual is folded truthfully into its description, the inbox is drained, and the
rest is serialized behind S3. Building is how the queue drains from here.
