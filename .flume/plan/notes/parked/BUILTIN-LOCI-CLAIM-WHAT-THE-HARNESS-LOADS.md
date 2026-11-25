# Built-in loci: one out-of-fence assertion pins the old rule glob

Parked. The work is done and green except for one test the fence does not admit. `tests/it/lock_declaration_rows.rs:2620` (`the_embedded_lock_kind_facts_match_todays_hand_written_kinds`) asserts `rule.governs_glob == Some("*.md")` against the embedded lock. Re-deriving the lock (required by builtin_lock_frozen) makes it `**/*.md`, so the test turns red in afterMerge `cargo test`. Editing it reverts the commit, since the fence has `tests/it/builtin_loci.rs`, `tests/it/main.rs` and snapshots but not this file.

The re-plan should add `tests/it/lock_declaration_rows.rs` to the fence. The edit is one line: `"*.md"` becomes `"**/*.md"` at :2620. It could also assert the memory row's new `exclude` beside :2631.

Verified in this tick, then reverted:
- `sdk/src/builtins.ts`: rule glob is now `**/*.md`. Memory's locus gains `exclude: [".claude/agents", ".claude/commands", ".claude/rules", ".claude/skills"]`. Both are cited to code.claude.com/docs/en/memory, retrieved 2026-11-24.
- `src/builtin_kind.rs` has the same facts. The memory list is a cited `MEMORY_EXCLUDED_LOADER_FOLDERS` const. Unit pin: `every_built_in_kind_glob_is_a_placement_template`.
- `src/compose.rs` `overlay_builtin_kind`: when a row carries `governs`, its lifted `excluded` replaces the kind's. Exclusions belong to the locus, so a relocation never inherits exclusions spelled against another root.
- `src/builtin_lock.rs`: the lift test also compares `excluded`.
- `src/builtin_lock.toml` and `examples/base-harness/.temper/lock.toml`: rule glob and memory exclude rows re-derived. builtin_lock_frozen is green, and so is the base-harness emit test.
- The gauntlet `projection_tree` snapshot moves by one line, the rule glob (re-bless it).
- `tests/it/builtin_loci.rs`: the four tests[] lines and the `.claude` pin, all green. The overlay test is `a_committed_lock_rows_exclude_overlays_the_built_in_kinds`.

Clippy and fmt were clean; full `cargo test` failed only on the assertion above.
