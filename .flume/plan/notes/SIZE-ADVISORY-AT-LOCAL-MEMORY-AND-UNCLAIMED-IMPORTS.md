# SIZE-ADVISORY-AT-LOCAL-MEMORY-AND-UNCLAIMED-IMPORTS

Shipped as written. memory-local gains the same advisory as memory:
`when(loads("on-demand"), [extent(characters, 40000)])`. The root default gains
`unclaimedImportSizeClause` (exported from builtins.ts and composed into
rootDefaultContract): the same `when` shape, labelled
`root.when.loads=on-demand.extent`.

Engine seams:

- Root admissibility (`engine::rootless`) now admits one member-grain form: an
  each-grain `extent`, bare or under a `loads` guard. `judge_unclaimed_files`
  runs these over every import target no kind claims, read from disk. Every
  other member-grain clause at the root is still refused. This widens the root
  vocabulary. `representation.md`/`contract.md` still say the root carries only
  the selection grain; a spec sentence is owed if plan agrees.
- `DirectiveClassing::reached_files` is now a map from file to importer nodes.
  `graph::imported_file_moment` walks import arcs upstream: a file is on demand
  only when every member upstream carries an on-demand moment, and at launch as
  soon as one is launch-loaded. The spec doesn't settle a file imported by both
  the root and a subdirectory memory; launch wins here, because the file is
  already in context.
- A bare root `extent` sizes every unclaimed target, whatever its moment. Nothing
  ships it.

Owed: a `chore(harness)` re-emit of the dogfood `.temper/lock.toml`, since the
root rows moved. The example lock, embedded lock, gauntlet and matrix snapshots
are re-derived here.
