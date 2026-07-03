+++
# memory.anthropic — the built-in package for the `claude-code.memory` kind (`CLAUDE.md`).
# PRODUCT SOURCE, human-curated (specs/architecture/10-contracts.md: "named for its
# source"); the build embeds this as the shipped std-lib default, never writes it.
# Binds the qualified kind `claude-code.memory` — a bare `memory` binding collides
# with `agents-md.memory`, so publication qualifies.
# CLAUDE.md carries no frontmatter, so no field-shaped clause applies; the one decidable
# contract is a body-size budget. All sources retrieved 2026-07-02.

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 200
guidance = "CLAUDE.md is always-on context, paid every session. The memory docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' For each line ask: would removing it cause Claude to make mistakes? If not, cut it. (Advisory: Claude Code loads the file in full regardless of length; this is a context-cost budget, not a hard cutoff.)"
source = "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-02)"
+++

# memory.anthropic

Anthropic's documented contract for a project `CLAUDE.md`, as decidable clauses —
sourced from the Claude Code memory docs. Adopt, extend, fork, or ignore: data,
never a hardcoded check.

**Deliberately near-empty, because the format is.** `CLAUDE.md` is plain markdown
with no documented frontmatter and no required fields
(https://code.claude.com/docs/en/memory, retrieved 2026-07-02), so there is no
schema to gate — manufacturing a required field or a forbidden-key list would
fake a check the format does not carry (`specs/intent/00-intent.md`, law 3:
decidable clauses only — a gate that guesses cries wolf). The single clause is a
context-cost budget; everything else the contract could say is guidance.

What the clauses cannot carry, as guidance: a `paths:` frontmatter block belongs
on a `.claude/rules/*.md` file, not on `CLAUDE.md` — the memory docs document
`paths` only for rules, so a rules-style header on `CLAUDE.md` is dead
configuration. Split a large file with `@path` imports (resolved relative to the
importing file, absolute allowed, recursion capped at four hops; wrap a path in
backticks to mention it without importing). If the repo already ships an
`AGENTS.md` for other agents, don't duplicate it — create a `CLAUDE.md` that
`@AGENTS.md`-imports it (or symlink, except on Windows where the import is the
recommended bridge). Mind the loading asymmetry: every *ancestor* `CLAUDE.md`
loads in full at launch, while files in *subdirectories* load only when Claude
reads a file there — so a rule that must always hold belongs above the working
directory, not below it. Personal, un-shared notes go in `CLAUDE.local.md`
(gitignored), which is appended after `CLAUDE.md` at its level. (All the above:
https://code.claude.com/docs/en/memory, retrieved 2026-07-02.)
