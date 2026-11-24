# GUARD-JUDGES-AS-CHECK-DOES: residual guard/check gaps after the unification

Shipped: `guarded_loci` (main.rs) admits a kind iff `gate::site_class` says File;
the post edge runs `gate::source_dependency_stale` (layout import, include,
input) beside `config_stale_from_doc`; `locus_member_sites` walks through
`import::ignore_walk` from the harness root, bounded to each locus root's
ancestors and subtree, so the per-root walk-count pin in `check_cost.rs` still
reads 2. The gauntlet gained composition 6 (a top-level `charter` layout kind
importing prose) so the subset pin has the case the bug lived in.

Residual gaps:

- The layout half of `locus-declared` (`undeclared_layout_members_from_doc`)
  is not judged by the guard at either edge: it needs each site's member
  address, which comes off the unit-id derivation compose owns. The spec says a
  clause the guard binds it judges whole, so this is a residual gap.
- The pre edge still skips `.`-rooted loci (memory `**/CLAUDE.md`, `.mcp.json`)
  and does not consult ignore rules for its one path: a pending write may not
  exist yet for an ignore walk to admit. A gitignored path inside a
  non-`.` locus (e.g. `.claude/rules/local.md`) still binds at the pre edge
  though `check` would not discover it — that one is a superset case, a real
  (small) violation of the subset rule. Fix wants a single-path ignore query
  in `import` (outside this entry's fence).

Debt: `locus_member_sites` repeats discovery's three fences (`.git`, the
workspace, a nested governed root) because `import::discoverable_paths` and
`is_governed_root` are private; one shared filter in `import` would retire the
copy.
