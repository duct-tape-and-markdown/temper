# Built-in locus cites re-verified; two loci the current docs contradict

Every file kind's locus doc in `sdk/src/builtins.ts` now cites the page and section
naming its path, retrieved 2026-11-24. Cite-only, per the entry: two loci the docs
contradict are left unmoved and need their own entries with tests.

- **command**: the locus is `.claude/commands` + `*.md` (top level only). The
  skills page, "How a skill gets its command name" table, has a row "File in a
  subdirectory of `.claude/commands/`": "Subdirectory path relative to `commands/`
  with each `/` replaced by `:`, then the file name without extension",
  `.claude/commands/frontend/component.md` → `/frontend:component`. So the glob should
  be `**/*.md`, and the stem identity misses the `frontend:` prefix (two `x.md` in
  different subdirs give distinct commands but collide on the stem).
- **skill**: the locus is root `.claude/skills` + `*/SKILL.md`. The skills page,
  "Load skills in monorepos and subdirectories", documents a "Nested" row,
  `<subdir>/.claude/skills/<skill-name>/SKILL.md`, loaded "the first time Claude reads
  or edits a file in that subdirectory", invoked as `/apps/web:deploy`. Nested skill
  dirs are unclaimed (same question for `agents`: the sub-agents page walks *up* from
  cwd, so a nested `.claude/agents/` loads when a session starts below it).
  Note that the memory locus's exclude list only names root `.claude/*` folders, so a
  `CLAUDE.md` under `<subdir>/.claude/skills/` looks claimed as memory (unverified).

Also: `/docs/en/slash-commands` now serves the skills page, so the spec's
slash-commands cite (2026-07-22) points at content that no longer exists separately.
