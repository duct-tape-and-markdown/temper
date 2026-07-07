# Plan state

- Spec derived through: 5945405
- Audited through: 6ce4738
- Residue swept through: aa9a2e3
- This tick: residue sweep 32f0c32→HEAD (the GENRE-FOLD window). No fileable
  class. All surviving genre residue is comment/citation staleness that rides
  the next entry per file, never standalone: "genre value" vocab
  (extract.rs:172); dead `specs/architecture/*` cites (hash.rs, builtin*.rs,
  bundle.rs, read.rs:104, main.rs:477 — friction-noted). Retained
  `genre.<kind>` fence syntax + the SDK's standalone `GenreValue`
  fold-machinery (corpus-permitted, representation.md "nesting") are not
  residue. Cursor to HEAD. (Only commit past Audited-through is the aa9a2e3
  plan commit — no src/tests/sdk, so ship audit stayed dark.)
- Queue: 2 — MODE-VALUE-VOCABULARY (open, pickable), PACKAGING-CHANNELS
  (parked: still no .github/workflows/release.yml, root package.json still the
  private `temper-flume-harness` manifest, install.rs still pins SDK `^0.0.2`
  vs 0.0.4 published). Paths disjoint.

Plan continues: no — residue swept to HEAD; inbox/spec-delta/ship-audit all
current; MODE-VALUE-VOCABULARY is pickable, build takes over.
