# Plan state

- Spec derived through: f87cc0c
- Audited through: 95689ae
- Residue swept through: 3a2bed1
- This tick: Residue sweep 38c01c5..HEAD (3a2bed1). Only two commits in
  range touched src/tests/sdk: 1894af9 (tmpdir/fixture consolidation) and
  7329585 (plural helper, reporter.rs) — both already ship-audited clean
  last tick. Re-checked anyway: the plural sweep is complete (only
  `display::plural`'s own definition matches the suffix-ternary shape;
  read.rs's two `len() == 1` hits are noun-swap plurals, a different job).
  The tmpdir sweep left one straggler its own commit body flagged but no
  entry captured: `src/kind.rs:1079`'s `surface_tmpdir` is byte-identical
  (body + doc comment) to every sibling in-src module's `tmpdir(label)`
  helper, just named differently — missed by the shipped entry's `fn
  tmpdir` grep. Filed TMPDIR-HELPER-CONSOLIDATE(kindrs), open, disjoint
  from every other entry (touches only src/kind.rs). Checked the
  `tests/session_start.rs` +++-format accepted-debt note
  (open-questions.md, "Kept on purpose"): 1894af9 touched that file but
  only its tmpdir scaffolding, not the flagged KIND.md/PACKAGE.md
  fixtures — debt still stands, not yet "touched" in the sense the note
  means.
- Queue: TMPDIR-HELPER-CONSOLIDATE(mainrs), TMPDIR-HELPER-CONSOLIDATE
  (kindrs), FRONTMATTER-DELIMITER-SCAN-CONSOLIDATE, TEST-FIXTURE-HELPERS-
  CONSOLIDATE all open and pairwise disjoint, pickable in parallel —
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) (blockedBy
  test-fixture-helpers, still unshipped) — TEST-HELPER-DUPES-CONSOLIDATE
  (lockunitbuilders) (blockedBy recordwriters) — PACKAGING-CHANNELS
  (parked, condition unchanged).

Plan continues: yes — quiet closing pass is next: inbox empty, spec delta
empty, ship audit current (95689ae, nothing past it touches src/tests/sdk),
residue swept through HEAD.
