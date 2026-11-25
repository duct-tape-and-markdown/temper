<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `command` kind

a command loads as a skill does: its `description` is listed from session start, and its body reaches context only when invoked — by `/` plus its path under `commands/` with each `/` a `:`, or by Claude matching the description. It is one file, so it bundles no supporting files; reach for `.claude/skills/<name>/SKILL.md` when the procedure needs reference material loaded on demand.

## Locus

Each member is its own document, at:

- `.claude/commands/**/*.md`

## Clauses

The program binds it an empty contract — no clause holds its members.
