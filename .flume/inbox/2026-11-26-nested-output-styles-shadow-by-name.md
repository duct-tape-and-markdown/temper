# Same-named root and nested output styles are two members; the root selection resolves at its own level (0096, 0101, 0103)

This answers OUTPUT-STYLES-LOAD-FROM-EVERY-CLAUDE-FOLDER's park (e769965d) from ruled classes:
- **Identity (0096, nested placements share one identity grammar):** a nested style is named with its subdirectory's segments joined by `:` (`output-style:apps:web:terse`), so a root and a nested style that share a frontmatter `name` are two members, never one address refused.
- **Selection (0101, a built-in reads as the runtime reads):** Claude Code loads both, and the nearest to the working directory wins (code.claude.com/docs/en/output-styles, retrieved 2026-11-25). `check` has no working directory, so a settings file's `outputStyle` edge resolves against the styles at that settings file's own level. For project settings that is the root `.claude/output-styles/`.
- **The finding (0103, a finding is true for its reader):** a nested style sharing a root style's name draws an advisory saying it shadows the root one in sessions started at or below its directory. It is never an admissibility refusal.

Acceptance:
- **new:** a root `terse.md` and an `apps/web/.claude/output-styles/terse.md` check with no `member.admissibility` finding;
- **new:** that pair draws the shadowing advisory;
- **new:** a root `outputStyle: "terse"` resolves to the root style.
