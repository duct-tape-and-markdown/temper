# 0084 — a built-in claims what the harness loads

- **Date:** 2026-11-24 · **Status:** accepted

## Context

0081 states that two kinds never govern one path, and the sweep found
built-ins that break it (question `two-kinds-one-path`): `memory`
(`**/CLAUDE.md`) also claims a `CLAUDE.md` under `.claude/rules/`,
`.claude/agents/`, `.claude/commands/` and `.claude/skills/`, folders whose
documents other built-ins own. The current memory docs
(code.claude.com/docs/en/memory, retrieved 2026-11-24) say: "All `.md`
files are discovered recursively, so you can organize rules into
subdirectories like `frontend/` or `backend/`." They document no
filename exclusions. They do not describe a `CLAUDE.md` inside `.claude/`
loading as memory. So the harness loads that file as a rule. The same
reading found a second fidelity gap: `rule`'s locus is top-level only,
so a rule in `.claude/rules/frontend/` loads in Claude Code and is
invisible to temper. Ruled by John 2026-11-24 ("a kind should claim a
file if that is how the runtime works").

## Decision

**A built-in's locus is the harness's** (`builtins.md`, opening). It
claims exactly the paths the harness loads as that kind, and it cites
them. Where two loaders' documented reach meets, the kind whose loader
the documentation names owns the path, and the other kind excludes it.
For Claude Code today:

- `rule` claims `.claude/rules/**/*.md`;
- `memory` excludes the folders other built-in loaders own:
  `.claude/agents/`, `.claude/commands/`, `.claude/rules/` and
  `.claude/skills/`.

An excluded directory excludes everything beneath it
(`representation.md`, "locus").

## Rejected

- **Restoring the string-only rule** (no two kinds share glob text). It
  keeps a file whose kind is ambiguous, which is the routing the
  position rule exists to prevent.
- **Most-specific pattern wins.** That is a precedence rule temper
  invents rather than one the harness documents.

## Consequences

The built-in loci and their cites move, and `exclude` admits a directory.
Every built-in locus is re-verified against the current docs rather than
only `rule`'s, since a locus is only as current as its cite.
`two-kinds-one-path` closes once the collision check compares governed
paths rather than strings, and the built-ins pass it. A rule in a
subdirectory becomes governed, which can surface new findings for an
adopter whose `.claude/rules/` has subdirectories.
