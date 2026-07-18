## Surface

`src/drift.rs` implements the same "walk the committed lock's declaration
rows" job three separate times, each independently opening `lock.toml`,
tolerant-parsing it as `DocumentMut`, ranging over every top-level
`(kind, item)` pair, filtering to `item.as_array_of_tables()`, and pulling
columns off each row via `row.get(..).and_then(Item::as_str)`:

- `read_prior_provenance` (fn at 1918, loop body 1928-1946) — requires
  `name`+`source_path`+`emit_hash`, returns `Vec<ProvenanceRow>`.
- `config_stale` (fn at 2342, loop body 2355-2384) — same three columns,
  inlines the staleness diagnostic instead of returning rows.
- `emit_owned_targets` (fn at 2614, loop body 2624-2641) — only
  `name`+`source_path` (no `emit_hash`), returns `Vec<EmitOwnedEntry>`.

The doc comments on all three already cross-reference each other's
tolerant-absence behavior (1916-1917, 2611-2612), so the duplication is
self-aware in the code but never consolidated. A shared raw-row walk
(`(kind, Option<name>, Option<source_path>, Option<emit_hash>)`) that each
of the three maps/filters over would let `config_stale`'s and
`emit_owned_targets`' looser column requirements fall out of one `filter_map`
each, instead of re-deriving the walk. Consumers (`src/install.rs`,
`src/main.rs`, `tests/install.rs`) call only the three public fns, so the
consolidation is internal to `drift.rs` — no ripple.

## Observed at

b8fc7ca (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation

A private `fn walk_lock_rows(workspace_dir: &Path) -> Vec<RawLockRow>` (raw,
all-`Option` columns) that `read_prior_provenance`, `config_stale`, and
`emit_owned_targets` each filter/map over — the home stays `drift.rs`, this
is a same-file `fn`-level unification, not a new module.
