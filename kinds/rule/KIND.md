+++
governs = { root = ".claude/rules", glob = "*.md" }

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
unconditionally.

Built-in means **temper-sourced, not privileged** (`specs/15-kinds.md`): this
definition ships embedded beside `packages/rule.anthropic`, same medium and
schema as a custom kind, differing only in source. The adapter faces stay
engine code (the frontmatter format is Claude Code's); unknown keys — the
Cursor `.mdc` keys the package's `forbidden_keys` clause exists to catch —
ride into the feature map via permissive extraction.

This definition must extract feature-for-feature what `rule_features`
(`src/extract.rs`) extracts today; the unification wave retires that function
against this file, equivalence pinned by snapshot before the swap.
