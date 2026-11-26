# The undeclared refusal needs the typecheck's tool edge first

Built and run, then backed out: emit refusing an undeclared typecheck,
install scaffolding it (harness.ts tool + `typecheck:`, `tsconfig.json`,
`typescript` + `@types/node` devDeps at sdk/package.json's ranges, and
`dependency_resolves` requiring `.temper/node_modules/.bin/tsc`), and a
`wire_sdk_harness` helper that splices a `fixture-tsc` tool and its
declaration into each program, with a stub `.bin/tsc`. 14 tests went red.

Blocker: every harness must now compose a tool that nothing on the
reference graph reaches. `tool.degree` (advisory) fires on every fixture,
every fresh install and the shipped example. The example's
advisories-denied check
(`the_shipped_example_harness_passes_its_own_check_with_advisories_denied`)
cannot pass. tool.degree's own guidance and 0104 count the typecheck as a
runner that reaches its tool by an edge, but the root has no edge source:
`TypecheckRow` is seam-inbound only, and `fold_runs` and `resolved_edges`
range only over kind members. Plan should slice a prerequisite: the lock
carries the typecheck as a root runs edge (for example a `RunsRow` with
`member = "root"` that gate turns into a `ResolvedEdge`, beside the
mention edges). Then this entry lands as above.

Other fallout to expect when it does:
- builtin_lock_frozen and bundle (kind-tool.md) compare memberless
  emits; they must drop the fixture typecheck's rows or declare it.
- tool_kind tests list tools and findings; `tool.unlocked` also fires
  on the fixture tool when there is no lockfile.
- Programs with no `harness({`, such as raw seam writers, skip the
  splice.
- Two install tests write their own `harness.ts`.

Landed here: the typecheck resolved its command against a relative
`--into` after the cwd moved (`temper emit --into examples/...` failed to
spawn). Also, the example's kinds.ts carried three `registration` type
errors on embedded kinds, now fixed, and it gained a `tsconfig.json` and
devDeps.
