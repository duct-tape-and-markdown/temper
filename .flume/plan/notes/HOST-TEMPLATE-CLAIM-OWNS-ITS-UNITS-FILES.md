# HOST-TEMPLATE-CLAIM-OWNS-ITS-UNITS-FILES

Shipped: `compose::cede_builtin_host_units` (run inside `declared_kinds_with_overlaid`,
after the subkind cede) pushes each built-in directory-unit host's unit subtree
(`kind::unit_subtree_pattern`: `**/.claude/skills/*/SKILL.md` -> `**/.claude/skills/*/**`)
onto every other governing kind's `ceded`, the host's own subkinds excepted. It reads
the overlaid locus, so a relocated skill cedes where it lives. It reaches check via
`assemble_lock_family`'s existing ceded copy-back, and schema/install via
`effective_kinds`.

Open edges for plan:
- Emit's placement (`drift::member_projection_path`) still derives ceded patterns only
  from rows naming the kind as `base`; it doesn't see the host cede. A program placing a
  memory inside a skill's directory would emit a file check no longer discovers. Only a
  contrived nested-memory name reaches it, but "emit never writes what check won't find"
  is the invariant. Rows don't say which kinds are built-ins, so closing this needs a
  decision on where the drift side learns that.
- The cede applies to custom kinds as well as built-ins ("every other kind"). A custom
  kind that deliberately claims files inside a skill's directory now loses them. I
  checked no test or fixture does that today.
- `builtin_kind::definition("memory")` alone doesn't carry the cede, since it's derived
  at composition. Callers that walk a raw definition (the older builtin_loci tests do)
  see only the literal exclusions.
- Folding 0084's literal `.claude/skills` exclusion into the cede is still a candidate:
  the root skill's subtree `.claude/skills/*/**` is now ceded too.
