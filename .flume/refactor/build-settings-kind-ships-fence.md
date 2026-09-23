## Surface

`SETTINGS-KIND-SHIPS(builtin)` cannot reach green inside its fence: adding
`settings` to `builtin_kind::all_kinds()` breaks two test files the entry does not
list. Both fail on the entry's own stated consequence (auto-adoption governs every
harness's `.claude/settings.json`, so `coverage.unmodeled-surface` retires and
`locus.undeclared-member` fires), so neither is a defect in the change — each is a
case whose premise the entry retires, in a file the fence forbids.

- `tests/settings_kind.rs:282`
  (`the_same_bytes_under_the_pre_0050_posture_carry_no_rollup_row_and_keep_the_advisory`):
  the pre-0050 contrast case asserts `coverage.unmodeled-surface` still names
  `.claude/settings.json` when the program authors no container member. With the kind
  embedded, that harness draws `locus.undeclared-member` instead and the advisory count
  is 0. Exactly the shape of the `tests/hook_kind.rs:184` case the entry already lists —
  the rollup-row half still holds, the advisory half is the entry's to retire.
- `tests/install.rs:796` (`a_fresh_dry_run_scaffolds_and_writes_nothing`):
  the represent-path preview lifts 4, not 3 — the fixture's `.claude/settings.json` is
  now a governed locus `install --represent` scaffolds a member for.
- `tests/install.rs:1474`
  (`the_governed_locus_binding_leaves_the_neighbouring_guard_arms_alone`):
  its "a manifest locus stays the manifest arm's" leg writes `.claude/settings.json` and
  expects exit 0. `guarded_loci` (`src/main.rs:623`) excludes a kind carrying a
  `collection_address`; the `settings` container carries none, so its locus binds and the
  guard exits 2 with the undeclared-member finding. That agrees with `check` — the
  agreement that function's own doc says it exists to hold — so the re-baseline is the
  case's, not the code's: point the manifest-only leg at `.mcp.json` (a collection
  address with no container kind) and assert `.claude/settings.json` under the locus arm.

Everything the entry does list re-baselines as scoped — verified locally with the kind
added and `src/builtin_lock.toml` regenerated from the memberless SDK emit (one kind row,
three `settings.type.*` clause rows; `tests/builtin_lock_frozen.rs`'s byte-compare
holds). The remaining reds under `cargo test --no-fail-fast` were the fenced
`builtin_contract_matrix`, `coverage_note`, `gauntlet`, `hook_kind`,
`lock_declaration_rows` targets alone.

Unrelated, same file, noticed while moving the header's counts: `src/builtin_lock.toml`'s
provenance header ends mid-sentence at line 51 ("…a known"). `9d3aed09` truncated the
comment block; the sentence's tail and the two paragraphs after it went with it.

## Observed at

70ee04ea (HEAD when observed).

## Suggested consolidation

Re-scope the entry with `tests/settings_kind.rs` and `tests/install.rs` added to
`files.edit` — no mechanism changes, three cases re-baseline against whole-file
governance.
