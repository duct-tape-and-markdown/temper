# SKILL-CLAIMS-SUBDIRECTORY-SKILLS

Shipped: skill locus `.` + `**/.claude/skills/*/SKILL.md`. A directory unit's id is
prefixed by the segments its glob's single `**` spans, `:`-joined
(`extract::globstar_span`; `unit_shape_id` now takes the kind's glob). Placement
inverts it for a `**/`-led directory glob (drift.rs / emit.ts `placementInUnit`).

Also needed beyond the file list: `import::host_units` keyed hosts by bare dir
name and `unit_dir` took the first segment under root (wrong under a `.` root);
coverage's directory branch now covers `.claude/skills` for an any-depth locus;
install `lift_base` folds directory kinds too; the example lock was re-emitted.
`surface_subdir` left alone: it has no production caller.

Open, surfaced, not chosen (both are plan's to raise as questions):
- SDK skill `identityField: "name"` writes the identity into frontmatter, so a
  nested skill emits `name: "apps:web:deploy"`, which `skill.allowed_chars.name`
  refuses. install -> emit of a nested skill rewrites its `name`. Whether `name`
  should be the leaf, or stay tied to the identity, is undecided.
- memory `**/CLAUDE.md` excludes only the root `.claude/skills`. A `CLAUDE.md`
  inside a nested skill dir is claimed as memory (declared loci carve host
  templates), while the same file at the root is a supporting-doc.

Dogfood: `tests/fixtures/extract_equivalence/.claude/skills` is now reached and
needs the human exclude (`relocate(skill, { exclude: [...] })`). Census: it is
the one fixture tree outside a nested harness; `examples/base-harness` has its
own lock and stays fenced.
