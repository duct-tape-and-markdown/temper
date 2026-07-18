## Surface
`architecture.md`'s codemap assigns `import.rs` "discovery" alone and
`drift.rs` "emit, the lock, drift detection" — but `import.rs`'s own module
header (`src/import.rs:1`, "Harness discovery and the `lock.toml` roll-up
writer") and its `RollupEntry` struct (160-174), `write_rollup` (585-608),
`rollup_tables` (613-624), and `create_dir_all`/`write_bytes` helpers
(627-640) implement the lock's roll-up write face inside the discovery
module. `drift.rs` is already the sole caller (constructs every
`RollupEntry` at `src/drift.rs:1310,1325`, calls `write_rollup` at
`src/drift.rs:1367`) — the call sites already sit in the module the codemap
names as owning "the lock"; only the serialization mechanics live on the
wrong side of the boundary.

## Observed at
7ac498a

## Suggested consolidation
Either move `RollupEntry`/`write_rollup`/`rollup_tables`/`create_dir_all`/
`write_bytes` into `drift.rs` (matching the codemap's stated job split,
`drift`'s caller already there), or amend `architecture.md`'s pipeline
codemap entry for `import` to name the roll-up writer as a stated second
job (the module's own header already claims it) — a page amendment is
human-ratified per Growth rules, not a call `plan` or `build` can make
unilaterally.
