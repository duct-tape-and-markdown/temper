# Parked: the nested rule locus is outside the placement-template grammar

`builtins.md` ("The shipped kinds", 0115) has `rule` claim
`**/.claude/rules/**`: rules nested at any depth, discovered recursively
inside each folder. Written as a locus that is root `.` with glob
`**/.claude/rules/**/*.md`. That glob has a second, interior `**`, and
`representation.md` ("locus", 0081) only allows "literal segments with at
most one `*`, optionally under a leading `**/`". Anything else is
refused at load. In code, `glob::is_placement_template` refuses it, and
`CustomKind::from_kind_fact_row` and `drift::splice_name` would too.

The two spellings that fit the grammar each break a cited fact:
- `**/.claude/rules/*.md` finds nested folders but stops recursing inside
  `.claude/rules/`. That drops `.claude/rules/frontend/react.md`, which the
  memory docs say is discovered and which ships today.
- The current `.claude/rules` + `**/*.md` recurses but never finds a
  subdirectory's own `.claude/rules/`, which is this entry.

Shipping it needs a spec ruling, which a build tick can't make. The
recommendation is to widen the template grammar to allow one interior
`**` after literal segments. The splice drops each `**` to zero segments
on placement. Identity is every span a wildcard covers, joined with `:`,
leaving out the literal folders. So
`apps/web/.claude/rules/frontend/x.md` reads as `apps:web:frontend:x`,
and the root case keeps its `frontend:x`. A `:`-bearing name already
places only by its declared location, and `refuse_disagreeing_location`
checks that round trip. `yields_to_folder_claim` would also need to accept
a claim whose folders end in `**`, so that memory's `**/CLAUDE.md` and
`**/AGENTS.md` cede to the rule's claim. That is "memory's exclusions
follow".

Nothing was changed in `src/`, `sdk/` or `tests/`.
