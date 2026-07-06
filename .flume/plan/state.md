# Plan state

- **Phase:** reconciled the S1–S7 chain against disk; S1 shipped, unblocked S2,
  and routed the inbox genre-unship ruling into a disjoint sdk/ entry.
- **Last shipped:** EMIT-PAYLOAD-SEAM / S1 (build 911cc45 / chore 1c382b8) — the
  engine runs the SDK program and is the sole compiler of the seam (verified on
  disk: 911cc45 touched sdk/ + src/drift.rs/import.rs/main.rs).
- **This tick:** S1 shipped, so its successor CHECK-READS-LOCK-GOVERNS (S2) is
  promoted from `blockedBy EMIT-PAYLOAD-SEAM` to `open` — re-verified its
  check-path symbols still exist (surface_units, skill_rule_corpus,
  live_extract_inplace, session_start_diagnostics, scratch_surface, governs_root/
  glob, discover_kind_units/builtin). Drained the inbox: filed John's genre-unship
  ruling as UNSHIP-PRESCRIBED-GENRES (`open`, per 15-kinds' genre Decision) —
  delete the prescribed `decision`/`law`/`bound` constructors + `Alternative` from
  the SDK, keep the generic `genre()`/`GenreValue`/`genreValue` mechanism the
  pilot consumes; verified zero call sites outside genres.ts/index.ts. It lives in
  sdk/, disjoint from the S2+ Rust spine, so parallel-safe. S3–S7 + PACKAGING
  unchanged.
- **In flight:** two `open` heads — CHECK-READS-LOCK-GOVERNS (src/main.rs,
  src/check.rs) and UNSHIP-PRESCRIBED-GENRES (sdk/src). Disjoint file sets, safe to
  fan out. S3–S7 wait behind S2, one tick at a time.
- **What's next:** build drains both heads; a CHECK-READS-LOCK-GOVERNS ship
  unblocks S3 (FIXTURES-OFF-IMPORT) on the following reconcile. Human owns the
  release creds (PACKAGING-CHANNELS), the USPTO name screen, and the
  genre-fence-format workshop.

Plan continues: no — the inbox is drained, S2 is unblocked to `open`, the genre
ruling is filed as a disjoint parallel-safe `open` entry, and the rest is
serialized behind S2. Building is how the queue drains from here.
