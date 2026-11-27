# Every verb reads committed rows through one retiring door

`compose::committed_declarations_from_doc` (doc in hand) and
`compose::read_committed_declarations` (workspace path) are now the one
read of committed declaration rows. Both retire another engine's stale
built-in rows. gate, explain, and main's `schema` and `guard` arms all go
through them, and `retire_stale_builtin_rows` is private to compose.
`schema` was a third raw reader the entry didn't name; it now reads the
same way.

`drift::read_declarations` has no src caller left. A future verb could
reach for it and quietly skip retirement. Tests still call it 84 times,
so it stays for now; consider making it test-only.

The divergence explain actually showed pre-fix: a collision row never
overlays (`overlay_builtin_kind` skips it), but `builtin_contract` took
the stale row's `guidance`. The test pins that path. explain also stopped
reading the lock twice (it used `read_lock_document` and then
`read_declarations`).
