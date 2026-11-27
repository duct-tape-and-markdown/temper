# Listing budget spans skills and commands

Shipped. The budget is one root clause, `root.listing.description+when_to_use`:
skills and commands summed together, each entry capped per member. The skill floor
no longer carries a listing clause.

- Shape: `listing()` takes `kinds?: KindDefinition[]` and narrows the members it
  counts by kind, subkinds included, through the judge's existing `is_kind`. When
  `kinds` is absent it counts every member. The kinds ride the `values` column,
  sharing it the way `gate` and `target` are shared, because `ClauseRow` has no
  `Default` and a new column would have touched about 140 struct literals. If a
  dedicated `kinds` column is preferred, that is a separate refactor.
- Migration-note line owed (release cut): the dial id moves from
  `skill.listing.description+when_to_use` to `root.listing.description+when_to_use`.
  Two kinds of authored contract lose the clause silently: a root `contract:`,
  because it replaces `rootDefaultContract` wholesale, and a spread of the old
  skill floor. Re-spread `rootDefaultContract` to keep it.
- The bundled reference documents kind topics only. The listing guidance has left
  `kind-skill.md`, and no topic now teaches the root's clauses. That is a gap in
  what the reference teaches.
- `explain` narrates a member's listing entry from its kind's contract, or else from
  a root `listing` clause that counts its kind (`listings_by_kind`, `src/read.rs`).
- The dogfood `.temper/lock.toml` at the repo root is outside build's paths, so a
  human must re-emit it (`chore(harness):`). Until then the repo's own lock carries
  the old skill row.
