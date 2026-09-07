## Surface

`CHECK-UNDECLARED-MEMBER-AT-A-GOVERNED-LOCUS` is implementable exactly as scoped,
but cannot reach green inside its fence. Two arms of **`tests/acceptance.rs`** —
a file `entry.files` does not list and the outer ceiling does allow — break on the
entry's intended behaviour:

- `tests/acceptance.rs:357` — `check_dispatches_the_spec_custom_kind_through_its_extractor_and_contract`,
  assertion `!output.contains("00-intent")`.
- `tests/acceptance.rs:394` — `check_reads_a_custom_kind_rooted_outside_specs`,
  assertion `!output.contains("0001-short")`.

Both are **whole-stdout substring** assertions standing in for "the clean member
trips no `extent` finding". Their shared fixture `author_custom_kind_lock`
(`tests/acceptance.rs:286`) writes a lock carrying a `[[declaration.kind]]` row and
one `[[declaration.clause]]` row and **no member rows at all**, then drops two
documents at the declared locus. That is a represented harness whose every
discovered member is undeclared — precisely the population this entry names — so
`locus.undeclared-member` fires over both documents, its artifact path is
`adr/0001-short.md` / `specs/00-intent.md`, and the coarse substring assertion
trips on the *new* finding rather than on an `extent` one.

The collision is in the assertion's grain, not in the behaviour: narrowing each to
the `extent` rule (or declaring the fixture's members) is a two-line edit. Nothing
about the entry's design needs to move — silencing this case would gut the check
for the commonest adoption shape (kind declared, members not), and the entry's own
`acceptance` states the teeth plainly: "from this entry on CI fails for any
represented harness carrying an undeclared document at a governed locus."

`cargo test --no-fail-fast` over the full implementation is green everywhere else,
including `tests/layout_kind.rs` untouched and
`tests/snapshots/gauntlet__check_diagnostics.snap` unmoved. `tests/acceptance.rs`
is the whole remainder.

A second, smaller fence gap sits on the entry's RIDER (the `ClauseRow::field` doc
comment at `src/drift.rs:3562`): `ts-rs` exports that doc comment, so rewording it
makes **`sdk/src/generated/ClauseRow.ts`** stale and fails
`tests/seam_bindings_current.rs::committed_bindings_byte_match_a_fresh_export`
(regenerate with `BLESS_SEAM_BINDINGS=1`). The fence carries `sdk/src/index.ts`
but not `sdk/src/generated/**`, so the rider is unshippable here too and was
reverted.

## Observed at

17137adc (HEAD when observed)

## Suggested consolidation

Re-scope the entry with `tests/acceptance.rs` added to `files.edit` — description:
scope both clean-member assertions to the `extent` rule instead of whole stdout,
so a second rule naming the same artifact cannot forge a failure. Add
`sdk/src/generated/ClauseRow.ts` too if the RIDER is to ride along, noting the
`BLESS_SEAM_BINDINGS=1` regeneration step.
