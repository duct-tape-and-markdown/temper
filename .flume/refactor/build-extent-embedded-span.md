## Surface

`extent` (this tick, EXTENT-PREDICATE) is fenced as **bodyless** for embedded
members — `src/engine.rs` `bodyless` lists `Predicate::Extent`, so a contract
binding `extent` to an embedded kind fails admissibility, exactly as the retired
`max_lines` did. But 0035's stated load-bearing motivation is the *embedded*
case: centercode had to withdraw advisory budgets on its embedded posture kinds
(orientation ~12 lines, directive ~4, step ~3) because no predicate ranged over
a value's size. This tick ships the vocabulary shift + the file-member half, but
does **not** deliver the embedded budget 0035 exists for — the entry's engine
description ("an embedded value's extent is its rendered span in the host
artifact, the load-bearing case") names a measurement with no data path.

The gap is concrete: an embedded member's `Features` are built at
`src/main.rs:2052` (`features_from_nested_member_row`) with `rendered_lines: 0`
/ `rendered_chars: 0`, because `drift::NestedMemberRow` (`src/drift.rs:3013`)
carries the member's `leaves`/`collections` but **no captured rendered span**.
The engine never re-renders (the `placed_edges` precedent: a render fact reaches
the engine as a declaration row or not at all), so the span must be captured by
`emit` — the SDK renders each embedded member to its `member.<kind> <key>` TOML
fence and knows its byte/line extent at that point.

## Observed at

fe8790e (HEAD when observed)

## Suggested consolidation

Add a captured rendered-extent to `NestedMemberRow` (e.g. `rendered_lines` /
`rendered_chars`, the same optional-column shape `placed_edges` takes),
populated by the SDK's embedded-member render in `sdk/src/` and lowered through
`sdk/src/declarations.ts`; lift it into the embedded member's `Features` at
`src/main.rs:2052` and drop `Predicate::Extent` from the `bodyless` fence in
`src/engine.rs`. Then centercode's embedded posture budgets resolve — the actual
0035 acceptance. (Two-sided seam change, `sdk.md` "The engine seam".)
