## Surface
`dial` is the one kind a run resolves off disk **three** times, where the cost
doctrine's own count-pin claims once per kind:

- `src/compose.rs:1121` — `assemble_lock_family`'s local-locus pre-pass, which
  ranges over every `Commitment::Local` kind (`dial`, `settings-local`).
- `src/compose.rs:1072` — `read_dial`, called from the same
  `assemble_lock_family` a few lines later (`src/compose.rs:1137`), resolves
  `dial` again to build the `Dial` from its features.
- `src/gate.rs:255` — the judging pass over `overlaid_builtin_kinds` resolves
  it a third time.

Measured this tick: a two-member harness plus one locked custom kind produces
19 `resolve_kind_units` calls for 16 consulted kinds — 15 built-ins + 1 custom
+ 2 local pre-pass + 1 `read_dial`. `tests/it/check_cost.rs`'s
`resolve_kind_units_runs_once_per_kind_not_twice` now pins that 19 exactly
(it previously hid the overshoot under a `resolves < 2 * 15` ceiling), so the
count moves the moment a consolidation lands.

The local pre-pass and `read_dial` both want the same `dial` units the judging
pass will resolve anyway; nothing between them invalidates the read.

## Observed at
f05acdb7 (HEAD when observed)

## Suggested consolidation
`assemble_lock_family` already resolves every local-locus kind once — keep
`LockFamily`'s per-kind `KindUnitsAndFeatures` from that pass and have both
`read_dial` and `gate`'s judging loop read it instead of re-resolving, so the
one home for "this kind's units" is the family, per
`specs/process/engineering.md`, "Derived state is computed, never stored
beside its source" (a cache is one home with one invalidation).
