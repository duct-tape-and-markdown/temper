# The guard's kind set is the effective set; emit placement still cedes nothing

Shipped: `guarded_loci` (main.rs) and `guarded_layout_loci` (install.rs) now read
`compose::effective_kinds` instead of re-spelling overlay + row lift, so subkinds
resolve and bases cede. The free `kind::locus_admits` takes a `ceded` slice;
`CustomKind::cedes` is retired (its one caller, `import::discover_kind_files`, asks
`locus_admits` with `kind.ceded`). The guard's file-locus match reads excluded and ceded
off the kind set by name (`install::locus_admits`, replacing `kind_excluded`).

Left open: emit placement (`drift::member_projection_path`, drift.rs ~l.1656) holds a
`KindFactRow`, which carries no ceded set, so it passes `&[]`. A base member placed at a
path inside a subkind's locus would be written and then read back as the subkind's.
For a one-segment name under a fixed-depth template this cannot happen; under a `**`
base glob placing into a subdirectory it might. Closing it means either threading the
effective kind set into placement or deriving a base's ceded patterns from the rows'
`base` column there — and choosing the refusal's wording (today's `ExcludedProjection`
would misname it). Worth an entry if plan judges the shape reachable.

Also: `main.rs` `guarded_manifests` and `container_kinds` still assemble their own kind
sets (overlay + `from_kind_fact_row`); a manifest kind is never a subkind's base today,
so it is not a ceding bug, but it is the same duplicate shape.
