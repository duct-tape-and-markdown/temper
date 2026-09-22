## Surface

The `ClauseRow` fixture consolidation ran on the Rust side only
(LOCK-ROW-FIXTURE-ONE-HOME(src)/(extent)); the SDK side never had one, so a
new row column still breaks a hand-spelled literal there:

- `sdk/src/declarations.ts:105` `clauseRow` — the one writer, which spells
  every column unconditionally (`x: cond ? v : undefined`).
- `sdk/test/emit.test.ts:220` and `:243` — two inline `ClauseRow` object
  literals inside one `assert.deepEqual` over `declarations.clauses`, each
  naming all twenty-three columns including the `undefined` ones. The two
  rows differ only in `predicate`/`field`/`severity`/`bound`; every other
  column is `undefined` in both. `node:assert/strict` compares own key sets,
  so any new column makes both literals mismatch.

Blocks `DEGREE-FIELD-SET-FILTER`, which adds a `ClauseRow.fields` column:
`sdk/test/emit.test.ts` is outside that entry's fence, and it is the **only**
out-of-fence file the change needs. Verified by implementing the entry whole
and patching that file locally: `cargo fmt --all --check`, `cargo clippy
--all-targets -- -D warnings`, `cargo test --no-fail-fast`, `cargo doc
--no-deps --document-private-items` and `pnpm --dir sdk test` all green
(183/183 on the SDK side), with `src/builtin_lock.toml` byte-unchanged.

The one in-fence dodge — lowering `fields` by conditional spread so the key
never appears on a non-`degree` row — is rejected: it deviates from the
twenty-two sibling columns' idiom purely to slip a fixture, and leaves the
next column in the same trap. Re-scoping the entry with
`sdk/test/emit.test.ts` added, or landing the consolidation first, unblocks
it in one tick.

## Observed at

264009c5 (HEAD when observed)

## Suggested consolidation

Give `sdk/test/` the base the Rust side already has — a `clauseRow(predicate,
severity)` helper returning every column at its default, with call sites
spreading their four divergences over it (`{ ...clauseRow("extent",
"advisory"), bound: { max: 300 } }`). There is no `sdk/test/common.ts` yet;
this is its first tenant, and `sdk/test/contract.test.ts`'s own
`skillClauseRow` reads rows rather than spelling them, so the base has one
call site to serve today and the whole family tomorrow.
