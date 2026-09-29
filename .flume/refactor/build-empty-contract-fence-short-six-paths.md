## Surface

`EMPTY-CONTRACT-IS-A-DECLARED-CONTRACT` is fence-short by six paths. The entry's
design is right and reaches green — implemented whole this tick, all five gates
verified (`cargo fmt`, `cargo clippy -D warnings`, `cargo test` 813+358 pass,
`pnpm --dir sdk test` 215 pass, `cargo doc`) — but six touched paths sit outside
`entry.files`, so the commit would revert whole. Nothing was committed.

The marker rides `AssemblyFactRow` as the entry specifies, and `compose.rs` reads
it as decision 0072's consequence states. Reading it means the two rows-or-default
functions need the assembly slice their callers never passed:

- `compose::builtin_contract(clauses, kinds, kind)` → `(clauses, kinds, assembly, kind)`
- `compose::root_contract(clauses)` → `(clauses, assembly)`

**Callers — seven sites, four files, one added `&declarations.assembly` argument
each** (`declarations` is already in scope at every one):

- `src/read.rs:2410` — `builtin_contract`
- `src/gate.rs:246` — `builtin_contract`; `src/gate.rs:519` — `root_contract`
- `src/main.rs:619`, `src/main.rs:668` — `builtin_contract`; `src/main.rs:775` — `root_contract`
- `src/install.rs:1149` — `root_contract`

**Derived artifacts the regeneration moves** (no hand edit, both re-derived by
the commands below):

- `tests/it/snapshots/it__gauntlet__projection_tree.snap` — the gauntlet
  fixture's emitted lock gains one `contract` marker row (`from = "machine"`,
  its one `expect` binding). `cargo insta accept`. **Also a fence bug — see
  `.flume/friction/build-snapshot-fence-glob-misses-tests-it-snapshots.md`**:
  `.flume/chain.ts`'s `BUILD_SURFACE_PATHS` already means to allow every
  build tick a snapshot re-bless, but its glob is `tests/snapshots/**` and this
  repo's snapshots live at `tests/it/snapshots/`. Fixing that glob drops this
  path from the re-scope entirely.
- `examples/base-harness/.temper/lock.toml` — gains eight `contract` markers,
  one per `expect`-bound kind. Regenerate:
  `ln -sfn $PWD/sdk examples/base-harness/.temper/node_modules/@dtmd/temper`
  (gitignored) then `cargo run -- emit --into examples/base-harness/.temper`.

Everything else the entry named was sufficient and is green: `sdk/src/assembly.ts`
(`contractDeclared` on `Harness`), `sdk/src/declarations.ts` (`assemblyFactRows`
writes the markers, kind-sorted after `mode`), `src/drift.rs` (the `fact`
discriminator widens; `sdk/src/generated/AssemblyFactRow.ts` re-blesses with
`BLESS_SEAM_BINDINGS=1`, which the fence already allows), `src/builtin_lock.toml`
(sixteen markers, no root marker — the memberless program declares no `contract`),
`sdk/test/emit.test.ts`, `tests/it/lock_declaration_rows.rs`,
`tests/it/root_contract.rs`. `tests/it/builtin_lock_frozen.rs` needed no edit, as
the entry predicted.

## Observed at

cc46bbb1 (HEAD when observed) — the premise delta since the entry was scoped at
7beb01ba changed none of this.

## Suggested consolidation

Re-scope the entry with `src/read.rs`, `src/gate.rs`, `src/main.rs`,
`src/install.rs` and `examples/base-harness/.temper/lock.toml` added to
`entry.files.edit`, described as the signature ripple and the derived example
lock. Add `tests/it/snapshots/it__gauntlet__projection_tree.snap` too unless the
`chain.ts` glob lands first.

The signature ripple is the honest shape of the change, not avoidable debt: the
alternative that keeps the old arity is a second pair of functions the real gate
path never calls, which would leave `check` applying the default over a contract
the author emptied — the exact collision 0072 rules against.
