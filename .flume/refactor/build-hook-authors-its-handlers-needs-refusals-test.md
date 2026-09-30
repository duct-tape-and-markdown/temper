## Surface

`HOOK-AUTHORS-ITS-HANDLERS` cannot reach green inside its fence: one file it does
not list authors a `hook()` on the retired flat spelling, and `sdk/tsconfig.json`
compiles `test/**/*.ts` under `strict`, so the stale line fails `tsc` — which is
both the `sdk test` gate and `common::ensure_sdk_built`, so every SDK-driven Rust
test falls with it.

- **`sdk/test/refusals.test.ts:380`** — `hook({ name: "PreToolUse", type: "command",
  command: "temper guard" })`, inside *"emit refuses an edge field naming a target
  that owns no projection"*. The hook is incidental scaffolding there (a
  registration member owns no projection to point an edge at); the fix is the one
  line the entry's other two SDK test files already take:
  `hook({ name: "PreToolUse", hooks: [{ type: "command", command: "temper guard" }] })`.

That is the whole out-of-fence list, verified: with the entry's own work applied
plus that one line, `node_modules/.bin/tsc -p sdk/tsconfig.json` is clean,
`pnpm --dir sdk test` leaves one failure (the tap-row re-baseline in the
in-fence `sdk/test/emit.test.ts`), and `cargo test --no-fail-fast` leaves exactly
two, both in the in-fence `tests/it/install.rs` (the gate-hook module shape at
:1277 and the lifted-registration module shape at :1409). Nothing else moved —
`it__gauntlet__projection_tree.snap` did not shift a byte.

Three further files spell the retired shape and stay **green** (their programs are
Rust string fixtures `node` type-strips, and the write face still nests flat
residue into one handler), so they are staleness rather than breakage — worth
riding the re-scope so the corpus stops carrying a shape the surface refuses:
`tests/it/gauntlet.rs:132,141`, `tests/it/settings_kind.rs:39,61`, and the
dogfood's own `.temper/hooks.ts:18,25,37,45` (a `chore(harness):` commit, outside
any build fence).

## Observed at

3ddb7fc5 (HEAD when observed).

## Suggested consolidation

Re-scope the entry with `sdk/test/refusals.test.ts` on the fence. No design
question is open — the entry's `sdk/src/builtins.ts` /
`sdk/src/declarations.ts` / `src/install.rs` work compiles and the breakage list
above is complete.
