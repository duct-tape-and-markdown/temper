# MODULE-CLOSURE-IS-FINGERPRINTED parked: fence misses the frozen lane

Every real SDK run loads at least `.temper/harness.ts`, so once emit writes
module rows, every lock `emit_program` writes carries them, memberless ones
included. `tests/it/builtin_lock_frozen.rs` byte-compares that lock against
`src/builtin_lock.toml`. It compares everything from `[[declaration.kind]]`
to EOF, and it asserts that the prefix is only the engine stamp. Its comment
says a new family joins its compare. That file is
outside the fence. The embedded lock must not carry module rows: the
stranger gate would then hash a phantom `.temper/harness.ts` in every harness.
Re-file with `tests/it/builtin_lock_frozen.rs` in the fence. Also widen it to
the emit_program suites that read or compare lock text (`emit.rs`,
`lock_declaration_rows.rs`, `install.rs`, `gauntlet.rs`) and to `.temper/lock.toml`.
The dogfood lock moves too.

A design fork plan should settle first. The dogfood (`file:../sdk`), the
test vendoring (`common::vendor_sdk`) and pnpm workspaces all reach packages
through symlinks, so one rule has to decide what "node_modules excluded" means:
- By realpath. GH #61's workspace package stays fingerprinted (0080 wants that). But a symlinked SDK is fingerprinted as `../sdk/dist/**`, which
  is gitignored build output, and its rows are tmp-relative `..` paths in tests.
- By resolved URL before realpath, i.e. any package path. This drops the SDK,
  and it also drops #61's case, which is the reason for the entry.
The corpus does not rank the two. I'd go with realpath plus the dogfood
absorbing it, but this is plan's or the human's call.

Mechanism I verified as viable: the engine owns the preload. It runs
`node --import 'data:text/javascript,...' harness.ts`, and a `module.registerHooks`
load hook (Node >=22.15; CI is on 24) records file: URLs into a
`Symbol.for` global, which emit.ts reads. No sdk/package.json export is needed.
A hook registered from inside the SDK is too late: ESM links every static
import before any module evaluates.
