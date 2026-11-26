# Typecheck runs when declared; the undeclared refusal waits on install's scaffold

Landed: `harness({ typecheck: { tool, project: input(...) } })` lowers to a
seam-inbound `TypecheckRow`; `emit_program` runs the tool's command with
`--noEmit --project <abs>` and refuses with tsc's diagnostics
(`DriftError::TypecheckFailed`); the project is a root-owned `input`. Tests
1 and 3 of tests[] pass against a real tsc (`common::wire_typescript_compiler`).

Next, one coherent step: refuse an undeclared typecheck (test 2). Not taken
here because it breaks `install`: its skeleton `harness.ts`
(`install::harness_entry_source`) declares none and install runs
`emit_program`. 0098's Consequences need building in the same commit:
scaffold the declaration, a `tsconfig.json`, and `typescript` in
`ensure_package_json` (`dependency_resolves` checks it too). Then every
emitting fixture declares one: ~56 `wire_sdk_harness` calls plus install's.
A real tsc costs ~1s per emit and fixtures may not typecheck, so a fixture
`.bin/tsc` stub that exits 0 is likely the shared helper.

Open for plan:
- Ordering: the declaration lives in the program, so it is known only after
  the program runs. The typecheck runs after that run and before any compile
  or write. A runtime throw from a mistyped program surfaces before tsc's
  diagnostics. The spec reads "before running the program".
- A typecheck→tool edge is not on the reference graph, so a tool only the
  typecheck runs still draws advisory `tool.degree`. Root has no edge facts
  (`compose::fold_runs`).
- `{ package, bin }` runs `<workspace>/node_modules/.bin/<bin>` (`.cmd` on
  Windows), with no walk up to ancestors. RUNNERS-REACH-THE-TEMPER-TOOL-BY-EDGE
  renders the same anchored path and should share
  `drift::typecheck_command`.
- The dogfood `.temper/` (human) must declare a typecheck before the refusal
  lands.
