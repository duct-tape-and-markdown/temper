# Built-in loci shipped; dogfood lock re-emit is the human half

The rule locus is now `.claude/rules/**/*.md`. Memory excludes `.claude/{agents,commands,rules,skills}`. Both changes are in all four homes: SDK, Rust population, embedded lock and the base-harness example lock. `overlay_builtin_kind` now takes `exclude` from any row that carries governs, and the lift test compares `excluded`.

- Human `chore(harness)` half: re-emit the dogfood `.temper/lock.toml`. Its rule and memory rows still hold the old locus. This tick did not touch it.
- The fact is copied by hand into four homes plus three test assertions. That is the debt BUILTIN-KIND-FACTS-DERIVE-FROM-THE-EMBEDDED-LOCK already names. Fixing it would also remove the unit test `rule_definition_matches_the_hand_authored_kind` in `src/builtin_kind.rs`, which spells the glob a fifth time.
- JSDoc trap: writing `**/` inside a `/** */` comment closes the comment. The SDK prose says "at any depth" instead.
