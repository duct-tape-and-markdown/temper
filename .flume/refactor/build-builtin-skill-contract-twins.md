## Surface
One job — resolve the embedded built-in `skill` contract the way the shipped
tool resolves it — spelled twice, identical bodies under two names:
`tests/it/acceptance.rs:35 builtin_skill_contract` and
`tests/it/contract_template.rs:28 skill_builtin`, both
`temper::builtin::contract("skill").expect("the skill floor is embedded")`.
Surfaced by the body-comparison census RENAMED-FIXTURE-COPIES-FOLD-INTO-ONE-HOME's
acceptance runs; neither file is in that entry's fence, so it could not be folded
there.

## Observed at
2e1c9577 (HEAD when observed)

## Suggested consolidation
One `common::builtin_skill_contract()` in `tests/it/common/mod.rs`, beside
`fresh_clause` (the existing "read the shipped default off the embedded lock"
family), reached from both suites.
