## Surface

Two independent glob engines for one job ("does this glob select this
path"): src/kind.rs:429 `glob_matches` — hand-rolled backtracking matcher,
`*` only, single path segment (callers in src/import.rs and
src/coverage_note.rs split on `/` and match per-segment) — vs
src/graph.rs:641 `glob_to_regex` + :630 `glob_matches_any` — anchored regex
handling `**`, `*`, `?` over whole paths.

## Observed at

0ccba8d

## Suggested consolidation

One glob surface on `globset` (sanctioned; already in the tree transitively
via `ignore` — Cargo.toml's own comments demand the solved mechanic over a
hand-roll), homed once; per-segment callers stop splitting.
