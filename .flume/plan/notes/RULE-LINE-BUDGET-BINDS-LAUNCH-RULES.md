# Rule line budget binds launch rules: shipped, two ripples

- The commit had to touch more files than the entry named: `sdk/test/builtins.test.ts` (it pinned the rule contract's predicate order), the `builtin_contract_matrix` snapshot (the rule row loses its top-level `extent` cell, because the budget now sits inside a `when`), and `examples/base-harness/.temper/lock.toml` (its engine stamp moves whenever the embedded lock changes). Plan's `files` should predict all three for any edit to a default contract.
- The contract matrix shows only top-level predicates. A clause guarded by `when` is invisible there: the rule row now shows no line budget, even though one still binds. Snapshots of that matrix will read as coverage loss each time a budget is guarded.
- The dogfood `.temper/lock.toml` engine stamp will also drift. It is outside build's writable paths, so a human `chore(harness):` re-emit owes it.
