# Plan state

- **Phase:** reconcile. HEAD 9c29274.
- **Last shipped:** SCAN-QUALIFIED-IDENTITY (build 7ae7240, chore 61e4e6f) —
  import/drift thread the qualified kind, no bare re-resolution; the `builtin_kind`
  enumeration test derives from the `kinds/` tree. Re-verified on disk.
- **This tick:** drained the inbox — routed REGISTERED-KIND-SHADOWS-EMBEDDED into
  pending as a new **open** entry (clean cite: `15-kinds.md`, "Decision: kind
  identity carries a provider axis", now carrying "an explicit registration owns
  its bare name outright"). Verified both symptoms on disk: symptom A is the
  `builtin_names.contains` short-circuit (import.rs:146) preempting a registered
  bare-`memory` kind; symptom B is an empty `ArrayOfTables` vanishing on round-trip
  (write_rollup, import.rs:641). Both loci + the two re-pinned tests live in
  `src/import.rs` alone — single file, only open entry, parallel-safe. Updated
  MEMORY-KIND (its file commit now waits on this fix first) and the open-questions
  bootstrap-fence datum. No other entry moved; all cites resolve.
- **Operational note (accepted, not queued):** the 17 `requirement.dangling`
  session-start findings are a **stale installed binary** — `cargo install --path .`
  clears them; the freshly-built binary's `temper check .temper` is clean.
- **Pickable now:** REGISTERED-KIND-SHADOWS-EMBEDDED (open, sole). Parked (human
  action): MEMORY-KIND (this fix, then file commit), PACKAGING-CHANNELS (release
  creds), COMMUNITY-DOCS (fence-widen + private reporting). Deferred (no consumer):
  EXTRACTION-VOCAB-GAPS, AGENT-KIND.

Plan continues: no — inbox drained, queue reconciled, one open entry ready; hand
to build to drain it.
