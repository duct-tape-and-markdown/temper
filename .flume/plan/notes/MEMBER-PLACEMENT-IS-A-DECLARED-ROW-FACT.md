# Member placement is a declared row fact

Shipped. Observations for plan:

- Debt `interim-template-placement` (.flume/plan/debt.md) is retired by this commit;
  the debt file is outside build's writable paths, so the drain owes its removal.
- "Places whole" is defined as: a `:` in a path-derived identity (every shape but
  `named-field`) is never placed by a template; of several templates a starless one
  places only its filename's stem; a lone template places any name. A memory named
  `README` now refuses (`UnplacedMember`) where it silently landed at `CLAUDE.md`.
- Location agreement with the name's prefix is checked engine-side through the read
  side's own `unit_shape_id` for `at` members only. A nested-file child may declare a
  location (relative to its host's unit) but its agreement is unchecked — no spec text
  says what a nested key's prefix is.
- The lift landed in `src/install.rs` (the scaffold), not `src/import.rs`: it writes
  `location:` only for a `:`-prefixed, non-`named-field` id. A named-field-or-file style
  whose field differs from its stem at the root (`name: Teaching Mode` in `teach.md`)
  still lifts with no location and re-emits at `Teaching Mode.md` — pre-existing, now
  fixable by widening the lift's criterion to "read path != whole placement".
- `sdk/src/prose.ts:84` still splits a nested skill's name on `:` for its invocation
  display (`/apps/web:deploy`); that is display, not placement, so it was left.
- The lock row gains a `location` column, written only when declared.
