# HAND-SPELLED-FACT-FOLDS: matcher pair already held by a real-writer pin

tap.rs log_path now joins crate::WORKSPACE_DIR, not a ".temper" literal.

You can't fold the guard matchers into one cross-language home. Each fact
already has one home per language: Rust's GUARD_MATCHER/BASH_MATCHER in
src/install.rs (GATE_HOOKS), and the SDK's GUARD_MATCHER (builtins.ts) and
SHELL_MATCHER (patterns.ts). The pair is already held by real writers.
`install --represent` scaffolds harness.ts over defaultPatterns(), whose
gateHooks() emits the settings rows. gate_installed then judges those rows
against GATE_HOOKS. tests/it/install.rs
gate_installed_never_scaffolds_and_reflects_represented_vs_not asserts that
reads green. Verified this tick: changing the SDK's GUARD_MATCHER to
"Write|Edit" makes that test fail with "guard hook missing". So the
comment pointers ("src/install.rs's GUARD_MATCHER/BASH_MATCHER") were the
only thing joining the two copies, and they are removed.

Not done: d2f1d4f9's install.rs/drift.rs sites, per the entry.
