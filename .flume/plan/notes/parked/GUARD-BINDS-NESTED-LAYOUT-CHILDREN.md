# Guard binds nested layout children: the fence omits src/import.rs, which the remaining segment must touch

Landed already (39a05bae): both edges name an undeclared hosted child that exists on
disk, through check's own `import::discover_nested_file` and the shared
`compose::hosted_member_address`; the gauntlet `spec` pin agrees at both edges.

Remaining: a pending Write that *creates* a hosted child (e.g.
`docs/guides/operate/stray.md` on `nested_layout_child_harness`) passes silently —
`LayoutGuard::hosted_member` (src/install.rs) short-circuits on `!target.is_file()`
because discovery only finds files on disk. Binding it needs a placement test over
check's host-unit enumeration, which is private to src/import.rs (`host_units`,
`unit_dir`, `nested_file_members`' host loop). Proposed:
`import::nested_file_placement(disc, kind, kinds, over, path) -> Option<NestedFileUnit>`,
matching the host template pattern under each `unit.dir`, minus `unit.entry` and the
claimed paths.

Why parked, not built around: this tick's fence is install.rs, compose.rs and tests.
Rebuilding host units in compose.rs from the public pieces (`discover_kind_files`,
`locus_root`, a copy of `unit_dir`, the nested-host recursion) would be a second
host derivation, and the entry's notes rule that out ("Reuse check's host
derivation, never a second one").

For plan: add `src/import.rs` to `files.edit` and re-queue. Or, if creating a new
nested child at the pre edge is out of this entry's acceptance (that criterion is
green on trunk for existing children), retire the entry and file the new-file case
as its own entry.
