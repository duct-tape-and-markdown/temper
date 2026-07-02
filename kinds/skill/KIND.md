+++
governs = { root = ".claude/skills", glob = "*/SKILL.md" }

[[extraction]]
primitive = "field"
key = "name"

[[extraction]]
primitive = "field"
key = "description"

[[extraction]]
primitive = "field"
key = "version"

[[extraction]]
primitive = "field"
key = "license"

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "sections"

[[extraction]]
primitive = "placement"
+++

# The skill kind — built-in

The declared definition of the Claude Code skill kind: a directory under
`.claude/skills/` carrying a `SKILL.md` with YAML frontmatter over a markdown
body (https://agentskills.io/specification, retrieved 2026-07-01). Identity is
the `name` field; `name-matches-dir` (the package's cross-artifact clause)
holds it to the directory.

Built-in means **temper-sourced, not privileged** (`specs/15-kinds.md`): this
definition ships embedded with the crate exactly as `packages/skill.anthropic`
does — same medium, same schema as a project's own custom kind, differing only
in where it sources from. The harness adapter faces (parsing the YAML
frontmatter in, emitting it back out) remain engine code, because that format
is Claude Code's to define; everything the *contract* ranges over is declared
here. The typed fields above are the documented frontmatter; unknown keys ride
into the same feature map via permissive extraction (`specs/15-kinds.md`,
"Extending a built-in kind"), so a project convention is already extracted.

This definition is load-bearing, not descriptive: `build.rs` embeds it, the
engine composes its declared extraction over the IR-derived unit
(`src/builtin_kind.rs`) — the same generic path a custom kind reads through —
and `import` discovers the kind off its `governs` locus. The hand-coded
per-field extractor it replaced is gone; equivalence was pinned by snapshot
before the swap.
