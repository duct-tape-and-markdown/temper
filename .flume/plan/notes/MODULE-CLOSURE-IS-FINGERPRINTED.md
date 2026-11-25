# MODULE-CLOSURE-IS-FINGERPRINTED shipped; vendored-SDK rows are a live adopter question

Mechanism: `run_sdk_program` passes `node --import data:` with a `module.registerHooks`
load hook. The hook records `file:` URLs under `Symbol.for("@dtmd/temper.loaded-modules")`.
`emit.ts` `loadedLocalModules()` realpaths them and drops any path with a `node_modules`
segment. The lock family is `[[declaration.module]]`, `member = "root"`, judged by
`module_stale_from_doc` with the re-emit remedy.

Seam shape: `Declarations.modules` is `Vec<String>`, not a `ModuleRow`. That's because
`seam_bindings_current.rs` lists every type explicitly and sat outside the fence.

Friction an adopter hits as well: an SDK linked by symlink (`file:`, a workspace, or
`common::vendor_sdk`) is fingerprinted through its realpath. That gives `../` rows into
gitignored `dist/`, whose hashes move on every SDK build. A fresh clone then reads
"unreadable" until it builds. The tests strip those rows through one helper,
`module_closure::without_module_rows`. It lives there because `tests/it/common` was
outside the fence, so it should move into `common` later. The committed example lock
carries only its 14 in-harness module rows. The regeneration recipe in
`emit_program_runs_the_shipped_example_harness` (vendor + emit + commit) would now
commit the `../` SDK rows. Hand-stripping them is a bad recipe. One option: exclude
modules outside the harness root, or outside the git tree. That needs a human ruling.

Dogfood: `.temper/lock.toml` was outside the fence and was not re-emitted. Its
root.fresh guidance is stale, and the next emit adds `../sdk/dist/**` rows.
