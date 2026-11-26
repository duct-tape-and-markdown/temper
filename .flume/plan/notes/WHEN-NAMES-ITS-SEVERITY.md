# WHEN-NAMES-ITS-SEVERITY shipped

`when()`'s options are now required, with a required `severity`, the same shape as
`clause()`. All 11 shipped `when` clauses in `sdk/src/builtins.ts` (marketplace
sources, hook handler types, MCP transports) state `severity: "required"`: that is
the value the old default gave them, so `src/builtin_lock.toml` stays byte-identical
(`cargo test --test it builtin_lock` green). The type test is in
`sdk/test/contract.test.ts`, with the `@ts-expect-error` calls wrapped in a closure
that never runs, so only tsc judges them. No other caller in the tree (`.temper/`,
examples) called `when()`.
