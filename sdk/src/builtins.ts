/**
 * The built-in Claude Code kinds — the face nouns a harness author imports.
 * Each is an
 * ordinary `kind<T>()` value built with the same constructor every provider uses
 * (ownership not privilege). Their five facts are external facts about the Claude
 * Code harness, cited at the point of claim.
 *
 * These are the SDK's own provider-face exports, surfaced through the
 * `@dtmd/temper/claude-code` subpath
 * — never from the root.
 */

import { kind } from "./kind.js";
import type { KindDefinition } from "./kind.js";
import type { Prose } from "./prose.js";
import {
  allowedChars,
  clause,
  deny,
  enumOf,
  forbiddenKeys,
  maxLen,
  maxLines,
  minLen,
  nameMatchesDir,
  required,
  uniqueName,
} from "./contract.js";
import type { Clause } from "./contract.js";

/** A Claude Code skill — a directory whose entry file is `SKILL.md` with YAML frontmatter. */
export interface Skill {
 /**
   * The description trigger — always in context; the body loads on invocation
   * (code.claude.com/docs/en/skills, retrieved 2026-07-15).
 */
  readonly description: string;
  /** The optional license field the skill spec carries (agentskills.io/specification). */
  readonly license?: string;
  /**
   * Set `true` to prevent Claude from automatically loading this skill — only
   * the user-invoked channel stays live (code.claude.com/docs/en/skills,
   * "Control who invokes a skill", retrieved 2026-07-15). Default: `false`.
   */
  readonly "disable-model-invocation"?: boolean;
  /**
   * Set `false` to hide the skill from the `/` menu — only the
   * description-trigger channel stays live (code.claude.com/docs/en/skills,
   * "Control who invokes a skill", retrieved 2026-07-15). Default: `true`.
   */
  readonly "user-invocable"?: boolean;
  /**
   * The optional path scope — a channel gate, not a channel of its own. A
   * present list removes the skill from *every* invocation channel — the `/`
   * listing, model invocation, and description-trigger invocation — until
   * Claude reads a file the globs match; an absent one leaves all channels
   * live. Distinct from a rule's `paths`, which registers the path-match as
   * the rule's channel: here the field gates the skill's existing channels
   * rather than being one, so it adds no `paths-match` registration entry
   * (code.claude.com/docs/en/skills, retrieved 2026-07-15; verified against
   * 2.1.210).
   */
  readonly paths?: readonly string[];
  readonly prose?: Prose;
}

/**
 * `skill` — `.claude/skills/<name>/SKILL.md`, a directory unit, YAML frontmatter
 * carrying `name` then `description`; registers on both documented invocation
 * channels — user-invoked (`/name`) and description-trigger — modulated per
 * member by the `disable-model-invocation`/`user-invocable` fields
 * (code.claude.com/docs/en/skills, agentskills.io/specification, retrieved
 * 2026-07-15).
 */
export const skill: KindDefinition<Skill> = kind<Skill>({
  name: "skill",
  locus: { kind: "at", root: ".claude/skills", glob: "*/SKILL.md" },
  format: "yaml-frontmatter",
  unitShape: "directory",
  registration: [{ via: "user-invoked" }, { via: "description-trigger", field: "description" }],
  identityField: "name",
});

/**
 * `command` — `.claude/commands/*.md`, the skill surface's legacy file placement
 * (Claude Code merged commands into skills; code.claude.com/docs/en/skills,
 * retrieved 2026-07-15): a lone file (identity from the stem, so no
 * `identityField` — like `rule`), the skill's field schema by import, registering
 * on the same two documented invocation channels as `skill`.
 */
export const command: KindDefinition<Skill> = kind<Skill>({
  name: "command",
  locus: { kind: "at", root: ".claude/commands", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "user-invoked" }, { via: "description-trigger", field: "description" }],
});

/** A Claude Code subagent definition. */
export interface Agent {
 /**
   * When Claude should delegate to this subagent — the sole registration
   * channel (code.claude.com/docs/en/sub-agents, retrieved 2026-07-15).
 */
  readonly description: string;
  readonly prose?: Prose;
}

