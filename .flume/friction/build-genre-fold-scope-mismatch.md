## Symptom

GENRE-FOLD's `entry.files` cited three loci that don't back what the entry's
prose claims once read against the actual code:

- `src/kind.rs`'s `from_kind_fact_row` note says to "restore the nested-member
  templates from the lock" — but `KindFactRow` (`src/drift.rs`) carries no
  column for a host kind's declared templates, and the SDK's `kindFactRow()`
  (`sdk/src/declarations.ts:182`) explicitly filters `genre`-locus kinds out
  of the composed kind facts entirely (`kindsInPlay` keeps only
  `locus.kind === "at"`). There is no fact to restore without a schema
  change, and `entry.schemaDelta` is `"none"`.
- `src/schema.rs` was cited as needing to "surface the nested-member shape,"
  but the file contains zero references to genres/nested members today —
  `emit()` only projects `contract::Predicate` clauses, nothing kind-shaped.
- The SDK-bindings ripple ("the ts_rs/JsonSchema derive at the retired type
  ripples to the SDK bindings") doesn't apply: no build step calls
  `.export()` anywhere in the repo, so there is no generated bindings file to
  ripple into, and `sdk/src/genres.ts`'s `GenreValue` is a hand-authored,
  independent TS interface for a different (standalone genre-locus kind)
  feature that isn't wired to `CustomKind`'s embedded templates at all.

## Cost this tick

~40 minutes tracing `KindFactRow`/`sdk/src/declarations.ts`/`sdk/src/kind.ts`
to confirm no lock fact or generated-bindings path exists before deciding
these three sub-clauses were partly stale/aspirational rather than something
to build. Landed a narrower fix instead: added `Primitive::Fenced` to
`from_kind_fact_row`'s reconstructed extraction (closing the raw-substrate
half of the gap, no schema change) and left `templates` empty there with a
comment naming the residual gap; left `schema.rs` and `sdk/` untouched.

## Suggested fix

When an entry cites a lock column, generated-bindings path, or file-level
surface that doesn't exist on disk, plan should verify the cite resolves (a
quick `rg` against the named struct/file) before shipping the entry, or mark
that sub-clause as an open question rather than folding it into an ATOMIC
entry's acceptance bar.
