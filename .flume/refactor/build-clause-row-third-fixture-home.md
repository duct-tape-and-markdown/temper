## Surface

A third exhaustive `ClauseRow` literal lives outside the two fixture bases
LOCK-ROW-FIXTURE-ONE-HOME established, so widening the row breaks it:

- `src/test_support.rs:141` `clause_row` — the src-side base.
- `tests/common/mod.rs:736` `clause` — the tests-side base.
- `tests/extent.rs:220` `row` — a private per-file helper spelling all
  twenty-three columns itself, differing from the base only in `label`,
  `kind`, `severity`, `bound`, and `unit`; every one of those five is
  reachable by struct-update over `common::clause`.

Blocks `DEGREE-FIELD-SET-FILTER`, which adds a `ClauseRow.fields` column:
`tests/extent.rs` is outside that entry's fence, and it is the **only**
out-of-fence file the change needs — verified by patching it locally and
running `cargo test --no-fail-fast` to completion (every other target
compiles, and the suite is green but for the expected
`seam_bindings_current` re-bless, which is in fence). Re-scoping the entry
with `tests/extent.rs` added, or landing the consolidation first, unblocks
it in one tick.

## Observed at

9cb98998 (HEAD when observed)

## Suggested consolidation

Fold `tests/extent.rs`'s `row` onto the tests-side base — `ClauseRow {
label: Some(...), kind: Some("skill".into()), bound: ..., unit: ...,
..common::clause(predicate, "advisory") }` — finishing
LOCK-ROW-FIXTURE-ONE-HOME on the tests side so the next column lands in two
places, not three.
