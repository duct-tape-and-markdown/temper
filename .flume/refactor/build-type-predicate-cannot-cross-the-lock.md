# `type` is a predicate the engine decides but no lock can carry

## Surface

The `type` predicate is wired at four of five homes and missing the middle one,
so it round-trips nowhere — an author who reaches for it gets a hard load error,
never the check they asked for:

- `sdk/src/contract.ts:68` — `type = (field: string)` is exported, and
  `sdk/test/root_exports.test.ts` ("the root carries the whole closed predicate
  vocabulary") asserts it is part of the vocabulary. It takes **no declared
  kind**, so it cannot express the thing the predicate exists for
  (`keywords` must be a `list`) even before it is emitted.
- `sdk/src/generated/ClauseRow.ts` — the clause row carries a column per
  predicate argument (`bound`, `charset`, `keys`, `values`, `count`, `target`,
  `degree`, `range`, `section`) and **none for a `ValueType`**.
- `src/contract.rs:323-410` — `predicate_from_row` has an arm for every
  predicate in the closed vocabulary **except `type`**, so a `type` row lifts to
  `None`.
- `src/compose.rs:181` — that `None` is `ClauseRowError::Predicate`, rejected
  loud. Correct behavior, and it means the failure is a refusal rather than a
  silent drop — but the refusal fires on a predicate the SDK told the author to
  use.
- `src/contract.rs:97` / `src/engine.rs:626` / `src/schema.rs:64,284,460` — the
  `Predicate::Type { field, kind }` variant is fully typed, decided by the
  engine, and projected to JSON Schema's `type` keyword. All of it is reachable
  only from Rust-side construction: `rg 'predicate = "type"'` over `src/` and
  `.temper/` returns nothing, and no shipped default contract carries one.

Cost this tick: `PLUGIN-MANIFEST-KIND` wanted the documented, decidable
"a wrong-typed component path field is a load error everywhere" clause
(`keywords` a string instead of an array —
code.claude.com/docs/en/plugins-reference, retrieved 2026-07-16). It is not
expressible, so `pluginManifestDefaultContract` names the hold in its header
instead, the way `skillDefaultContract` already names its own two.

## Observed at

024ba9b (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation

Finish the one seam that is missing: give `ClauseRow` a value-type column, add
the `predicate_from_row` arm that reads it, and give the SDK's `type()` its
second argument (`type("keywords", "list")`) over the closed
`string`/`integer`/`number`/`boolean`/`list`/`map`/`null` lattice
`ValueType` already spells. Every other home already exists; nothing here is a
new predicate, so the closed algebra does not grow.

The sibling gap this one does *not* cover, and which is a genuine model
question rather than a wiring one: `--strict`'s unrecognized-top-level-field
bar needs an **allow-list** over a closed key set. `forbidden_keys` is a
deny-list and cannot express the complement, and `Predicate::Optional`
(`src/contract.rs:88`) records a key as "part of the declared schema" but is
`Outcome::Holds` unconditionally (`src/engine.rs:621`) — nothing consumes the
record. Whether a closed surface is opt-in per contract, derived from the
`optional` rows, or a new predicate is a `specs/model/contract.md` decision, not
a build tick's to invent.
