# Guard binds nested layout children: a pending write that creates one is still unbound

Landed: `guarded_layout_loci` now returns `GuardedLayoutLoci`. It admits governs-less
layout kinds (`hosted`) beside the governed ones, along with the effective kind set. The
post edge names every undeclared hosted child, using check's own nested-file scan
(`import::discover_nested_file`) and one shared `Discovery` (at most 2 walks per call,
pinned). Addresses come from `compose::hosted_member_address`, which shares
`nested_host_and_base` and `site_id` with check's `resolve_kind_units` branch and with
`governed_member_address`, so the derivation exists once. The pre edge binds a write to
an existing hosted child. The gauntlet subset test now seats an undeclared `spec`, and
both edges name it.

Next: a pending write that *creates* a hosted child (a new file under a host unit) passes
silently. `discover_nested_file` only finds files on disk. Binding a new child needs the
host-unit enumeration (`import::host_units`, which is private) exposed as a placement test.
One way: `import::nested_file_placement(disc, kind, kinds, over, path) -> Option<NestedFileUnit>`,
sharing `nested_file_members`' host loop and matching the pattern under `unit.dir`, minus
the entry and claimed paths. `src/import.rs` was outside this tick's fence, so the next
tick needs it in `files.edit`. Then `LayoutGuard::hosted_member` (src/install.rs) drops its
`target.is_file()` short-circuit and calls the placement test instead. Its test is a
Write payload for `docs/guides/operate/stray.md` on `nested_layout_child_harness`
(tests/it/install.rs), plus a new-file case in the gauntlet pre-edge list.
