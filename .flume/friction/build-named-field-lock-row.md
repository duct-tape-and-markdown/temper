## Symptom
Adding `UnitShape`'s third mode (`named-field`) required a lock-row wire label
that embeds the identity field name (`named-field(name)`, the same
`<name>(<field>)` call syntax `registration` labels already use). But
`sdk/src/declarations.ts`'s `kindFactRow` only ever wrote `facts.unitShape`
verbatim — `identityField` was dropped from the kind-fact row entirely, for
every kind, always (silently harmless for `file`/`directory` since neither
needs it to reload an id, but load-bearing and lossy for `named-field`).
This file wasn't in the entry's `files` list, so it wasn't obvious until the
`--frozen` byte-compare test (`tests/builtin_lock_frozen.rs`, which spawns a
real `node`/`npm run build` against the actual SDK) failed against a
hand-authored guess.

## Cost this tick
~20 minutes: one aborted hand-authored TOML guess (`named-field(name)`, which
turned out right) then a full round of building the SDK, wiring a scratch
`.temper/harness.ts`, running `temper emit` against it, and diffing byte-for-
byte to discover the *actual* problem was upstream in `declarations.ts`, not
the Rust-side label parser.

## Suggested fix
A rule/CLAUDE.md line: "a field-carrying kind/registration fact belongs in
its *label* (`<name>(<field>)`), never a sibling column — check
`sdk/src/declarations.ts`'s row-builders whenever a `KindFacts`/`Registration`
fact gains a field-shaped payload." Would have pointed straight at
`kindFactRow`/`unitShapeLabel` instead of a build-and-diff round trip.
