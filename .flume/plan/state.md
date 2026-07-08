# Plan state

- Spec derived through: f87cc0c
- Audited through: f6ec58f
- Residue swept through: 99533af
- This tick: Residue sweep (job 4), ff09cee..99533af. Checked: (1) test-builder
  duplication beyond tests/common — remaining per-file write_* helpers
  (agent_kind, command_kind, coverage_note, memory_gate, graph, install,
  requirement_roster) differ in signature/content per real kind, not
  duplicates of one job; (2) stale pre-rename vocabulary (activation→
  registration, apply→emit) — none found in src/sdk, only unrelated "apply"
  usages (json_splice, kind::Primitive::apply, guidance prose); (3) engine
  agents-md.memory kind residue — confirmed dropped by BUILTIN-KIND-FLATTEN
  (3712d24), matches open question's spec-faithful default, no code residue;
  (4) kinds/+packages/ retirement — sole survivor is tests/session_start.rs's
  KIND.md/PACKAGE.md fixtures, already tracked as accepted debt in
  open-questions.md ("Kept on purpose"), not a fresh finding. No new residue
  fileable this tick; pending.json and open-questions.md unchanged.
- Queue: PACKAGING-CHANNELS parked (unchanged) — no open entries.

Plan continues: yes — every input (inbox, spec delta, ship audit, residue
sweep) is now current through HEAD; next tick's live input is job 5, the
quiet closing pass.
