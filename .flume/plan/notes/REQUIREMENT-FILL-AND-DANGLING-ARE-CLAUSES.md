# Requirement fill and dangling ship as requirement-kind clauses

`requirementDefaultContract` now has `count({min:1})` (label `requirement.count`) and
`resolves` (label `requirement.resolves`), both at required. No new predicate was minted:
the fill is literally the whole-grain cardinality clause the spec names, and a dangling
`satisfies` is a declared reference that fails to resolve.

Watch these:
- **The requirement kind's clauses bind each requirement's opt-in selection, never the
  kind's population.** `gate::kind_selections` now skips `requirement`. Without that skip,
  `count` judged "at least one requirement exists" and failed every requirement-less tree.
  `coverage::check` is the only judge, and it takes the dialed contract's clauses. The
  first `count` and first `resolves` it finds are the ones it applies.
- **`required: false` lowers the fill finding to Warn after the clause severity is read.**
  A dial that raises `requirement.count` does not escalate an optional requirement. An
  optional unfilled requirement now prints an advisory where it used to print nothing.
  Several tests changed with that.
- **The rule ids changed** from `requirement.unfilled`/`requirement.dangling` to the labels.
  The message dropped the "required requirement" prefix. `examples/base-harness/README.md`
  was updated. The example lock did not need a re-emit: it binds no requirement contract,
  so the embedded default applies.
- `resolves` now has two meanings, a root binding (route walk) and a requirement binding
  (satisfies). Nothing yet fences `resolves` or `count` bound at other kinds, where they
  judge nothing. That is a pre-existing gap for the root-bound family.
