# 0014 — command and agent join the shipped kinds

- **Date:** 2026-07-07 · **Status:** accepted

## Context

`builtins.md` ruled commands and agents forward work (0011 registered the
fork), reserving field schema, contract stance, and registration for a
ruling. A doc fetch (2026-07-07) found the premise moved: Claude Code
merged custom commands into skills — "a file at `.claude/commands/deploy.md`
and a skill at `.claude/skills/deploy/SKILL.md` both create `/deploy` and
work the same way," legacy files "support the same frontmatter"
(code.claude.com/docs/en/skills, retrieved 2026-07-07; the standalone
slash-commands page no longer exists).

## Decision

**command** ships as a second placement of the skill surface, never a
second schema: the skill's field schema by import, a lone markdown file
under `.claude/commands/`, identity from the file stem.

**agent** ships as a new kind: markdown under `.claude/agents/`, scanned
recursively with subdirectories organizational; identity from the
frontmatter `name` field, never the filename — the third identity mode
(directory name, file stem, named field); `name` and `description` are the
only required fields, `name` charset "lowercase letters and hyphens"
(code.claude.com/docs/en/sub-agents, retrieved 2026-07-07).

**Registration recuts from a scalar to a set of documented channels.**
`user-invoked` joins the vocabulary (0011) as a channel beside
`description-trigger`: skills and commands sit on both, modulated per
member by the documented `disable-model-invocation` / `user-invocable`
fields — ordinary declared fields, not registration facts.

Gaps stay surfaced, never filled: command subdirectory namespacing and a
personal `~/.claude/commands/` location are undocumented today, and
skill/command names carry no documented charset — so no charset clause;
the agent kind's is the documented asymmetry.

## Rejected

One skill kind with two globs (muddies unit shape and identity
derivation). Scalar registration naming command "user-invoked only" (false
to the two-channel docs). Modeling plugin-context agent variants now
(plugins ignore `hooks`/`mcpServers`/`permissionMode`; recorded as cites,
modeled when plugin manifests land).

## Consequences

`builtins.md` counts five shipped kinds; the provider module gains both
kinds and their default contracts (clauses in code, cited and dated —
agent: required fields, charset, closed vocabularies, per-scope name
uniqueness; skill/command: the current docs' recommendations); the
built-in lock re-derives; the engine gains named-field identity; the skill
kind's stale profile re-verifies against the same fetch.
