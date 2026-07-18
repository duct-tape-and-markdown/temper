# Plan state

- Spec derived through: 810da42
- Audited through: 798d2d1
- Residue swept through: 798d2d1
- Posture swept through: absent — rotation never initialized, owed once jobs
  1-3 are quiet
- This tick: INBOX (refactor-captures). Drained
  build-discovery-infallible-result-plumbing.md (observed 8b06146, cut
  0bc0ee9): re-verified every claim on disk at HEAD 1126033 — `ImportError`
  (import.rs:138) has `ReadDir` (142) with no constructor anywhere in src/ or
  tests/ (`read_entries`, its sole builder, retired at 0bc0ee9) and `Write`
  (153) the lone live variant; six discovery fns still return a `Result` no
  arm can `Err` (`discover_builtin` 206, `discover_nested_file` 252,
  `declared_governed_paths` 304, `discover_kind_files` 358,
  `discover_kind_units` 393, `scan_locus` 417); ripple confirmed at
  json_manifest.rs:316/431, main.rs:1353/1372, install.rs:332. Filed
  DISCOVERY-INFALLIBLE-RESULT-COLLAPSE (open, per engineering.md "One job,
  one home") and deleted the capture.
  This is a refile: the prior attempt (648fba5) landed the same entry but was
  reverted by the fence gate — its `files.retire` array named
  "ImportError::ReadDir (src/import.rs:142)", a symbol description, not a
  repo-relative path, so no fence glob matched it. Fixed by dropping
  `files.retire` to `[]` and keeping the ReadDir removal as prose inside the
  src/import.rs edit description (pending-entry.md: retiring a symbol in a
  surviving file is an edit, never a `files.retire` entry). Simulated all
  three afterCommit gates locally (parse, fence-regex, disk-existence/per
  cite) against the rewritten pending.json before committing.
  Parked gates re-tested at 1126033: both hold, unchanged from 04b33de — the
  only commit since (1126033 itself) touched `.flume/**` only, so nothing in
  src/graph.rs or .github/ moved.
- Queue: 3 pending — 1 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE),
  2 parked on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER).
  Open forks: (multi-harness-projection), (lazy-grounds).

Plan continues: yes — post-ship reconciliation is owed next: 798d2d1..HEAD
touched src/ (0bc0ee9) and cursors are still unmoved at 798d2d1; that job
ranks below this tick's inbox job and was untouched by it.
