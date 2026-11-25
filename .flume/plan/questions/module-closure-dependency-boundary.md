# Where does the fingerprinted module closure stop when the SDK is linked, not installed?

Driver: MODULE-CLOSURE-IS-FINGERPRINTED (0080; `authoring.md`, "The SDK"). "Every local module the run loaded, `node_modules` excluded" is filtered on the realpath. An SDK linked by symlink (`file:`, a workspace, `common::vendor_sdk`) realpaths out of `node_modules`. It then lands as `../sdk/dist/**` rows: gitignored build output whose hashes move on every SDK build. A fresh clone reads `fresh` "unreadable" until it builds. Adopters on a workspace-linked SDK hit this too.

Each candidate fits 0080's words: (a) drop modules outside the harness root, which misses a monorepo's authored `../shared/` helpers; (b) drop modules outside the git tree, or gitignored ones; (c) keep the rows as they are.

Stance: (d). "`node_modules` excluded" means dependencies excluded. The boundary is package resolution, not the path. A module reached through a bare specifier, and the package closure behind it, is a dependency however it is linked. The load hook's resolve step sees the specifier, so it can tell. The objection: this is subtler than a path test, and a package that re-exports authored code would hide it.

Dependents: the example-lock regeneration recipe (`emit_program_runs_the_shipped_example_harness`) would commit `../` rows. `module_closure::without_module_rows` strips them in tests, and it belongs in `tests/it/common`. The human half: the dogfood `.temper/lock.toml` has not been re-emitted since 0080 (stale root `fresh` guidance), and re-emitting it now would commit `../sdk/dist/**` rows.
