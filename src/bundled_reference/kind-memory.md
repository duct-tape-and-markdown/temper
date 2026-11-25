<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `memory` kind

a `paths:` frontmatter block belongs on a `.claude/rules/*.md` file, not on `CLAUDE.md` — the memory docs document `paths` only for rules, so a rules-style header on `CLAUDE.md` is dead configuration. Split a large file with `@path` imports (resolved relative to the importing file, absolute allowed, recursion capped at four hops; wrap a path in backticks to mention it without importing). If the repo already ships an `AGENTS.md` for other agents, don't duplicate it — create a `CLAUDE.md` that `@AGENTS.md`-imports it (or symlink, except on Windows where the import is the recommended bridge). Mind the loading asymmetry: every ancestor `CLAUDE.md` loads in full at launch, while files in subdirectories load only when Claude reads a file there — so a rule that must always hold belongs above the working directory, not below it. Personal, un-shared notes go in `CLAUDE.local.md` (gitignored), appended after `CLAUDE.md` at its level.

## Locus

Each member is its own document, at:

- `**/CLAUDE.md`

## Clauses

- `extent` (advisory): CLAUDE.md is always-on context, paid every session. The memory docs' size target is under 200 rendered lines per memory file — 'longer files consume more context and reduce adherence.' Measured render-side, off the projected artifact rather than the source body. For each line ask: would removing it cause Claude to make mistakes? If not, cut it. (Advisory: Claude Code loads the file in full regardless of length; this is a context-cost budget, not a hard cutoff.)
