# Plan state

- Spec derived through: f87cc0c
- Audited through: f6ec58f
- Residue swept through: ff09cee
- This tick: Ship audit (job 3), cbce31d..f6ec58f. ca5557d (build) consolidated
  the 5 raw-Unit-literal builders (spec_unit, decision_unit, memory_unit,
  frontmatter_unit, unit_from_member) into one tests/common::raw_unit(id,
  frontmatter, body, source_path) — verified on disk: old builder names gone
  from tests/agent_kind.rs, tests/extract_equivalence.rs,
  tests/nested_member.rs, tests/section_contains.rs; raw_unit present in
  tests/common/mod.rs; full `cargo test` green (agent_kind,
  extract_equivalence, nested_member, section_contains and the full suite,
  including session_start). f6ec58f (chore) correctly dropped
  TEST-HELPER-DUPES-CONSOLIDATE(rawunit) from pending.json now that its work
  shipped. Re-verified PACKAGING-CHANNELS still parked: still no
  .github/workflows/release.yml (only temper.yml, a check job), root
  package.json still the private flume manifest (temper-flume-harness),
  sdk/package.json unchanged at 0.0.5 — restamped re-verified at f6ec58f.
- Queue: PACKAGING-CHANNELS parked (unchanged this tick) — no open entries.

Plan continues: yes — ship audit now current through HEAD (f6ec58f), but
residue sweep cursor still trails at ff09cee (untouched since b6c613f) and no
open entry remains for build to pick; next tick's live input is the residue
sweep (job 4).
