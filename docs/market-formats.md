# Market formats — the harness landscape, cited

Reference corpus for the customization-artifact surfaces of the major AI coding
harnesses. Like `horizons.md`, this lives in `docs/` deliberately: it is
**pre-intent reference material**, never contract — spec sections and curated
KIND.md files that encode any fact below re-cite it at their own point of claim
(`.claude/rules/collaboration.md`). All facts fetched from official docs,
**retrieved 2026-07-02**, by three research agents; anything a fetched page did
not confirm is marked UNVERIFIED. Re-verify before encoding — this market moves
fast (two host migrations and one vendor change were found *during* retrieval).

## The consolidated matrix

| Harness | Memory | Scoped rules | Skills (SKILL.md) | Custom agents | Hooks | Reads other tools' files |
|---|---|---|---|---|---|---|
| Claude Code | CLAUDE.md | `.claude/rules/*.md` (`paths`) | ✔ standard | `.claude/agents/` | ✔ | — |
| OpenAI Codex | AGENTS.md (native, primary) | Starlark permission rules only | ✔ standard | `.codex/agents/` (TOML) | ✔ (same event vocab as Claude Code) | AGENTS.md |
| GitHub Copilot | copilot-instructions.md + AGENTS.md + CLAUDE.md | `.github/instructions/*.instructions.md` (`applyTo`) | ✔ standard | `.github/agents/*.agent.md` | ✔ (same event vocab) | `.claude/skills`, `.claude/settings.json`, CLAUDE.md, GEMINI.md |
| Cursor | AGENTS.md (root + nested) | `.cursor/rules/*.mdc` (`globs`/`alwaysApply`/`description`) | ✔ standard | `.cursor/agents/` | ✔ (~25 events) | `.claude/` + `.codex/` skills *and* agents |
| Windsurf → Devin | global_rules.md + AGENTS.md | `.devin/rules/*.md` (`trigger` modes; `.windsurf/rules/` legacy) | ✔ standard, cites agentskills.io | ✗ (built-in modes only) | ✔ (12 events) | `.claude/skills`, `.agents/skills` |
| Cline | AGENTS.md + `.clinerules/` | `.clinerules/*` w/ `paths` frontmatter | ✔ (mechanics, standard unnamed) | `.cline/agents/` (format UNVERIFIED) | partial (SDK-documented) | `.claude/skills`, `.cursorrules`, `.windsurfrules` |
| Roo Code | AGENTS.md (+ `.local.md`) | `.roo/rules/`, mode-scoped `.roo/rules-{mode}/` | ✔ standard, cites agentskills.io | `.roomodes` custom modes (YAML) | ✗ (documented absent) | `.agents/skills` |
| Gemini CLI | GEMINI.md (hierarchical, `@path` imports) | ✗ | ✔ standard, cites agentskills.io | `.gemini/agents/*.md` | ✔ (different event vocab) | `.agents/skills` |
| JetBrains Junie | **AGENTS.md is primary** (`.junie/AGENTS.md` → root → legacy guidelines) | ✗ (`.junie/rules/` UNVERIFIED, likely nonexistent) | ✔ standard, cites agentskills.io | `.junie/agents/`, `.agents/` | partial (`SessionStart` confirmed) | `.cursor/`, `.claude/`, `.codex/` skills |
| OpenCode | AGENTS.md, falls back to CLAUDE.md | ✗ (`instructions` config key, unconditional) | ✔ standard (native `skill` tool) | `.opencode/agents/*.md` | JS/TS plugins (~30 events) | `.claude/*`, `.agents/skills` |
| Amazon Q → Kiro | `.amazonq/rules/*.md` + auto memory-bank | ✗ (rules are unconditional) | ✗ (in Q guide) | CLI surface migrated to Kiro (UNVERIFIABLE from Q docs) | migrated to Kiro | — |
| Aider | CONVENTIONS.md (explicit `--read` only) | ✗ | ✗ | ✗ | ✗ | — |

## The two standards (first-class format providers)