/**
 * `agent` — every markdown file under `.claude/agents`, discovered recursively (a
 * containing subdirectory is purely organizational), YAML frontmatter carrying
 * `name` then `description`; identity is the `name` field (never the filename),
 * the named-field mode; registers on the description-trigger channel only — no
 * user-invoked slash command (code.claude.com/docs/en/sub-agents, retrieved
 * 2026-07-15).
 */
export const agent: KindDefinition<Agent> = kind<Agent>({
  name: "agent",
  locus: { kind: "at", root: ".claude/agents", glob: "**/*.md" },
  format: "yaml-frontmatter",
  unitShape: "named-field",
  registration: [{ via: "description-trigger", field: "description" }],
  identityField: "name",
});

/** A Claude Code rule — a flat markdown file with an optional `paths` scope. */
export interface Rule {
 /**
   * The path scope — a present list matching zero files is a dead edge; an absent
   * one loads unconditionally (code.claude.com/docs/en/memory, retrieved 2026-07-15).
 */
  readonly paths?: readonly string[];
  readonly prose?: Prose;
}

/**
 * `rule` — `.claude/rules/<name>.md`, a lone file (identity from the stem), YAML
 * frontmatter; registers a path scope (code.claude.com/docs/en/memory, retrieved
 * 2026-07-15).
 */
export const rule: KindDefinition<Rule> = kind<Rule>({
  name: "rule",
  locus: { kind: "at", root: ".claude/rules", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "paths-match", field: "paths" }],
});

/** A Claude Code memory file — `CLAUDE.md`, loaded in full at launch, no frontmatter. */
export interface Memory {
  readonly prose?: Prose;
}

/**
 * `memory` — a root `<name>.md` (`CLAUDE.md`, `AGENTS.md`), a lone file loaded
 * unconditionally, with **no frontmatter** (code.claude.com/docs/en/memory,
 * retrieved 2026-07-15): the whole file is the body, so the kind declares no
 * `format`. A project `CLAUDE.md` may sit at either `./CLAUDE.md` or
 * `./.claude/CLAUDE.md` — equal documented locations (same source) — and the
 * any-depth locus (a `CLAUDE.md` at any directory) covers both; a
 * module-carried memory projects the root file.
 */
