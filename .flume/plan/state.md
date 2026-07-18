# Plan state

- Spec derived through: 810da42
- Audited through: 21dbdc0
- Residue swept through: 21dbdc0
- This tick: INBOX — drained refactor capture build-per-kind-scan-rewalk.md
  (observed a4cffee) into SCAN-SHARE-DISCOVERABLE-SET. Verified the claim at
  HEAD (5c9e63c): import.rs unmoved since a4cffee (the glob memo 12fc95e
  landed in kind.rs), so the capture's addresses hold — `collect_glob`
  (import.rs:429) still walks the filesystem per kind via `read_entries`
  (554), using the shared `discoverable` set (`discoverable_paths` 505,
  memoized by `Discovery` 51) only as a membership filter, not the
  enumeration source; the `**` loci re-traverse whole subtrees per kind.
  Consolidation: derive each kind's matches by glob-matching the in-memory
  set, recording file-vs-dir (which `discoverable_paths` drops at 536-538,
  but the WalkBuilder entry carries via file_type) at walk time. Contained to
  import.rs — the set's only readers are its own scan_locus/collect_glob.
  Measure-first bar already met by check_cost.rs; count-pin the read_dir
  re-walk to zero. gate OPEN, no fork blocks it. Capture deleted (git is the
  archive).
- Queue: 3 pending — 1 pickable OPEN (SCAN-SHARE-DISCOVERABLE-SET), 2 parked
  on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open
  forks: (multi-harness-projection), (lazy-grounds).

Plan continues: yes — post-ship reconciliation of window 21dbdc0..HEAD (the
glob-memo build 12fc95e shipped as 5c9e63c, unaudited/unswept; cursors still
at 21dbdc0). One job per tick — inbox took precedence this tick.
