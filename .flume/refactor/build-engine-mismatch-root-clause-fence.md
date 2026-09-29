## Surface

`ENGINE-MISMATCH-IS-A-ROOT-CLAUSE` widens the shipped root default contract
from three clauses to four. Two homes hold the *old* count and are outside the
entry's fence, so the entry cannot reach green as scoped. Both are the same
shape of debt: a census of the root default written down somewhere the entry's
`files[]` census (`rg Predicate::LocusDeclared`) does not reach, because
neither spells the variant — one spells the labels, the other is emitted data.

- `src/builtin_lock.rs:120` — the in-module unit test
  `the_embedded_lock_parses_into_kind_facts_and_floor_clauses_only` asserts
  `root_labels == vec!["root.reachable", "root.fresh", "root.locus-declared"]`,
  with the surrounding comment (`:93-94`) and the assertion message (`:121`)
  both naming "the trio". A fourth kind-less row fails it by construction —
  which is exactly the assertion's job, so the fix is to extend the list and
  the two prose spots, never to widen the match.
- `examples/base-harness/.temper/lock.toml:343` — the shipped example's
  committed lock carries the root default's clause rows, and
  `tests/it/emit.rs::emit_program_runs_the_shipped_example_harness`
  byte-compares the file against what the example's own program emits. The
  fourth row lands there too. Regeneration is the documented remedy and needs
  the SDK vendored into the example root first:
  `ln -s $PWD/sdk examples/base-harness/.temper/node_modules/@dtmd/temper`,
  then `cargo run -- emit --into examples/base-harness/.temper`.
  `examples/**` clears the outer ceiling but is not in `entry.files`.

Everything else in the entry reached green: `src/contract.rs`,
`src/engine.rs`, `src/gate.rs`, `src/schema.rs`, `sdk/src/contract.ts`,
`sdk/src/assembly.ts`, `sdk/src/index.ts`, `src/builtin_lock.toml`,
`tests/it/builtin_lock_frozen.rs`, `tests/it/root_contract.rs`,
`tests/it/manifest_schema_oracle.rs`,
`tests/it/snapshots/it__gauntlet__projection_tree.snap`,
`sdk/test/contract.test.ts`, `sdk/test/emit.test.ts`. `pnpm --dir sdk test`
was green (212 pass) and `cargo test --no-fail-fast` failed on exactly the two
targets above, nothing else.

Two details the re-scoped entry can take as decided rather than re-derive:

- The spelling: `Predicate::EngineMatches` / wire key `engine-matches` /
  `engineMatches()` / label `root.engine-matches`.
- The clause carries **guidance and no `cite`**, unlike what the entry's
  `sdk/src/assembly.ts` line asks for ("a `cite` like its three siblings").
  Two of those three siblings carry none, and the frozen lane's own assertion
  message says why: a cite is owed where a verdict rests on an *external*
  fact. `engine-matches` compares temper's own lock stamp against temper's own
  `VERSION` — nothing external — so a URL there would be invented, which the
  `collaboration` rule forbids outright. Both census tests were extended to
  pin that `reachable` is the only cited root clause.

`sdk/test/emit.test.ts` also holds **two** positional clause-row censuses
(`:259` and `:366`), not the one the entry's line reference names; both need
the fourth row.

## Observed at

d2e55aca (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation

Re-scope the entry with `src/builtin_lock.rs` and
`examples/base-harness/.temper/lock.toml` added to `files.edit`. Longer-term,
the root default's membership is currently transcribed in five places
(`src/builtin_lock.rs`, `tests/it/builtin_lock_frozen.rs`,
`tests/it/root_contract.rs`, `sdk/test/contract.test.ts`,
`sdk/test/emit.test.ts` ×2) — every one of them deliberately, since each pins
a different derivation against a different source. That is not duplication to
collapse; it is a census whose *scoping* query needs to be
`rg 'root\.locus-declared'` alongside `rg Predicate::LocusDeclared`, since a
widened root default moves label strings and emitted data, not just variants.
