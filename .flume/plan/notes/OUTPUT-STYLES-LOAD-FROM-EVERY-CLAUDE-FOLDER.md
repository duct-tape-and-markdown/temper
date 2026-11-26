# OUTPUT-STYLES-LOAD-FROM-EVERY-CLAUDE-FOLDER: shipped, three loose ends

Shipped: locus `**/.claude/output-styles/*.md`; a named-field-or-file id at a `**`
locus is the `**` span then the name or bare stem (extract.rs `unit_shape_id`); emit
places `apps:web:terse` at `apps/web/.claude/output-styles/terse.md` (drift.rs
`placement_in_unit`) and writes the leaf into frontmatter `name` (sdk kind.ts
`identityFieldValue`); new peer predicate `unshadowed` (advisory, label
`output-style.unshadowed`, dialable); memory cedes `**/.claude/output-styles/*.md`
via a new `cede_folder_claims` (compose.rs, kind.rs `yields_to_folder_claim`). graph.rs
needed no change: the root `terse` is the only member with that identity.

Loose ends:
- `.temper/lock.toml` (dogfood) is outside build's writable paths and is now stale
  (memory `ceded`, output-style locus, the new clause row). It needs a
  `chore(harness):` re-emit.
- A root style whose frontmatter `name` holds a `:` (`name: "Mode: Teach"`) reads
  back fine but emits as nested: the leaf split and placement both read `:` as a
  placement separator. 0096 reserves `:` in path segments, not in names. Should a
  name holding `:` be refused, escaped, or left as a known limitation?
- The `**`-span prefix applies to every named-field-or-file kind at a `**` locus,
  custom kinds included (0096 is "one identity grammar"). No custom kind in the tree
  hit it.