export const memory: KindDefinition<Memory> = kind<Memory>({
  name: "memory",
  locus: { kind: "at", root: ".", glob: "**/CLAUDE.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
});

/**
 * A Claude Code hook — a fields-only registration member surfacing inside
 * `settings.json`, keyed under its lifecycle event. It owns no artifact of its own; a
 * handler names how it fires (`command`/`http`/`mcp_tool`/`prompt`/`agent`) plus the
 * documented common fields (code.claude.com/docs/en/hooks, retrieved 2026-07-15).
 * Authoring `hook(...)` builds a member whose typed fields fold into its manifest entry;
 * emit erases it into a registration write fact (`emit.ts`).
 */
export interface Hook {
  /** The handler kind — how the hook fires when its event matches. */
  readonly type?: "command" | "http" | "mcp_tool" | "prompt" | "agent";
  /** The shell command or executable a `command` handler runs. */
  readonly command?: string;
  /** Seconds before the handler is canceled. */
  readonly timeout?: number;
  /** The tool-name filter a tool-scoped event fires on (`"*"`/`""`/absent = all). */
  readonly matcher?: string;
}

/**
 * `hook` — a `settings.json` `hooks.<Event>` registration member: a fields-only kind (no
 * body slot), its members discovered off the `.claude/settings.json` manifest at the
 * `hooks.<Event>` collection address, keyed by lifecycle event; registers on the `event`
 * channel (code.claude.com/docs/en/hooks, retrieved 2026-07-15). The first manifest kind
 * temper ships — the read side of 0021's manifest-authoring surface.
 */
export const hook: KindDefinition<Hook> = kind<Hook>({
  name: "hook",
  locus: { kind: "at", root: ".claude", glob: "settings.json" },
  unitShape: "file",
  registration: [{ via: "event", field: "event" }],
  shape: "fields",
  collectionAddress: { manifest: "settings.json", keyPath: "hooks.<Event>" },
});

/**
 * A Claude Code MCP server — a fields-only registration member surfacing inside
 * `.mcp.json`, keyed by name under `mcpServers`. It owns no artifact of its own; its
 * `type` names the transport (`stdio` default, or `http`/`streamable-http`/`sse`/`ws`),
 * and each transport reads a different field set — `command`/`args`/`env` for a local
 * stdio process, `url`/`headers` for a remote connection
 * (code.claude.com/docs/en/mcp, retrieved 2026-07-15). Authoring `mcpServer(...)` builds a
 * member whose typed fields fold into its `mcpServers.*` entry; emit erases it into a
 * registration write fact (`emit.ts`).
 */
export interface McpServer {
  /**
   * The transport. Absent reads as `stdio`, so an entry that carries a `url` but no
   * `type` is a configuration error — Claude Code treats it as a stdio server and skips
   * it. `streamable-http` is an alias for `http`.
   */
  readonly type?: "stdio" | "http" | "streamable-http" | "sse" | "ws";
  /** The executable a stdio server runs. */
  readonly command?: string;
  /** The arguments passed to a stdio server's `command`. */
  readonly args?: readonly string[];
  /** Environment variables set in a stdio server's process. */
  readonly env?: Readonly<Record<string, string>>;
  /** The endpoint a remote (`http`/`sse`/`ws`) server connects to. */
  readonly url?: string;
  /** Static headers sent to a remote server. */
  readonly headers?: Readonly<Record<string, string>>;
  /** Milliseconds before a tool call to this server aborts. */
  readonly timeout?: number;
}

/**
 * `mcpServer` — a `.mcp.json` `mcpServers.*` registration member: a fields-only kind (no
 * body slot), its members discovered off the `.mcp.json` manifest at the `mcpServers.*`
 * collection address, keyed by server name; registers on the `connection` channel
 * (code.claude.com/docs/en/mcp, retrieved 2026-07-15). The second manifest kind temper
 * ships, and the first whose entries are objects — each server's fields fold into the
 * member the read surfaces.
 */
export const mcpServer: KindDefinition<McpServer> = kind<McpServer>({
  name: "mcp-server",
  locus: { kind: "at", root: ".", glob: ".mcp.json" },
  unitShape: "file",
  registration: [{ via: "connection" }],
  shape: "fields",
  collectionAddress: { manifest: ".mcp.json", keyPath: "mcpServers.*" },
});

/**
 * The default contract for `skill` — Anthropic's documented skill contract: the Agent
 * Skills open standard (agentskills.io), Anthropic's platform upload
 * validation, and Claude Code's own docs.
 * All sources retrieved 2026-07-15.
 *
 * Checks the strictest documented profile: the spec and upload validation are
 * hard, Claude Code's runtime is deliberately forgiving ("All fields are
 * optional") — a skill that passes here loads everywhere the format is
 * honored, not merely on the machine it was written on; where the runtime
 * diverges, the clause's own guidance says so.
 *
 * Deliberately absent — undecidable, so never gate clauses: whether the
 * description actually triggers well or reads third-person (semantic);
 * vagueness/no-op detection (semantic); gerund naming (judgment). Two
 * decidable spec rules are also absent, pending a vocabulary addition (a
 * narrow shape predicate governs additions): the name
 * must not start/end with a hyphen or contain consecutive hyphens; likewise
 * the platform's "no XML tags in the description."
 *
 * Authoring notes the clauses cannot carry: prefer gerund or noun-phrase
 * names (`processing-pdfs`, `pdf-processing`) over vague ones (`helper`,
 * `utils`); `disable-model-invocation: true` for side-effectful workflows you
 * want to time yourself; `user-invocable: false` for background knowledge
 * that is not a command; `metadata` is the sanctioned home for versioning —
 * there is no top-level `version` field.
 */
export const skillDefaultContract: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "Every skill declares a `name` — the slug the harness binds to. Claude Code alone would default it from the directory name, but a nameless skill is not portable: the spec and Anthropic's upload validation both require it.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(minLen("name", 1), {
    severity: "required",
    guidance: "A present-but-empty name fails the spec's 1-64 character bound.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(allowedChars("name", { ranges: ["a-z", "0-9"], chars: "-" }), {
    severity: "required",
    guidance:
      "Lowercase letters, digits, and hyphens only — `PDF-Processing` is the spec's own counter-example. The charset also keeps XML out of the name, which Anthropic's upload validation separately forbids.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(maxLen("name", 64), {
    severity: "required",
    guidance: "Keep the name short and slug-like; it becomes a directory and an id.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(deny("name", ["anthropic", "claude"]), {
    severity: "required",
    guidance:
      "Reserved words, enforced by Anthropic's platform upload validation (not by the open spec, and not by Claude Code's runtime — which itself ships a `claude-api` skill). Keep them out if the skill will ever travel through the API or claude.ai.",
    cite: "https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview#skill-structure (retrieved 2026-07-15)",
  }),
  clause(nameMatchesDir(), {
    severity: "required",
    guidance:
      "The spec requires the name to match its parent directory. Claude Code decouples the two (the frontmatter name is a display label; the directory names the slash command, except for a plugin-root SKILL.md) — but a mismatch is a portability trap and a reader trap even where it loads.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(required("description"), {
    severity: "required",
    guidance:
      "The description is how the model chooses this skill from potentially 100+ available — it is the skill's API. Claude Code would fall back to the body's first paragraph; the spec and upload validation require it declared.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(minLen("description", 1), {
    severity: "required",
    guidance:
      "Say both what the skill does and when to use it, with the keywords a user would naturally say. Write in third person — the text is injected into the system prompt, and inconsistent point-of-view causes discovery problems.",
    cite: "https://agentskills.io/specification#description-field (retrieved 2026-07-15)",
  }),
  clause(maxLen("description", 1024), {
    severity: "required",
    guidance:
      "The spec's cap. Claude Code additionally truncates the skill listing at 1,536 combined characters (description + when_to_use) — truncation, not rejection, but text past the fold cannot help the model choose.",
    cite: "https://agentskills.io/specification#description-field (retrieved 2026-07-15)",
  }),
  clause(maxLen("compatibility", 500), {
    severity: "required",
    guidance: "Optional field; when present the spec caps it at 500 characters. Most skills do not need it.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(maxLines(500), {
    severity: "advisory",
    guidance:
      "Progressive disclosure: keep SKILL.md under 500 lines and move detailed reference material to separate files, one level deep. Once a skill loads, its body stays in context across turns — every line is a recurring token cost. The context window is a public good.",
    cite: "https://agentskills.io/specification#progressive-disclosure (retrieved 2026-07-15)",
  }),
  clause(forbiddenKeys(["globs", "alwaysApply"]), {
    severity: "required",
    guidance:
      "Cursor `.mdc` keys. Nothing in the Agent Skills spec or Claude Code's documented frontmatter accepts them — a skill authored with them is carrying dead configuration that another tool's semantics silently fail to apply.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
];

/**
 * The default contract for `command` — `skillDefaultContract`'s clauses minus `nameMatchesDir`: a
 * command is a lone file with no parent directory to match, so the one clause
 * that ranges over the directory relationship does not apply; every other
 * documented skill-schema recommendation, name-requiredness included, still
 * governs a command by the same import (code.claude.com/docs/en/skills,
 * retrieved 2026-07-15).
 */
export const commandDefaultContract: readonly Clause[] = skillDefaultContract.filter(
  (entry) => entry.predicate.key !== "name-matches-dir",
);

/**
 * The default contract for `agent` — Anthropic's documented subagent contract
 * (code.claude.com/docs/en/sub-agents, retrieved 2026-07-15): `name` and
 * `description` are the only required fields, `name` is a "unique identifier
 * using lowercase letters and hyphens" (no digits, unlike a skill's `name`), and
 * "keep `name` values unique across the whole tree" — a same-scope collision
 * loads only one definition.
 *
 * Deliberately narrow, like `ruleDefaultContract`: undecidable properties (whether the
 * description triggers well, model/permissionMode's semi-open vocabularies) stay
 * out of the gate — the format documents little else that is decidable.
 */
export const agentDefaultContract: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "Every subagent declares a `name` — its unique identifier. Claude Code binds identity to this field alone, never the filename, so a nameless subagent cannot be delegated to.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
  clause(allowedChars("name", { ranges: ["a-z"], chars: "-" }), {
    severity: "required",
    guidance:
      "Lowercase letters and hyphens only — no digits, unlike a skill's `[a-z0-9-]` name. Hooks receive this value as `agent_type`.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
  clause(uniqueName(), {
    severity: "required",
    guidance:
      "Keep `name` values unique across the whole tree — when two files in one scope declare the same name, Claude Code loads only one of them, silently shadowing the other.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
  clause(required("description"), {
    severity: "required",
    guidance:
      "The description is how Claude decides when to delegate to this subagent — write it so the trigger is unambiguous.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
];

/**
 * The default contract for `rule` — Anthropic's documented contract for a Claude Code
 * rules file, sourced from the memory docs (`.claude/rules/` landed in
 * v2.0.64; `packages/rule.anthropic/PACKAGE.md`, the curated authoring
 * reference this migrates verbatim). All sources retrieved 2026-07-15.
 *
 * `paths` is the one documented frontmatter key for rules: glob patterns
 * (brace expansion supported) that scope the rule to matching files. Rules
 * without it load at launch with the same priority as CLAUDE.md; path-scoped
 * rules load when Claude reads a matching file. Note skills now take a
 * `paths` key too — the two schemas are separate. (Guidance only: an
 * optional field asserts nothing decidable, so it carries no clause of its
 * own: `required` is the one
 * presence predicate, and its absence is not itself a predicate.)
 * https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-15)
 *
 * What the clauses cannot carry, as guidance: keep a rule to facts Claude
 * should hold whenever the rule is in scope — concrete enough to verify ("use
 * 2-space indentation", not "format code properly"). If an entry is a
 * multi-step procedure or only matters occasionally, it belongs in a skill
 * (on-demand) rather than a rule (always-on). Prefer path-scoped rules when
 * one convention governs scattered paths; prefer per-directory CLAUDE.md when
 * directory owners maintain their own. Treat rules like code: prune them when
 * behavior drifts, and test a change by watching whether Claude's behavior
 * actually shifts.
 */
export const ruleDefaultContract: readonly Clause[] = [
  clause(forbiddenKeys(["description", "globs", "alwaysApply"]), {
    severity: "required",
    guidance:
      "Cursor `.mdc` keys. Claude Code's documented rules schema is `paths`-only; a rule authored with Cursor frontmatter is configuration another tool's semantics silently fail to honor — the rule loads, the scoping you meant does not. (That Claude Code ignores unknown keys is observed behavior, not documented contract — the documented schema is the citation.)",
    cite: "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-15)",
  }),
  clause(maxLines(200), {
    severity: "advisory",
    guidance:
      "Unconditional rules are always-on context, paid every session: the docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' (Distinct from the hard 200-line/25KB cutoff, which applies only to auto-memory MEMORY.md; rules load in full regardless of length.) For each line ask: would removing it cause Claude to make mistakes? If not, cut it.",
    cite: "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-15)",
  }),
];

/**
 * The default contract for the qualified `claude-code.memory` kind — Anthropic's
 * documented contract for a project `CLAUDE.md` (`packages/memory.anthropic/PACKAGE.md`,
 * the curated authoring reference this migrates verbatim). Retrieved 2026-07-15.
 *
 * Deliberately near-empty, because the format is: `CLAUDE.md` is plain
 * markdown with no documented frontmatter and no required fields
 * (code.claude.com/docs/en/memory, retrieved 2026-07-15), so there is no
 * schema to gate — manufacturing a required field or a forbidden-key list
 * would fake a check the format does not carry. The single clause is a context-cost
 * budget; everything else the contract could say is guidance.
 *
 * What the clauses cannot carry, as guidance: a `paths:` frontmatter block
 * belongs on a `.claude/rules/*.md` file, not on `CLAUDE.md` — the memory
 * docs document `paths` only for rules, so a rules-style header on
 * `CLAUDE.md` is dead configuration. Split a large file with `@path` imports
 * (resolved relative to the importing file, absolute allowed, recursion
 * capped at four hops; wrap a path in backticks to mention it without
 * importing). If the repo already ships an `AGENTS.md` for other agents,
 * don't duplicate it — create a `CLAUDE.md` that `@AGENTS.md`-imports it (or
 * symlink, except on Windows where the import is the recommended bridge).
 * Mind the loading asymmetry: every ancestor `CLAUDE.md` loads in full at
 * launch, while files in subdirectories load only when Claude reads a file
 * there — so a rule that must always hold belongs above the working
 * directory, not below it. Personal, un-shared notes go in `CLAUDE.local.md`
 * (gitignored), appended after `CLAUDE.md` at its level.
 */
export const memoryAnthropicDefaultContract: readonly Clause[] = [
  clause(maxLines(200), {
    severity: "advisory",
    guidance:
      "CLAUDE.md is always-on context, paid every session. The memory docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' For each line ask: would removing it cause Claude to make mistakes? If not, cut it. (Advisory: Claude Code loads the file in full regardless of length; this is a context-cost budget, not a hard cutoff.)",
    cite: "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-15)",
  }),
];

/**
 * The default contract for the qualified `agents-md.memory` kind — the AGENTS.md
 * standard's contract for a memory file, which is that there is almost none
 * (`packages/memory.agents-md/PACKAGE.md`, the curated authoring reference
 * this migrates). Guidance-only, and that is the honest encoding: `AGENTS.md`
 * "is just standard Markdown" with no required fields, no sections, and no
 * frontmatter (agents.md, retrieved 2026-07-15); the format deliberately
 * constrains nothing. A default contract that manufactured a required field, a size
 * gate, or a forbidden-key list would assert a contract the standard
 * disclaims. Even the tempting size
 * number is a *tool's* rule, not the format's: agents read the closest
 * `AGENTS.md` in the tree (nested, nearest-wins; agents.md, retrieved
 * 2026-07-15); Codex concatenates the chain root-to-cwd and stops once
 * combined size hits a byte budget, not a per-file line count
 * (`project_doc_max_bytes`, 32 KiB default;
 * learn.chatgpt.com/docs/agent-configuration/agents-md, retrieved 2026-07-15);
 * Gemini CLI reads `GEMINI.md` by default and only treats `AGENTS.md` as an
 * alias when configured via `context.fileName` (geminicli.com/docs/cli/gemini-md,
 * retrieved 2026-07-15); Claude Code does not read `AGENTS.md` natively —
 * bridge it with a `CLAUDE.md` that `@AGENTS.md`-imports it
 * (code.claude.com/docs/en/memory, retrieved 2026-07-15).
 */
export const memoryAgentsMdDefaultContract: readonly Clause[] = [];

/**
 * Every documented Claude Code hook lifecycle event — the closed set a `hooks.<Event>`
 * key is drawn from (code.claude.com/docs/en/hooks, "Hook events", retrieved 2026-07-15).
 * The allowlist the `hook` default contract's one decidable clause ranges over; the
 * update ritual when the docs add an event is to re-fetch and extend this set, never to
 * re-derive from memory.
 */
const DOCUMENTED_HOOK_EVENTS = [
  "SessionStart",
  "Setup",
  "UserPromptSubmit",
  "UserPromptExpansion",
  "PreToolUse",
  "PermissionRequest",
  "PermissionDenied",
  "PostToolUse",
  "PostToolUseFailure",
  "PostToolBatch",
  "Notification",
  "MessageDisplay",
  "SubagentStart",
  "SubagentStop",
  "TaskCreated",
  "TaskCompleted",
  "Stop",
  "StopFailure",
  "TeammateIdle",
  "InstructionsLoaded",
  "ConfigChange",
  "CwdChanged",
  "FileChanged",
  "WorktreeCreate",
  "WorktreeRemove",
  "PreCompact",
  "PostCompact",
  "Elicitation",
  "ElicitationResult",
  "SessionEnd",
] as const;

/**
 * The default contract for `hook` — Anthropic's documented hooks contract
 * (code.claude.com/docs/en/hooks, retrieved 2026-07-15). A hook surfaces at
 * `hooks.<Event>`, so the member the gate reads is the lifecycle event itself, its name
 * carried as the `event` field off the collection key. The one decidable, cited property
 * of that member is its event: a key outside the documented set is dead configuration —
 * Claude Code silently never fires a hook under an unrecognized event, so the strictest
 * documented profile is that the event is one temper's cited docs name.
 *
 * Deliberately absent — the handler's own schema (`type`/`command`/`url`/`timeout`, the
 * matcher grammar) lives one array level deeper than `hooks.<Event>`, inside each event's
 * matcher-group list, which the collection address does not walk into; a clause over it
 * would range over a field the read never surfaces, so it is no clause at all. What the
 * clauses cannot carry, as guidance: keep a handler's `type` among
 * `command`/`http`/`mcp_tool`/`prompt`/`agent`; a `command` handler needs a `command`, an
 * `http` handler a `url`; the `matcher` filters tool-scoped events and is inert on events
 * that carry no tool (`UserPromptSubmit`, `Stop`, and their siblings).
 */
export const hookDefaultContract: readonly Clause[] = [
  clause(enumOf("event", DOCUMENTED_HOOK_EVENTS), {
    severity: "required",
    guidance:
      "A hook keys under its lifecycle event; an event outside the documented set is dead configuration — Claude Code silently never fires a hook under an unrecognized event. If this is a newly-documented event, re-fetch code.claude.com/docs/en/hooks and extend temper's cited set rather than working around the finding.",
    cite: "https://code.claude.com/docs/en/hooks (retrieved 2026-07-15)",
  }),
];

/**
 * Every documented `.mcp.json` server transport — the closed set a server entry's `type`
 * is drawn from (code.claude.com/docs/en/mcp, retrieved 2026-07-15). `stdio` is the
 * default when `type` is absent; `streamable-http` is the MCP spec's own name for `http`,
 * accepted as an alias so configurations copied from server docs work unchanged; `sse` is
 * documented but deprecated; `ws` is the WebSocket transport. The update ritual when the
 * docs add a transport is to re-fetch and extend this set, never to re-derive from memory.
 */
const DOCUMENTED_MCP_TRANSPORTS = ["stdio", "http", "streamable-http", "sse", "ws"] as const;

/**
 * The default contract for `mcpServer` — Anthropic's documented `.mcp.json` contract
 * (code.claude.com/docs/en/mcp, retrieved 2026-07-15). A server surfaces at `mcpServers.*`,
 * keyed by name, its transport-specific fields folded into the member. The one decidable,
 * cited property that holds across every transport is `type`: a value outside the
 * documented set is a transport Claude Code cannot honor, so the strictest documented
 * profile is that a present `type` names one temper's cited docs carry. An absent `type`
 * passes — Claude Code reads it as `stdio`, the documented default.
 *
 * Deliberately absent — the per-transport requirements are conditional on `type`, which no
 * single-field clause can decide: a `url` with no `type` is a configuration error (Claude
 * Code reads it as a stdio server and skips it), a stdio server needs a `command`, and a
 * remote server needs a `url` — each a two-field implication the closed predicate
 * vocabulary cannot express, so it rides guidance rather than a clause that would range
 * over a field the shape of the check cannot see.
 */
export const mcpServerDefaultContract: readonly Clause[] = [
  clause(enumOf("type", DOCUMENTED_MCP_TRANSPORTS), {
    severity: "required",
    guidance:
      "A server's `type` names its transport; a value outside the documented set is one Claude Code cannot honor. Absent reads as `stdio` — but an entry that carries a `url` with no `type` is then a configuration error, because Claude Code treats it as a stdio server and skips it: add `type: \"http\"` (or `sse`/`ws`). A stdio server needs a `command`; a remote server needs a `url`. If this is a newly-documented transport, re-fetch code.claude.com/docs/en/mcp and extend temper's cited set rather than working around the finding.",
    cite: "https://code.claude.com/docs/en/mcp (retrieved 2026-07-15)",
  }),
];
