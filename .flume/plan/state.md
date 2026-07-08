# Plan state

- Spec derived through: f87cc0c
- Audited through: 63f3c61
- Residue swept through: e50d082
- This tick: Ship audit (job 3), e50d082..63f3c61. Build shipped and a
  chore(flume) commit (63f3c61) already removed the four picked entries from
  pending.json: PATH-SEP-NORMALIZE-CONSOLIDATE (366e35a), SDK-SORT-COMPARATOR-
  CONSOLIDATE (fc8926c), IN-SRC-TMPDIR-HELPER-CONSOLIDATE (a9942d4),
  EXTRACT-FENCE-SCAN-CONSOLIDATE (802529d). Verified each on disk, not just
  the log: document.rs/coverage_note.rs call drift::to_lock_path;
  declarations.ts/emit.ts's 7 sort sites call compareStrings; extract.rs's
  three prose scans call the new track_fence; 8 of 10 tmpdir(label) sites
  call test_support::tmpdir (main.rs via its own `mod test_support;` per the
  filed friction). `cargo test`, `cargo clippy --all-targets -- -D warnings`,
  and `pnpm --dir sdk test` all green. Re-checked the surviving 3 entries'
  gate conditions: recordwriters/lockunitbuilders untouched by this batch;
  PACKAGING-CHANNELS's parked reason re-verified true (no
  `.github/workflows/release.yml`, root `package.json` still the private
  flume manifest). a9942d4 left drift.rs's and coverage_note.rs's own
  `tmpdir(label)` copies unmerged (disjoint-or-serialize with the
  then-open PATH-SEP-NORMALIZE-CONSOLIDATE) — now unblocked, a residue-sweep
  candidate, not filed this tick (one job per tick).
- Queue: TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) open — TEST-HELPER-
  DUPES-CONSOLIDATE(lockunitbuilders) blockedBy recordwriters —
  PACKAGING-CHANNELS parked (condition unchanged).

Plan continues: yes — residue sweep (job 4) is next: Residue swept through
(e50d082) trails HEAD (63f3c61), and drift.rs/coverage_note.rs's deferred
tmpdir(label) copies are a known, now-unblocked candidate.
