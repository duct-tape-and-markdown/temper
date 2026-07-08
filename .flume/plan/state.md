# Plan state

- Spec derived through: f87cc0c
- Audited through: ff09cee
- Residue swept through: 63f3c61
- This tick: Ship audit (job 3). Inbox empty, spec delta empty; commits past
  the old Audited-through (63f3c61) touching src/tests/sdk: 9e6a6fb (in-src
  tmpdir driftcoveragenote consolidation), ec2dd42 (6-builder recordwriters
  slice), ff09cee (build's ship commit, pending.json-only). Verified on disk:
  both tmpdir copies (drift.rs, coverage_note.rs) gone, common::tmpdir used;
  the 6 recordwriters builders (write_requirements/write_rule/
  write_retired_manifest/write_lock/retired_manifest_name/clean_skill) live
  once in tests/common, call sites migrated. Both shipped entries already
  absent from pending.json (ff09cee). Re-tested the one stale gate this
  unblocks: TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders) was blockedBy
  recordwriters — now open. Re-verified all 9 duplicate builders it names
  (surface_unit, skill_member, rule_member, rule_kind_facts, skill_kind_facts,
  skill_surface_unit, findings_for, author_rule_satisfies, requirement) plus
  both check_harness shapes still live at their (ec2dd42-shifted) line
  numbers; rewrote the entry's file descriptions with current lines rather
  than patch stale ones. PACKAGING-CHANNELS re-checked: still no
  `.github/workflows/release.yml`, root package.json still the private flume
  manifest — parked reason holds unchanged.
- Queue: TEST-HELPER-DUPES-CONSOLIDATE(lockunitbuilders) open (unblocked) —
  PACKAGING-CHANNELS parked (re-verified unchanged).

Plan continues: yes — residue sweep (job 4) is still due: Residue swept
through (63f3c61) trails Audited through (ff09cee), and 9e6a6fb/ec2dd42 are
unswept src/tests commits.
