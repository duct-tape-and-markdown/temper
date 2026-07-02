<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- BUG, high priority (first external-harness contact, found by a second
  project's import attempt): `import`'s scan roots are asymmetric — skills at
  `<harness>/skills/*/SKILL.md` (a slice-1 fixture leftover / plugin layout)
  but rules at `<harness>/.claude/rules/*.md` — so NO single harness_path
  captures a standard Claude Code project (`.claude/skills/` + `.claude/rules/`).
  Fix per the locus doctrine (specs/15-kinds.md, emit/read face; spec line now
  corrected in specs/20-surface.md): harness_path is the PROJECT ROOT and every
  built-in kind scans its real Claude Code locus — `.claude/skills/*/SKILL.md`,
  `.claude/rules/*.md`. Session-start/check --harness inherit the fix. Keep a
  bare-skill-dir / skills-tree arg working only as explicitly-plugin-shaped
  input if cheap, else drop it. Note temper's own repo has no .claude/skills/,
  so add a fixture harness WITH both, plus a test that one import of a standard
  project captures skills and rules together. Repro: any repo with
  .claude/skills/ — import from root gets 0 skills; import from .claude/ gets
  0 rules.
