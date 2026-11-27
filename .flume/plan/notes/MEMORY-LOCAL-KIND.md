# MEMORY-LOCAL-KIND shipped: CLAUDE.local.md is its own local-class kind

- `memory-local` claims `**/CLAUDE.local.md` at `.`, commitment `local`, memory's four
  loader-folder excludes, `on-read-beneath`. Ships `memoryLocalDefaultContract`: one
  advisory `extent` 200 lines, the same size target memory carries.
- The engine reads `@path` imports for `memory-local` as it does for `memory`
  (`declared_directives`, src/builtin_kind.rs). I took that from the spec's "treated the
  same way". Nothing in the corpus names directives for the local file explicitly, so a
  plan tick may want the spec to say so.
- The dogfood's own `.temper/lock.toml` needs a `chore(harness)` re-emit: the
  built-in digest stamp moved. That path is outside build's writable set. The example
  lock was re-emitted here; only its stamp moved.
- Debt: the local-flavor discovery walk waives ignore rules over the whole tree, so an
  any-depth local glob also reads a `CLAUDE.local.md` under `node_modules/` or a vendored
  tree. `settings-local` and `dial` never noticed because their loci are fixed paths.
- The cite date `retrieved 2026-11-27` is copied from specs/builtins.md. It is one day
  after this tick's clock, so the spec's date may be a typo.
- The gauntlet coverage snapshot moved: one more kind, `memory-local (0)`.
