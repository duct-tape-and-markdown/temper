# 0064 — a file temper does not write joins as a declared input

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(external-commitment)` fork (GH #29). An adopter wanted scripts as
roster members: committed, not an emit target, taking a lock row. Every
locus shape failed one of the three. The adopter's workaround was a
member whose unit is its own record, carrying the script's path as a
field — "arguably the honest shape". What that shape lacked was a
fingerprint on the script, which 0055's declared inputs now supply. A
field report on 0.0.18 hit the same fork through a read-only `source`
kind over a committed code snapshot. Ruled by John 2026-09-29 on the
session's recommendation.

## Decision

**No new commitment class.** A committed file temper does not write
joins the roster through a member whose record temper writes and which
declares the file as an input. The file is fingerprinted, its drift
names the member and routes to re-verifying the member's claims, and the
member is the lock row and the edge target. `authoring.md` states it
beside inputs.

## Rejected

- **A `commitment: "external"` file-locus class.** It makes one path both
  a governed member's locus and a file temper never writes, which
  invariant 7 forbids, and duplicates what 0055 already fingerprints.
- **A layout kind over the foreign file.** A layout parses a markdown
  heading tree; over code, a line such as `#region` would read as a
  heading — structure mined from content.

## Consequences

The GH #29 answer is the record-plus-input shape, documented. A
`source` kind over a committed snapshot can retire in favor of inputs on
the members whose claims rest on it.
