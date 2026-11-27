<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `memory` kind

a `paths:` frontmatter block belongs on a `.claude/rules/*.md` file, not on `CLAUDE.md` — the memory docs document `paths` only for rules, so a rules-style header on `CLAUDE.md` is dead configuration. Split a large file with `@path` imports (resolved relative to the importing file, absolute allowed, recursion capped at four hops; wrap a path in backticks to mention it without importing). If the repo already ships an `AGENTS.md` for other agents, don't duplicate it — create a `CLAUDE.md` that `@AGENTS.md`-imports it (or symlink, except on Windows where the import is the recommended bridge). Mind the loading asymmetry: every ancestor `CLAUDE.md` loads in full at launch, while files in subdirectories load only when Claude reads a file there — so a rule that must always hold belongs above the working directory, not below it. Personal, un-shared notes go in `CLAUDE.local.md` (gitignored), appended after `CLAUDE.md` at its level.

## Locus

Each member is its own document, at:

- `**/CLAUDE.md`
- `**/AGENTS.md` — at launch, loaded only where none of `CLAUDE.md`, `.claude/CLAUDE.md`, `CLAUDE.local.md` sits in the working directory or above; in a subdirectory, on demand only where that subdirectory has none of its own; an import still reaches it

## Clauses

The program binds no clauses over it — any default contract the kind ships is what holds it.

## Requirements

- `orientation` (a member of this kind fills it) — read [requirement-orientation.md](requirement-orientation.md).
