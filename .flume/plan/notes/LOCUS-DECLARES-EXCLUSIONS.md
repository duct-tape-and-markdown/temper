# LOCUS-DECLARES-EXCLUSIONS shipped; the guard reads exclusions off the kind map, not GuardedLocus

The seam is in. `at` loci take `exclude`, the row gains an `exclude` column, and
`CustomKind::excluded` is lifted in `from_kind_fact_row`. At load, each entry must be
a starless path that the locus pattern matches. `import::discover_kind_files`
subtracts the entries, and `check`'s locus-declared half reads that subtracted
discovery. `gate.rs` needed no edit.

I did not follow the plan's shape for the guard. The entry said `main.rs guarded_loci`
should carry exclusions onto `GuardedLocus`, but `tests/it/check_cost.rs:1623` builds
`GuardedLocus` literally, and that file is outside the fence. A new field would not
compile. Instead, both guard edges look up the kind's exclusions in
`GuardedLayoutLoci::kinds`, the overlaid built-ins plus custom kinds that
`guarded_layout_loci` already builds under the same gate (`install::kind_excludes`).
`main.rs` is untouched.

Debt: `guarded_loci` (main.rs) and `guarded_layout_loci` (install.rs) each build the
same effective kind set. Folding them into one `CustomKind`-carrying locus list,
with `check_cost.rs` fenced, would retire the name lookup.

A relocation delta cannot author `exclude`. `relocatedLocus` spreads the base
locus, but built-ins exclude nothing. A future built-in with exclusions would be
refused at load once relocated to a root its paths no longer sit under.

A row that excludes nothing omits the key, rather than writing `exclude: undefined`.
`sdk/test/emit.test.ts:214` deep-equals a row with no such key.
