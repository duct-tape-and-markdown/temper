+++
# memory.agents-md — the built-in package for the `agents-md.memory` kind (`AGENTS.md`).
# PRODUCT SOURCE, human-curated (specs/architecture/10-contracts.md: "named for its
# source"); the build embeds this, never writes it. Binds the qualified kind
# `agents-md.memory` — a bare `memory` binding collides with `claude-code.memory`.
# NO CLAUSES: the AGENTS.md standard defines no schema to gate — see the body. Author's
# note: a `max_lines` gate was considered and rejected (the only real size limit is
# Codex's combined-byte chain budget, not a per-file line count — faking it would break
# law 3). All sources retrieved 2026-07-02.
+++

# memory.agents-md

The AGENTS.md standard's contract for a memory file — which is that there is
almost none. Adopt, extend, fork, or ignore: data, never a hardcoded check.

**Guidance-only, and that is the honest encoding.** `AGENTS.md` "is just standard
Markdown" with no required fields, no sections, and no frontmatter
(https://agents.md/, retrieved 2026-07-02); the format deliberately constrains
nothing. A package that manufactured a required field, a size gate, or a
forbidden-key list would assert a contract the standard disclaims
(`specs/intent/00-intent.md`, law 3: decidable clauses only — never fake a gate).
So this package carries zero clauses and speaks only in guidance. Even the
tempting size number is a *tool's* rule, not the format's — see below.

Real-world reading behavior worth knowing — none of it a clause you can honestly
write over a single file:

- **Nested, nearest-wins.** Agents read the closest `AGENTS.md` in the tree, so
  every subproject can ship a tailored file (https://agents.md/, retrieved
  2026-07-02). Put directory-specific guidance in that directory, not the root.
- **Codex** concatenates the chain root-to-cwd and stops once combined size hits
  `project_doc_max_bytes` (32 KiB default), skips empty files, and reads an
  optional `AGENTS.override.md` ahead of `AGENTS.md`
  (https://developers.openai.com/codex/guides/agents-md, retrieved 2026-07-02).
  That budget is a *combined byte* limit across the chain, not a per-file line
  count — which is exactly why it is not a clause here.
- **Gemini CLI** reads `GEMINI.md` by default and only treats `AGENTS.md` as an
  alias when `context.fileName` is configured to include it; it supports `@path`
  imports to split large files (https://geminicli.com/docs/cli/gemini-md/,
  retrieved 2026-07-02).
- **Claude Code** does not read `AGENTS.md` natively; bridge it with a `CLAUDE.md`
  that `@AGENTS.md`-imports it (https://code.claude.com/docs/en/memory, retrieved
  2026-07-02).
