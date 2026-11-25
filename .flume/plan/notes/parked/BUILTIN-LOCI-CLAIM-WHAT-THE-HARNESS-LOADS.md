# Built-in loci fix needs src/builtin_kind.rs, outside the fence

Parked: the entry cannot ship inside its fence. Discovery, check, emit and install do not read the built-in kinds' loci from the embedded lock. They read them from the hand-written Rust population in `src/builtin_kind.rs` (`claude_code_rule` and `claude_code_memory` via `definitions()`, used in `gate.rs:276`, `main.rs`, `install.rs`, `compose.rs`, `read.rs`). That file is not in the fence; only `src/builtin_lock.toml` is.

Editing only the SDK and the lock breaks things:
- `builtin_lock::tests::every_lock_kind_row_lifts_to_the_facts_the_rust_population_spells` asserts that each lock row's `governs` equals the Rust kind's. Changing the rule glob to `**/*.md` in the lock alone turns it red (afterMerge `cargo test`).
- None of the three `tests[]` lines can go green. Discovery walks `builtin_kind`'s `Governs { ".claude/rules", "*.md" }`, and the memory kind has an empty `excluded`.

The re-plan should add `src/builtin_kind.rs` to the fence. In that file, the rule glob becomes `**/*.md`, and memory gets `excluded: [".claude/agents", ".claude/commands", ".claude/rules", ".claude/skills"]` with the memory-docs cite (retrieved 2026-11-24). That makes the lift test compare `excluded` too, or add a column assertion for it, since it is a fact the row now carries.

A second gap to weigh in the same tick: `compose::overlay_builtin_kind` overlays `governs`, `templates` and `content` from a committed lock row, but not `exclude`. A harness whose committed lock carries the memory row (the dogfood's `.temper/lock.toml` after its re-emit) would keep the Rust kind's `excluded`. That is fine once the Rust kind carries the list, but a relocated memory locus declaring its own exclude would silently drop it. `src/compose.rs` would be needed for that.

Nothing else was touched.
