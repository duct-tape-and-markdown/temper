# Plan state

- Spec derived through: cd7135b
- Audited through: 5389103
- Residue swept through: fe0c502
- This tick: Residue sweep (job 4). Re-verified the prior tick's forecast:
  every remaining `floor` mention across src/tests/sdk (builtin.rs, kind.rs,
  contract.rs, extract.rs, read.rs, drift.rs, roster.rs, schema.rs,
  main.rs, engine.rs, builtin_lock.rs, and test fn/variable names in
  coverage.rs, rule_contract.rs, contract_template.rs, session_start.rs,
  cli.rs, graph.rs, lock_declaration_rows.rs, requirement_roster.rs,
  nested_member.rs, gate_fail_loud.rs, display_rule.rs, memory_gate.rs,
  agent_kind.rs, acceptance.rs; sdk/src/assembly.ts, contract.ts,
  declarations.ts) is comment/doc-comment/identifier staleness with no
  live production symbol attached — 706139a's own commit body already
  carved these out ("ride whichever entry next opens those files for a
  real reason"), matching spec-system's comment/citation staleness
  exception; nothing fileable this class. Checked the other two commits
  merged since 77b2eb9 for fresh residue: 4ed4027's retired symbols
  (member_name/description_trigger_value/relative_to_member_module) are
  fully gone from src/tests/sdk; 39079e3 left no dead custom-kind
  fixtures — the only surviving .temper/kinds//packages KIND.md/PACKAGE.md
  authoring is the already-accepted tests/session_start.rs debt recorded
  in open-questions "Kept on purpose". `cargo clippy --all-targets` is
  clean (no dead-code signal). Cursor advances to HEAD; pending.json and
  open-questions.md unchanged.
- Queue: unchanged — RETIRE-POSTURE-VOCABULARY-FOR-ENFORCEMENT-MODE open
  and pickable; RETIRE-OWN-PATH-MACHINERY blockedBy it; PACKAGING-CHANNELS
  parked.

Plan continues: yes — job 5 (quiet closing pass) is next: all four prior
inputs are current (empty inbox, no spec delta, no unaudited src commits,
residue cursor now at HEAD).
