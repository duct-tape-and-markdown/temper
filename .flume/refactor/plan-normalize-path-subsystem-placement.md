## Surface

`normalize_path` (src/graph.rs:929-940, doc 924-928: "Lexically normalize a
path — drop `.` and resolve `..` against a preceding normal segment —
**without touching disk**") is a pure `std::path::Component`-only leaf
utility with no dependency on any judges-subsystem type — it never touches
`Node`, `ResolvedEdge`, or a judge/selection concept. It lives in
`graph.rs` (the `judges` subsystem per `specs/process/architecture.md`'s
codemap) but its external callers are all `pipeline`-subsystem modules:
`src/import.rs` (396, 517) and `src/drift.rs` (672, 1799, 1885), each
calling `crate::graph::normalize_path`. architecture.md's codemap homes
exactly this shape of leaf vocabulary — "no internal dependencies" — in
`foundation`, alongside `address.rs`'s own path/field mechanics. None of
architecture.md's three named debt edges (drift→install,
frontmatter→builtin_kind, extract's upward imports) cover this one; it
reads as an undeclared pipeline→judges edge, or evidence `normalize_path`
is simply homed one subsystem too late.

## Observed at

4baa5c4 — plan diffs forward from here.

## Suggested consolidation

Move `normalize_path` to `src/address.rs` (foundation); update its three
pipeline call sites plus graph.rs's own three internal callers (875, 883,
921) to `crate::address::normalize_path`. Needs a human call on whether
this is worth a mechanical entry now, or should wait for architecture.md's
own three named debt edges to land first — the codemap's Invariants section
doesn't currently name this edge as debt, so filing it outright would be
asserting a reading the page hasn't yet ratified.
