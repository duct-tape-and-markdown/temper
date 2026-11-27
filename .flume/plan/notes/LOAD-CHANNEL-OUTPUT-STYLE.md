# LOAD-CHANNEL-OUTPUT-STYLE: `selected(field)` channel shipped

Channel named `selected(outputStyle)` (SDK `{ via: "selected", field }`): the field
names the *selecting* edge's field, not one of the member's own. Every other
field-carrying channel names a field on the member itself, so this is the first
channel whose field is a cross-kind fact; `kind.rs` and `kind.ts` say so.

Selection is one derivation: `graph::selected_members` resolves exactly as
`resolved_edges` does, folded by `gate::selections`, read by both the `loads` guard
and `explain`. Unselected means no load moment (guard silent) and never dead (a user
setting or `/config` may select it), so reach is unchanged.

Structural move: `gate()` now reads every kind's units/features (built-in and
custom) before any contract is dispatched, since a style's moment depends on the
settings kind's members, which sort after it. Diagnostic order is unchanged;
`load_strand` took an `allow(too_many_arguments)` (precedent: `explain`).

Selection reads top-level members only (embedded members compose after dispatch).
No shipped edge selects from an embedded member today; a custom kind that did would
go unseen.

The engine stamp moved with `src/builtin_lock.toml`, so
`examples/base-harness/.temper/lock.toml` was re-emitted (stamp only).
`.temper/lock.toml` (outside the fence) carries no output-style row, so the
dogfood is unaffected.
