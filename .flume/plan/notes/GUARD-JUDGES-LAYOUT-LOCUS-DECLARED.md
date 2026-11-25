# Guard judges the layout half of locus-declared at both edges

Shipped. Both edges call `drift::undeclared_layout_members_from_doc`, the judge
`check` uses, so the guard's words match check's. Member addresses come from
`compose::governed_member_address`, which uses the same fold base as check's
`governs` scan (`governed_fold_base`, now shared with `resolve_kind_units`).
`install::guarded_layout_loci` admits kinds through `gate::site_class == Layout`.
The post edge shares one bounded walk helper (`install::locus_files`) with the
file-locus sites.

Narrower than check, never wider (the gauntlet pin covers this):
- **Nested layout children are not bound** (a layout kind with no `governs`,
  whose locus and address come from a host unit, e.g. gauntlet `spec`). Binding
  them means running host discovery at guard sites. That is the next tick if
  "judges whole" reaches that far.
- **`named-field` layout kinds are not bound at the pre edge.** Their identity is
  in the pending bytes' frontmatter, and the frontmatter parser is private to
  `frontmatter.rs`. The post edge binds them because it reads the document on
  disk. A `pub(crate)` text-level id derivation in `frontmatter.rs` would close
  this gap.
- `.`-rooted layout loci skip at the pre edge, the same rule as the file-locus
  half.
- The post edge reads a site's identity but not whether its body fits the layout.
  If a stray's body does not fit, check may refuse loud where the guard still
  names it. This edge case is unpinned.

Fixed signatures: `shell_edge_findings` derives the layout loci itself, because
`tests/it/check_cost.rs` (outside the fence) calls it with three args.
`install::guard` gained a `&LayoutGuard` param.
