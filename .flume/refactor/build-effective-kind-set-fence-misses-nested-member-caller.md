## Surface

`EFFECTIVE-KIND-SET-IS-DERIVED-ONCE-PER-RUN`'s `files[]` carries four paths
(`src/compose.rs`, `src/gate.rs`, `src/read.rs`, `tests/it/check_cost.rs`) and
the entry directs a signature change on
`compose::builtin_units_and_features_by_kind` — "Both take the hoisted map
instead, threaded through `kind_units_and_features` (1043) and
`builtin_units_and_features_by_kind` (1078)". That function has a **second
caller the entry did not name**:

- `tests/it/nested_member.rs:1443` — the `compose_corpus` helper
  (`nested_member.rs:1437`) calls
  `compose::builtin_units_and_features_by_kind(&family.overlaid_builtin_kinds,
  &disc, &family.declarations, &cache)`.

The path is outside the fence, so the directed change is unbuildable as scoped:
with the function's arity changed and that call site untouched, the `it` target
does not compile and the `cargo test` gate reverts the whole commit.

No way around it from inside the fence. The function needs **both** maps — the
overlaid one is the built-in/custom split its own loop selects on, the declared
one is what each inner `kind_units_and_features` must receive — so every honest
shape (extra param, or `&LockFamily` in place of the two) changes the arity, and
every shape that preserves arity does so by having the call site pass the
overlaid map where the declared map is expected: a silent subset that happens to
be correct only for a lock declaring no custom kinds. `rg` confirms the two
callers are the only ones:
`tests/it/nested_member.rs:1443` and `src/read.rs:2347`.

## Observed at

e4185300 (HEAD when observed).

## Suggested consolidation

Re-scope the entry with `tests/it/nested_member.rs` added to `files.edit`; the
edit there is one line — thread `&family.declared_kinds` in as the second
argument. The rest of the entry executes exactly as written and was verified
green this tick under the probe patch:

- `cargo test --no-fail-fast`: 868 passed, 0 failed — `tests/it/nested_member.rs`
  is the *complete* out-of-fence list, nothing else breaks.
- The new count pin (`check_cost::declared_kind_set_is_derived_once_per_check_run`)
  passes post-fix and fails pre-fix at **4** derivations, not the entry's
  predicted 3: the embedded kind set carries **two** locus-less kinds, not one,
  so the pre-fix count is 1 (assembly) + 1 (manifest cache) + 2 (each locus-less
  kind's own unit resolution). Worth correcting in the re-scoped entry's text.
- `tests/it/check_cost.rs`'s existing
  `gate_manifest_cache_read_is_hoisted_across_governing_kinds` loses its
  `declarations` fixture entirely once `build_manifest_cache` stops deriving:
  the two `KindFactRow`s and the `Declarations` built from them become dead, as
  do the `CollectionAddressRow`/`Declarations`/`KindFactRow` imports. Expected,
  not a loss of coverage — the pin is about manifest reads per governing kind,
  and the kinds reach it through the map directly now.

One unrelated operational note for whoever runs this next, already captured at
`.flume/friction/plan-worktree-cargo-test-needs-sdk-node-modules.md`: a fresh
worktree needs `pnpm -C sdk install && pnpm -C sdk build` before `cargo test`,
or 64 SDK-dependent tests fail at `tests/it/common/mod.rs:111`.
