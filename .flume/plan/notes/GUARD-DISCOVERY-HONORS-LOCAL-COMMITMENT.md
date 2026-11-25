# Guard discoverability honors a local host's commitment

Shipped: `import::discovers_path` takes the walk flavor (`local_governs`); the
local flavor waives the workspace fence and every git ignore declaration, as
`discoverable_paths(_, true)` does. `nested_file_placement` now tests
discoverability per candidate host with `local_governs(host, over)`, after
placement, instead of once with the committed flavor up front.

Observed: the two governed callers in `src/install.rs` (`pending_finding`'s
governed branch, `matches_governed_locus`) can never see a local kind —
`gate::site_class` returns `None` for `Commitment::Local`, and the binary's file
loci skip local kinds too. They now pass the governing kind's flavor for
parity, but it is always `false` there; the only live behavior change is the
hosted child under a local host. Both `tests[]` lines therefore exercise the
hosted path (edit of an existing gitignored child, and a pending create).

Debt, not taken (outside the entry): the post edge's `locus_files`
(`src/install.rs`) walks with `discovery_fence(false)` and git ignores on. It is
reached only for governed loci, which are never local today, so it is not a
live gap — but it is the same flavor question spelled a second way, should a
local governed kind ever become a guarded site.
