# Plan state

- Spec derived through: 5945405
- Audited through: 6ce4738
- Residue swept through: 32f0c32
- This tick: ship audit. GENRE-FOLD (ccb0724) confirmed shipped on disk —
  tests/genre_leaf.rs retired, tests/nested_member.rs present, no
  GenreValue/GenreCollections/LeafAddress/fold_genres residue,
  EmbeddedMember/MemberAddress + fold_members live. Already drained from the
  queue, so no entry to drop. Re-tested the PACKAGING-CHANNELS parked gate:
  still true — no `.github/workflows/release.yml` (only temper.yml), root
  package.json still the private `temper-flume-harness` manifest, install.rs
  still pins SDK `^0.0.2` (sdk now at 0.0.4) — nothing shipped to unblock it.
  Audited cursor c685a93→6ce4738 (HEAD).
- Queue: 2 entries — MODE-VALUE-VOCABULARY (open, pickable), PACKAGING-CHANNELS
  (parked on release creds). Disjoint: MODE's compose/install/main/builtin_lock/
  sdk/tests vs PACKAGING's release.yml/package.json — no shared path.

Plan continues: yes — residue sweep: Residue swept through (32f0c32) trails
HEAD. GENRE-FOLD left comment-level "genre value" staleness (extract.rs:172,
kind.rs docs) that rides the next entry opening those files; the retained
`genre.<kind>` fence surface syntax is not residue.
