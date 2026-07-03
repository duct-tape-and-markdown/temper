+++
provider = "claude-code"
governs = { root = ".claude/rules", glob = "*.md" }
format = "yaml-frontmatter"
unit_shape = "file"
activation = { via = "paths-match", field = "paths" }

[[extraction]]
primitive = "field"
key = "paths"

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "sections"

[[extraction]]
primitive = "placement"
+++

# The rule kind — built-in

The declared definition of the Claude Code rule kind: a markdown file under
`.claude/rules/`, optional YAML frontmatter whose one documented key is
`paths` — glob-scoped loading — over an instruction body
(https://code.claude.com/docs/en/memory#path-specific-rules, retrieved
2026-07-01). Identity is the file stem; a rule with no frontmatter loads
unconditionally. Activation is the path match: "path-scoped rules trigger
when Claude reads files matching the pattern," and "rules without a `paths`
field are loaded unconditionally" (same doc, retrieved 2026-07-02) — so an
absent field is a live `always`-shaped edge, and only a present glob set
matching zero repo files is dead (`specs/45-governance.md`, reachability).

Kind identity is qualified `claude-code.rule` — the `provider` header names
the authority that defines the format, and placement mirrors identity
(`specs/15-kinds.md`, "Decision: kind identity carries a provider axis"); a
bare `rule` resolves here while unique in the assembly.

Built-in means **temper-sourced, not privileged** (`specs/15-kinds.md`): this
definition ships embedded beside `packages/rule.anthropic`, same medium and
schema as a custom kind, differing only in source. The adapter faces stay
engine code (the frontmatter format is Claude Code's); unknown keys — the
Cursor `.mdc` keys the package's `forbidden_keys` clause exists to catch —
ride into the feature map via permissive extraction.

This definition is load-bearing, not descriptive: `build.rs` embeds it, the
engine composes its declared extraction over the IR-derived unit
(`src/builtin_kind.rs`) — the same generic path a custom kind reads through —
and `import` discovers the kind off its `governs` locus. The hand-coded
per-field extractor it replaced is gone; equivalence was pinned by snapshot
before the swap.
