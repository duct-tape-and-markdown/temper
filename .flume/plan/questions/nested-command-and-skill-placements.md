# Do the command and skill kinds reach the nested placements the current docs document?

`specs/builtins.md`, "The shipped kinds", gives **command** identity "from the file stem" at `.claude/commands/*.md` (top level), and **skill** a root `.claude/skills/*/SKILL.md`. The current skills page (code.claude.com/docs/en/skills, retrieved 2026-11-24) contradicts both:

- "How a skill gets its command name": a file in a subdirectory of `.claude/commands/` is named by its subdirectory path with `/` → `:` (`.claude/commands/frontend/component.md` → `/frontend:component`). The kind misses those files, and stem identity would collide two `x.md` in different subdirectories.
- "Load skills in monorepos and subdirectories": a nested `<subdir>/.claude/skills/<name>/SKILL.md` loads on first touch of that subdirectory, invoked `/apps/web:deploy`. Unclaimed today. `agents` may have the same reach (the sub-agents page walks up from cwd). Memory's excludes name only the root `.claude/*` folders, so a `CLAUDE.md` under a nested `.claude/skills/` may read as memory (UNVERIFIED).

The spec's `docs/en/slash-commands` cite (2026-07-22) now serves the skills page.

This is an identity-grammar change to a spec section, so it is human-authored. Options: (1) widen both loci and give a nested command a path-derived identity; (2) keep the top-level loci and name nested placements as unmodeled. Recommendation: 1, because a harness's own loaded documents being ungoverned is the gap temper exists to close. Sections: `specs/builtins.md` "The shipped kinds"; `specs/model/representation.md` "locus".
