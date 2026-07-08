# Plan state

- Spec derived through: cd7135b
- Audited through: a112dbe
- Residue swept through: d3c2805
- This tick: Residue sweep (job 4) — swept `src/`, `tests/`, `sdk/` against the
  corpus's demolition language (own_path, import_hash, Template.leaves/
  collections, the retired KIND.md/PACKAGE.md curated-tree format). Every
  live symbol found is already operationalized: own_path by RETIRE-OWN-PATH-
  MACHINERY, import_hash and Template.leaves/collections by RETIRE-DEAD-
  DECLARED-SURFACE. `CustomKind` (src/kind.rs) checked against
  representation.md's "kind" section — it is lock-declared Rust data, not a
  parsed file format ("there is no kind file format to parse it from"), so
  it is not residue. tests/session_start.rs's `+++`-format KIND.md/
  PACKAGE.md fixtures remain the one named accepted debt (open-questions.md,
  "Kept on purpose"), explicitly deferred to the next ship audit — not this
  job. No spec-delta or inbox content this tick (both empty); no src/tests/
  sdk commits past `Audited through` (job 3 stays quiet). Clean sweep — no
  new entries filed; cursor advances to HEAD.
- Queue: 6 — CI-DOCUMENTED-TWO-LINE-JOB, SDK-VERSION-LOCKSTEP,
  RETIRE-DEAD-DECLARED-SURFACE (open, pickable, disjoint); INSTALL-WHOLE-
  CONVERSION → RETIRE-OWN-PATH-MACHINERY (chained behind SDK-VERSION-
  LOCKSTEP); PACKAGING-CHANNELS (parked).

Plan continues: yes — job 5 (quiet closing pass) is next: all four inputs
above are now current (inbox empty, spec delta empty, ship audit quiet,
residue swept to HEAD), so the next tick re-verifies queue disjointness and
every gate reason, then hands off.
