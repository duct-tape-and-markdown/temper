# 0115 — cost coverage, rejected values, matcher misses, install evidence

- **Date:** 2026-11-27 · **Status:** accepted

## Context

The loop raised four plan questions. Three were class-covered with the spec text missing. The fourth turned on an external fact that the docs leave silent, so it was probed under the collaboration rule's probe-before-encoding bar. Triaged under John's direction.

## Decision

- **Context cost reaches what the runtime cuts and lists** (`builtins.md`, default contracts; 0101, 0112).
  - The listing budget is one root-bound clause over every model-invocable skill and command, since one budget spans both kinds.
  - The size advisory covers every file the runtime cuts at the per-file limit, subdirectory memory, imports and rules among them.
  - `rule` claims `**/.claude/rules/**`, matching nested skills and memory (0090).
  - A load moment is a registration fact. On-demand subdirectory memory and a skill's paths-gated listing are channels, so `explain` and the advisories read one fact.
- **A value the runtime rejects draws a finding** (`builtins.md`, intro; 0101, 0103).
  - Probed with Claude Code 2.1.332: a settings key of the wrong JSON type is skipped alone and the rest of the file applies, and `.mcp.json` with a non-object `mcpServers` is rejected.
  - The finding names the key and the type it carries, and a regenerate keeps the value. The posture lands in `engineering.md`: "A reader refuses or reports what it consumes".
- **A runtime-matched string is judged only where a miss is decidable without intent** (`builtins.md`). This comes from the hand-built drift field report.
  - No default clause guesses an author's intended target.
  - `explain` states how each permission path pattern resolves, and names an agent that shares a built-in agent's name.
  - An allow rule's match against a command a member runs correlates two members. It joins `cross-source-edge-correlation` as that question's second driver.
- **A seated hook is an unrepresented install's evidence** (`distribution.md`; 0103, no invisible state).
  - Where a temper `SessionStart` hook is seated on a tree with no program, `check` judges it and its modelines.
  - Where none is seated, `check` claims nothing.

## Rejected

- **Judging an unrepresented gate on every program-less tree.** That warns a stranger about a gate nobody installed.
- **Near-miss agent names as a finding.** Similarity to a built-in name is taste.
- **Per-kind listing budgets.** They miss the joint overrun.

## Consequences

Four questions are retired. The rule kind's nested placement follows memory's and skill's grammar.

## Amendment (2026-11-27)

The settings half of the rejected-value probe is documented, so it now cites
the settings page ("Fix a broken settings file"). Only the `.mcp.json`
collection case rests on the probe. A search of the public tracker before
filing found the compaction fact reported more precisely in
anthropics/claude-code#91689: only recently read files return verbatim, and
older ones return as a reference. `builtins.md` now says so (0111's third
fact, narrowed).
