# Plan state

- Spec derived through: f87cc0c
- Audited through: ff09cee
- Residue swept through: ff09cee
- This tick: Residue sweep (job 4). No earlier job live: inbox empty, spec
  delta empty past f87cc0c, no commit past Audited-through (ff09cee) touches
  src/tests/sdk (18a110a is plan-only). Swept 9e6a6fb/ec2dd42 plus the whole
  tree for uncaptured "one job, one home" residue via a full Explore scan of
  tests/, src/, sdk/. Found and filed TEST-HELPER-DUPES-CONSOLIDATE(rawunit):
  5 near-identical raw-`Unit`-literal constructors (spec_unit
  tests/section_contains.rs:27, decision_unit tests/nested_member.rs:39,
  memory_unit + frontmatter_unit tests/extract_equivalence.rs:78,170,
  unit_from_member tests/agent_kind.rs:60) with no shared home, all building
  the same `Unit { ..., satisfies/satisfies_clauses/published_requirements:
  Vec::new() }` shape. Ruled out as false positives / already-accepted: (1)
  src/kind.rs:918's own spec_unit — sole in-src copy, crate-boundary twin
  pattern already sanctioned (test_support::tmpdir vs common::tmpdir); (2-4)
  slug_charset/req/lock_widget_kind pairs — exactly one copy per side of the
  integration-test crate boundary, same sanctioned twin pattern, not 2+
  copies on one side; (5) feature() — self-documented forced mirror
  (Features's fields are private outside the crate). Vocabulary check
  (genre/floor/seam/kinds-packages) against decisions 0001/0012: all clean or
  already-accepted debt (706139a's deferred comment-only floor mentions,
  session_start.rs's already-tracked kinds/packages fixtures). New entry
  shares tests/common/mod.rs, tests/agent_kind.rs, tests/extract_equivalence.rs
  with the open lockunitbuilders entry — filed blockedBy it, not open, per
  the disjoint-or-serialized rule.
- Queue: TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders) open —
  TEST-HELPER-DUPES-CONSOLIDATE(rawunit) blockedBy lockunitbuilders —
  PACKAGING-CHANNELS parked (unchanged this tick).

Plan continues: no — every input is current: inbox empty, spec delta empty,
ship audit has nothing new past a plan-only commit, residue swept through
HEAD's last code-touching commit with one new entry filed and the rest ruled
clean. Two entries open/blockedBy (pickable in sequence), one parked. Build
takes over.
