# Plan state

- Spec derived through: a9f7b9e
- Audited through: 77d590e
- Residue swept through: 77d590e
- This tick: Post-ship reconciliation, window 3d13eb4..HEAD. Its one
  src/tests/sdk commit is 3611335 — MANIFEST-ADAPTER-READ: the JSON manifest
  adapter's read face (json_manifest.rs, a serde_json peer to frontmatter.rs;
  `Manifest::read`/`read_kind` reuse `import::discover_kind_files` — no second
  discovery surface), verified on disk. Audit: the entry shipped and is
  dropped from pending; its downstream blocker cleared, so HOOK-KIND flips
  `blockedBy MANIFEST-ADAPTER-READ` → `open`. As the first manifest kind it
  inherits the read-path dispatch 3611335 deferred: `src/main.rs` added to
  HOOK-KIND's edit list (route a `Fields`+`collection_address` kind through
  `Manifest::read_kind`, not the frontmatter loader). MANIFEST-WRITE-SIDE
  stays parked — json_manifest.rs now on disk clears its entry-refs blocker,
  but phase-1 kinds are unshipped and it is still a placeholder needing
  re-scope; reason refreshed. PACKAGING-CHANNELS parked, untouched.
  Sweep: 3611335 is corpus-sanctioned (0021, representation.md "Reach";
  manifests are frontmatter's peer) and subtractive over discovery — no new
  residue. It opened `src/extract.rs` (hunks 25/912/1108) but left the
  floor-mention rider (196-198) as unchanged context → undischarged, carried.
  The vocabulary rider's `extract.rs:1153/1188` "law 5" reclassified: they
  are `.to_string()` decision-fixture strings (now 1227/1262), not doc
  comments — moved to the excluded-fixture class alongside kind.rs. Other
  riders re-verified on their untouched files, stamps to 77d590e. Both
  cursors advance.
- Queue: HOOK-KIND (open, next) → MCP-SERVER-KIND (blockedBy HOOK-KIND) →
  MANIFEST-WRITE-SIDE (parked, phase 2) → PACKAGING-CHANNELS (parked).
  Disjoint: only the head is `open`.

Plan continues: no — inbox empty, no specs delta past a9f7b9e,
reconciliation done and both audit/residue cursors at HEAD. HOOK-KIND is
`open` for build to pick up.
