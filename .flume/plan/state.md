# Plan state

- Spec derived through: f87cc0c
- Audited through: cbce31d
- Residue swept through: ff09cee
- This tick: Ship audit (job 3), ff09cee..cbce31d. 878fb2d (build)
  consolidated 9 other copy-pasted test builders (surface_unit, skill_member,
  rule_member, rule_kind_facts, skill_kind_facts, skill_surface_unit,
  findings_for, author_rule_satisfies, requirement) into tests/common — a
  disjoint set from rawunit's 5 named raw-Unit-literal constructors,
  confirmed unchanged on disk (spec_unit, decision_unit, memory_unit,
  frontmatter_unit, unit_from_member all still present, same shapes, only
  shifted line numbers in extract_equivalence.rs/agent_kind.rs, since 878fb2d
  trimmed earlier lines in those two files). cbce31d shipped lockunitbuilders
  (pending.json-only commit). Unblocked TEST-HELPER-DUPES-CONSOLIDATE(rawunit)
  from blockedBy lockunitbuilders to open, refreshed its line numbers.
  Re-verified PACKAGING-CHANNELS still parked: no
  .github/workflows/release.yml, package.json still the private flume
  manifest — restamped re-verified at cbce31d.
- Queue: TEST-HELPER-DUPES-CONSOLIDATE(rawunit) open (pickable) —
  PACKAGING-CHANNELS parked (unchanged this tick).

Plan continues: no — inbox empty, spec delta empty past f87cc0c, ship audit
now current through HEAD (cbce31d). Residue sweep cursor still trails at
ff09cee (untouched this tick, carried forward) but no live job precedes it
next tick beyond resuming there. One open entry pickable; build takes over.
