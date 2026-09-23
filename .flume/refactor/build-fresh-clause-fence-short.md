## Surface

`FRESH-CLAUSE-JUDGES-STALENESS` is unshippable under its fence: six paths the
entry never listed must change for `cargo test --no-fail-fast` and
`pnpm --dir sdk test` to go green. Implemented the whole entry first (new
`Predicate::Fresh`, the three drift judges threaded off the root clause, the
SDK `fresh()` + `rootDefaultContract` row, the regenerated embedded lock) and
swept; every in-fence target passed or was fixable. These did not:

1. `src/builtin_lock.rs:117` —
   `the_embedded_lock_parses_into_kind_facts_and_floor_clauses_only` pins the
   shipped root default's row inventory at exactly `["root.reachable"]`, with
   the message "a second kind-less row is a row nobody declared". The entry
   adds a second kind-less row by design, so the pin must be restated to the
   two-row inventory. Same claim `tests/builtin_lock_frozen.rs` carries (which
   *is* fenced) — the lib-side twin was missed.
2. `tests/root_contract.rs:335` —
   `the_shipped_root_default_binds_reachable_and_rides_a_harness_that_declares_none`
   asserts the emitted default's kind-less rows lift back as exactly
   `[("root.reachable", Reachable, Advisory)]`. Second-row break, same cause.
   The test's *name* also becomes false once the default binds two predicates.
3. `tests/prose_include.rs:77,79,112,130` — four `drift::include_stale(&into)`
   calls. `include_stale` is one of the three judges the entry re-signatures to
   take the root `fresh` clause, so this file cannot compile. Its sibling
   `tests/layout_prose_import.rs` (the `layout_import_stale` twin) *is* fenced;
   the `include_stale` twin was missed.
4. `tests/manifest_schema_oracle.rs:143` — a non-exhaustive `match` over
   `Predicate`. Every closed-vocabulary addition breaks it (`E0004`); it needs
   `Predicate::Fresh` in its `return None` arm, one line.
5. `tests/settings_kind.rs:14,249,325` — asserts `check` findings under the
   literal rule id `config.stale`. The entry's point is that those findings now
   report under the `fresh` clause's label, so these move to `root.fresh`.
6. `examples/base-harness/.temper/lock.toml` — the example's program declares
   no `contract`, so `harness()` composes `rootDefaultContract` and the
   example's committed lock gains the `root.fresh` row.
   `tests/emit.rs::emit_program_runs_the_shipped_example_harness` byte-compares
   it (`cargo run -- emit --into examples/base-harness/.temper`). Regeneration
   only — no hand edit.

Two further non-blocking references to the retired rule id sit outside the
fence and would ship stale: `src/install.rs:1334` and `sdk/src/builtins.ts:1360`
both name `config.stale` in prose. Comments only; no gate reads them.

Two scope notes for the re-cut, from having built it:

- The entry names severity and label as what the clause supplies. Guidance is
  the third channel on the same clause and `check::Diagnostic::with_guidance`
  already exists — threading two of three repeats exactly the gap
  `ROOT-REACHABLE-GUIDANCE-REACHES-ITS-FINDING` just closed, so the judges
  should read all three.
- The entry asks `tests/check_cost.rs` to pin "the no-clause case doing no walk
  at all". That skip is `gate.rs`'s `if let Some(clause)`, and the lock document
  is already parsed by then for other reasons, so the read/parse counters cannot
  observe it. The observable claim is behavioural — a root contract binding no
  `fresh` clause reports no staleness finding — and belongs in
  `tests/acceptance.rs` beside the dialed case, not in the cost suite.

## Observed at

4db8fdb9 (HEAD when observed).

## Suggested consolidation

Re-scope the entry with those six paths added to `files.edit`. Three are
one-line or mechanical (2, 4, 6); two are the fenced pins' unfenced twins
(1 and 3 — `tests/builtin_lock_frozen.rs` and `tests/layout_prose_import.rs`
were listed, their siblings were not), which suggests the scoping pass keyed on
one home per claim where the codebase keeps two.
