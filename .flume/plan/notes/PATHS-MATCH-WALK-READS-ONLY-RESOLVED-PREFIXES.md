# paths-match walk scoped to resolved glob prefixes

Shipped: `compose::repo_file_set(root, globs)` walks only each glob's literal
directory prefix (segments before the first carrying `* ? [ ] { } \`, never
the file-name segment); `graph::registration_globs` collects every
`paths-match`/`paths-listing` glob across `by_kind`, and both callers (gate,
explain's `reachability_orphaned`) hand it in. Still raw disk. A symlinked
prefix contributes nothing, matching the unfollowed whole-tree walk.
Counter picked: `compose::repo_file_set_entries_read` (entries walkdir
yields, dirs included) — a per-mechanism counter, which rust.md's
"count pin per verb at the owners" bar says is the wrong holder long term;
walkdir has no owner yet.

Observed debt:
- A prefix-less glob without `**` (`*.md`, `*/x.md`) still walks the whole
  tree; its segment count bounds the depth it can match, so a `max_depth`
  per prefix would close that cheaply. Not taken (scope).
- An uncompilable glob is treated as matching by `dead_registration` yet
  still widens the walk via its prefix; harmless, slightly wasteful.