- **AGENTS.md** (https://agents.md, retrieved 2026-07-02): plain markdown at
  repo root, no required fields, no frontmatter; nested files, nearest wins.
  Stewarded by the **Agentic AI Foundation under the Linux Foundation**. 20+
  named consumers on the site (Codex, Jules, Gemini CLI, Aider*, Goose,
  OpenCode, Zed, Warp, Devin, Windsurf, Junie, Amp, Cursor, Roo, Copilot
  coding agent, …). *Aider is listed by agents.md but its own docs show no
  auto-load — trust the tool's docs over the standard's list.
- **Agent Skills** (https://agentskills.io/specification, retrieved
  2026-07-02): directory + `SKILL.md`, YAML frontmatter — required `name`
  (1–64, lowercase+hyphens, must match dir) and `description` (1–1024);
  optional `license`, `compatibility`, `metadata`, experimental
  `allowed-tools`. No top-level `version` field. Three-stage progressive
  disclosure (~100-token metadata → <5000-token body → resources on demand).
  Originated by **Anthropic**, published open, stewarded via the
  `agentskills/agentskills` GitHub org — **no formal governance body**
  (Linux-Foundation claims for *this* standard are unverified; do not conflate
  with AGENTS.md's stewardship above). Home page enumerates **43 adopting
  tools**, including Claude Code, Codex, Copilot, Cursor, Gemini CLI, Junie,
  OpenCode, Kiro, Roo.

## Structural findings

1. **Seven artifact-kind families cover the market**: memory/instructions,
   scoped rules, skills, custom agents/modes, commands/prompts (a dying kind —
   Codex deprecated prompts for skills; Cursor absorbed commands into skills),
   hooks, MCP config (the most format-divergent family).
2. **Format authority is mixed tool/standard.** Proprietary kinds (Cursor
   `.mdc`, Codex TOML subagents, Copilot `applyTo` instructions) belong to
   their tool; skills and AGENTS.md belong to standards consumed by dozens of
   tools. Any kind-identity axis must admit both kinds of provider.
3. **The vendor axis fails on the data.** Copilot's own surfaces diverge
   (`.vscode/mcp.json` `servers` vs. the cloud agent's web-form `mcpServers`);
   Windsurf changed vendors (Cognition), docs now under the Devin brand with
   `.devin/rules/` preferred.
4. **Cross-tool loci reads are first-class**, not compat shims: Cursor loads
   `.claude/skills/` and `.claude/agents/`; Copilot honors `CLAUDE.md` with
   its `paths` semantics and reads `.claude/settings.json` hooks; OpenCode
   falls back to `CLAUDE.md`; `.agents/skills/` is emerging as the neutral
   skills locus (Codex, Cursor, Gemini, Junie, OpenCode, Roo, Windsurf).
5. **The hook event vocabulary converged** across Claude Code, Codex, and
   Copilot (`SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`,
   `PreCompact`, `SubagentStart/Stop`, `Stop`); Gemini and Windsurf carry
   their own vocabularies.
6. **Legacy single-file rules are dead everywhere** — `.cursorrules` (dropped
   from Cursor's docs entirely), `.windsurfrules`, `.roorules`, monolithic
   `.clinerules` — all superseded by directory-of-files models. Migration
   demand for `import` is real and current.
7. **Citation targets are volatile**: `docs.cursor.com` → `cursor.com/docs`;
   Windsurf docs → `docs.devin.ai/desktop/cascade/*`; Amazon Q CLI → Kiro
   User Guide; Roo docs → `roocodeinc.github.io`. Retrieval dates are
   load-bearing.

## Import-channel priorities (product read)

1. **AGENTS.md** — universal, trivial format, one adapter covers the market's
   shared memory kind; Gemini's GEMINI.md adds `@path` imports (the
   MEMORY-KIND shape).
2. **Cursor** — largest install base among the forks; `.mdc` is a real
   distinct format (`description`/`globs`/`alwaysApply` — four activation
   modes); the dropped `.cursorrules` legacy is a standing migration story.
3. **Codex + Copilot** — skills and hooks nearly free given convergence; the
   per-provider work is the scoped-rules keys (`applyTo` vs `globs` vs
   `paths`) and the agent formats (TOML vs `.agent.md`).
4. **Low priority**: Aider (no artifact surface to govern), Amazon Q/Kiro
   (mid-rebrand, unstable docs).

## Per-harness detail

The full per-tool tables (locus, format keys, activation modes, per-claim doc
URLs, and the UNVERIFIED register) live in the three research reports this file
condenses; the load-bearing subset is the matrix above. Key UNVERIFIED items
before any encoding: Codex subagent TOML exact layout; Cursor memories +
custom modes; Cline agent/hook file formats; Windsurf workflow frontmatter;
Junie hook event list; Gemini SKILL.md full schema; everything Amazon-Q-CLI
(migrated to Kiro).
