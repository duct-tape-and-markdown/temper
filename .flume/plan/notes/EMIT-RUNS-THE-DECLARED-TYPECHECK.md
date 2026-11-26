# Undeclared typecheck refuses; install scaffolds one; fixtures splice one

All of the entry is done: the refusal (`DriftError::TypecheckUndeclared`), install's
scaffold (a `tsc` tool and the declaration in `harness.ts`, a `tsconfig.json`,
`typescript`/`@types/node` devDeps at sdk/package.json's ranges, and
`dependency_resolves` requiring the anchored `.temper/node_modules/.bin/tsc`), the
example's declaration, and the README snippets.

Human follow-ups:
- The dogfood `.temper/` declares no typecheck, so CI's `emit --frozen` (temper.yml)
  refuses until it does. It needs the tool and the declaration, a `tsconfig.json`,
  and the devDeps in package.json and its lock.
- `docs/example-config.md` and `docs/MIGRATING-0.0.21.md` show `harness({...})` with
  no typecheck. They are outside build's paths.

Observed:
- Fixture splice: `common::typechecked` reroutes each `harness(` through a wrapper.
  The wrapper adds a `{ file: ".temper/fixture-tsc" }` stub tool. It is a file tool
  so it draws no `tool.unlocked`.
- `src/builtin_lock.toml` and `src/bundled_reference/` can no longer come from a
  plain emit, because every emit now carries a typecheck. The tests drop the
  fixture's rows (`common::without_fixture_typecheck_rows`) and its Commands line in
  kind-tool.md. The lock header's "re-run that emit" recipe now needs the same strip.
- Fixed a pre-existing race. The relative-workspace pin read the cwd without
  `CWD_MUTEX`, so it went red under parallel chdir tests.
- The example emit test links a real tsc on unix only, and Windows has no CI test
  lane.

Still open (from the prior tick):
- The program runs before tsc. The spec says the typecheck runs "before running the
  program".
- There is no ancestor walk-up for the bin.
- RUNNERS-REACH-THE-TEMPER-TOOL-BY-EDGE should share `drift::typecheck_command`.
