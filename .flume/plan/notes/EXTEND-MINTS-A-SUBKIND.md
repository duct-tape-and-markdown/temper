# Subkind shipped: base column, ceded loci, load-time refusal

Row: `KindFactRow.base` (absent on relocations, so builtin_lock.toml is
unchanged). Engine: `compose::resolve_subkinds` runs inside
`declared_kinds_with_overlaid`, bases first. It fills a subkind's undeclared
facts from its base, refuses an unknown base, a cycle, or a locus outside the
base's (`LockRowError::Subkind`), and pushes the subkind's pattern onto the
base's new `CustomKind::ceded`. `discover_kind_files` and
`CustomKind::locus_admits` subtract `ceded`, and `assemble_lock_family` copies
it back onto `overlaid_builtin_kinds`. Containment is `kind::locus_within`,
a witness test over the placement-template grammar. The SDK's `locusWithin`
mirrors it.

Gaps BASE-SELECTIONS-BIND-A-SUBKIND or later ticks should know about:
- The guard's locus set (`main.rs` guarded loci ~l.850, `install.rs` ~l.1750)
  builds its own kind set from `overlay_builtin_kind` + `from_kind_fact_row`,
  so it never sees `ceded`. A pending write in a subkind locus matches both
  kinds there. That is a duplicate of `declared_kinds_with_overlaid`, and
  consolidating it is the fix.
- The free `kind::locus_admits(pattern, excluded, path)` (emit placement,
  guard) does not cede.
- `extend` has no `fields` face, so "adds fields" from the spec is unbuilt.
  It also refuses a non-`at` base.
- Containment checks templates only, never the base's excludes. A subkind
  inherits its base's commitment class when its row declares none.
