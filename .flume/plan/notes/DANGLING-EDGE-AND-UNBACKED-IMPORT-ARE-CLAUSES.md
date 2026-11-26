# Dangling edge and unbacked import ship as root clauses

Shipped `resolves` (required) and `import-backed` (advisory) as root predicates,
appended to `rootDefaultContract` in `sdk/src/assembly.ts`. The entry named
`sdk/src/builtins.ts`, but the root default lives in `assembly.ts`.

Watch these:
- **Opt-in through `root_clause`, like `fresh`.** A committed lock already holding
  root rows (rows-or-default) holds no `resolves` row, so `check` reports no route
  or unbacked-import finding until the next `emit`. The only nudge is
  `engine-matches`, which is advisory. That covers this repo's own `.temper/lock.toml`:
  it is outside build's writable paths, so a human re-emit is owed. An authored root
  contract that leaves out `resolves` silences the route verdict in `check`; emit's
  refusal is unchanged.
- **Graph-side rule ids now match the clause labels** (`root.resolves`,
  `root.import-backed`), so standalone `graph::check` callers read the same address.
  The gate rebinds through the new `Diagnostic::under_clause`.
- **The `resolves` cite** is the sub-agents `skills` preload section
  (retrieved 2026-11-26). The frontmatter row was verified verbatim. The
  missing-skill-is-skipped behaviour came only from a fetch summary, so the guidance
  claims only that a name matching nothing delivers nothing.
- **REQUIREMENT-FILL-AND-DANGLING-ARE-CLAUSES** touches the same root-row
  enumerations: `builtin_lock_frozen.rs`, `builtin_lock.rs`, `root_contract.rs`,
  `sdk/test/{contract,emit}.test.ts` and the gauntlet snapshot. Expect a rebase
  through them.
