## Surface

MEMBER-ADDRESS-GRAMMAR-ONE-HOME's files[] carries an eight-site hand-parse
inventory. This tick closed two of them, so that entry inherits six:

- `gate.rs:627` — the function that held it (`embedded_hosts_by_source`) is
  deleted; its consumer now calls `graph::embedded_hosts_by_key`.
- `graph.rs:333` (`edge_host`'s `nested.host.split_once(':')`) — folded into
  the new reader `graph::embedded_source_host`, which reads the halves
  `parse_nested_address` already splits at what was `graph.rs:1703`. That
  split is now the reader half the entry predicted it would become, so it is
  no longer a *duplicate* site — it is the one home.

Still open, unverified here: `graph.rs:1348`, `:1624`, `read.rs:238`,
`:1994`, `drift.rs:892` (line numbers pre-date this commit's edits to
`graph.rs`/`read.rs`; re-verify before scoping).

## Observed at

1d480a99 (HEAD when observed), closed in this tick's `build:` commit.

## Suggested consolidation

`graph::embedded_source_host` / `graph::nested_key` are the reader halves the
remaining sites should route to; `read.rs`'s two want a `kind:name` reader
(the `<host-address>` grammar) that still has no home.
