# Built-in loci: three SDK tests pin the old rule glob

Parked a second time on the fence. The whole entry was built and verified green in this tick, then reverted. `cargo test` (all 938), clippy, fmt and doc all passed. The one failing gate was afterMerge `pnpm --dir sdk test`, which had 3 failures. Each one asserts the old `.claude/rules` + `*.md` locus, and all of them live in `sdk/test/`, which the fence does not admit:

- `sdk/test/builtins.test.ts:1443` and `:1458`: `assert.match(error.message, /\.claude\/rules\/\*\.md/)` becomes `/\.claude\/rules\/\*\*\/\*\.md/`.
- `sdk/test/emit.test.ts:251`: the rule row's `governs_glob: "*.md"` becomes `"**/*.md"`. Memory is absent from that fixture.

The re-plan should add `sdk/test/builtins.test.ts` and `sdk/test/emit.test.ts` to the fence. The SDK edit cannot be dropped: builtin_lock_frozen ties the SDK to the embedded lock, and the lift test ties that lock to the Rust population.

What this tick built, unchanged from the previous park note's shape:
- SDK, Rust population, `src/builtin_lock.toml` and `examples/base-harness/.temper/lock.toml` all moved. The rule glob is `**/*.md`. The memory row gains `exclude = [".claude/agents", ".claude/commands", ".claude/rules", ".claude/skills"]`. Every row sits right after `governs_glob`, which is the emit's own column order.
- The lift test compares `excluded`. `overlay_builtin_kind` sets `excluded` from the row whenever the row carries governs.
- `lock_declaration_rows.rs:2620` changes to `**/*.md`, and it also asserts `memory.exclude`.
- Re-bless the gauntlet `projection_tree` snapshot: one line changes, the rule glob.
- `tests/it/builtin_loci.rs` holds 5 tests covering the four tests[] lines plus the `.claude` pin. The pin `every_built_in_kind_glob_is_a_placement_template` is a unit test in `builtin_kind.rs`.
